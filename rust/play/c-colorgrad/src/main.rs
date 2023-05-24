use std::{convert::TryInto, f64::consts::PI, io::Write};

use anyhow::{anyhow, Result};
use colorgrad::Gradient;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
//use ndarray::{Array, Array1};
use ndarray::prelude::*;

fn main() -> Result<()> {
    ctrlc::set_handler(move || {
        let mut stdout = std::io::stdout();
        stdout.queue(terminal::LeaveAlternateScreen).unwrap();
        stdout.queue(cursor::Show).unwrap();
        stdout.flush().unwrap();
        std::process::exit(0);
    })
    .unwrap();

    if let Some(opt) = std::env::args().nth(1) {
        match opt.as_ref() {
            "a" => animate(),
            "a2" => tokio::runtime::Runtime::new().unwrap().block_on(animate2()),
            "b" => bgbrb(),
            "t" => towers(),
            _ => bgbrb(),
        }
    } else {
        Err(anyhow!("Please provide a function to run: [bt]"))
    }
}

struct GridGradient {
    size: (u32, u32),
    grads: Vec<Gradient>,
}

impl GridGradient {
    fn new(size: (u32, u32), bottom: Gradient, top: Gradient) -> Self {
        let mut grads = Vec::new();
        for i in 0..size.0 {
            let g = colorgrad::CustomGradient::new()
                .colors(&[
                    colorgrad::Color::from(top.at(i as f64 / size.0 as f64)),
                    colorgrad::Color::from(bottom.at(i as f64 / size.0 as f64)),
                ])
                .build()
                .unwrap();
            grads.push(g);
        }
        Self { size, grads }
    }

    fn at(&self, x: u32, y: u32) -> colorgrad::Color {
        self.grads[x as usize].at(y as f64 / self.size.1 as f64)
    }
}

struct GradientStack(Vec<Gradient>);

impl GradientStack {
    fn at(&self, x: f64, y: f64) -> colorgrad::Color {
        let gradient_slice: Vec<colorgrad::Color> = self.0.iter().map(|g| g.at(x)).collect();
        let grad = colorgrad::CustomGradient::new()
            .colors(&gradient_slice)
            .build()
            .unwrap();

        grad.at(y)
    }

    fn meter_at(&self, levels: Vec<usize>) -> Vec<Vec<colorgrad::Color>> {
        levels
            .iter()
            .enumerate()
            .map(|(i, level)| {
                let x = i as f64 / levels.len() as f64;
                let gradient_slice: Vec<colorgrad::Color> =
                    self.0.iter().map(|g| g.at(x)).collect();
                let grad = colorgrad::CustomGradient::new()
                    .colors(&gradient_slice)
                    .build()
                    .unwrap();

                grad.colors(*level)
            })
            .collect()
    }
}

trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for f32 {
    fn abs(&self) -> Self {
        (*self as f32).abs()
    }
}

impl Abs for i16 {
    fn abs(&self) -> Self {
        (*self as i16).abs()
    }
}

impl Abs for u16 {
    fn abs(&self) -> Self {
        *self
    }
}

async fn animate2() -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let config = device.default_input_config().unwrap();
    let err_fn = |_| {};

    let (tx, rx) = tokio::sync::watch::channel(vec![0f32; 64]);

    fn mean<T>(data: &[T]) -> Option<T>
    where
        T: cpal::Sample
            // + std::fmt::Display
            + num_traits::cast::FromPrimitive
            + num_traits::Zero
            + Abs
            + std::ops::Div<Output = T>,
    {
        let a = Array1::from_vec(data.to_vec());

        a.map(|x| x.abs()).mean()
    }

    let _width = 47;
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| {
                    let width = 2 * size().0 as usize;

                    // Prepare plan for fft
                    use num_traits::Zero;
                    use rustfft::{num_complex::Complex, FftPlanner};
                    let mut planner = FftPlanner::<f32>::new();
                    let fft = planner.plan_fft_forward(width);
                    let mut samples = vec![Complex::zero(); width];

                    // Fill buffer with real values
                    for (i, val) in data
                        .iter()
                        .chain([0.].iter().cycle())
                        .step_by(2) // use just one channel of audio
                        .take(width)
                        .enumerate()
                    {
                        samples[i].re = *val;
                    }

                    fft.process(&mut samples);
                    //samples[0] = samples[0] / 2.;

                    // Take normalized magnitudes
                    let mut meter: Vec<f32> = samples
                        .iter()
                        .skip(1)
                        //.skip((width / 2) + 1)
                        //.skip(width / 2)
                        .take(width / 2)
                        //.map(|x| x * 2.)
                        .map(|x| x.norm_sqr().sqrt())
                        //.map(|x| x.norm())
                        //.map(|x| 20. * x.log10())
                        //.map(|x| x / samples.len() as f32)
                        //.map(|x| 20. * x.log(10.))
                        //.map(|x| (x.re.powi(2) + x.im.powi(2)).sqrt())
                        .map(|x| x / (samples.len() as f32).sqrt())
                        //.map(|x| x / samples.len() as f32)
                        //.map(|x| x.norm())
                        .collect();

                    meter = meter
                        .iter()
                        .map(|x| 20. * x.log10())
                        .map(|x| {
                            if x < -30. {
                                0.
                            } else if x > 0. {
                                1.
                            } else {
                                (x + 30.) / 30.
                            }
                        })
                        .collect();

                    // meter = meter.iter().map(|x| 20. * x.log10()).collect();

                    // meter = meter.iter().map(|x| x.abs() / 100.).collect();

                    /*
                    print!(
                        "--->>>>>>>>>>>{:?}<<<<<<<<<<<------",
                        meter.iter().sum::<f32>()
                    );
                    */
                    //print!("{:?}", meter);

                    /*
                    for (i, v) in meter.iter().enumerate() {
                        if *v > 1. {
                            println!("--->>{}<<-->{}<::>{:?}<--", width, *v, i);
                        }
                    }
                    */

                    tx.send(meter).unwrap();

                    // let m: f64 = mean::<f32>(data).unwrap_or(0f32).into();
                    // tx.send(m).unwrap();
                },
                err_fn,
            )
            .unwrap(),
        cpal::SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| {
                    println!("i16{}", data.len());
                    let _m: f64 = mean::<i16>(data).unwrap_or(0i16).into();
                    //tx.send(m / 65536.).unwrap();
                },
                err_fn,
            )
            .unwrap(),
        cpal::SampleFormat::U16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| {
                    println!("u16{}", data.len());
                    let _m: f64 = mean::<u16>(data).unwrap_or(0u16).into();
                    //tx.send(m / 32768.).unwrap();
                },
                err_fn,
            )
            .unwrap(),
    };
    stream.play().unwrap();

    let grads = GradientStack(vec![colorgrad::rainbow(), colorgrad::cividis()]);
    let mut stdout = std::io::stdout();
    let s = size();

    stdout.queue(cursor::Hide).unwrap();

    stdout.queue(cursor::MoveTo(0, s.1 as u16 - 34)).unwrap();
    stdout
        .queue(SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 }))
        .unwrap();
    stdout.queue(Clear(ClearType::FromCursorDown)).unwrap();
    stdout.flush().unwrap();

    //let mut xx = 0.;
    //let height = 16;
    let height = size().1 as usize;
    loop {
        let levels = &*rx.borrow();
        //println!("{:?}", rx.borrow());

        // let levels = Array::linspace(0., 2f64 * PI, size().0 as usize);
        // let levels = levels.map(|x| (((x + (xx as f64)).sin().abs()) * height as f64) as usize);

        let levels: Vec<usize> = levels
            .iter()
            .map(|x| (x * height as f32) as usize)
            .collect();
        //println!("{:?}", levels);
        //let level = levels.iter().map(|x| (x * height as f32) as usize).collect();
        //let mut levels = Vec::new();
        //for _ in 0..s.1 {
        //    levels.push(level);
        //}
        let bars = grads.meter_at(levels.to_vec());
        //xx += 0.2;
        for (i, bar) in bars.iter().enumerate() {
            for (j, color) in bar
                .iter()
                .chain([colorgrad::Color::from_rgb_u8(0, 0, 0)].iter().cycle())
                .take(height)
                .enumerate()
            {
                stdout
                    .queue(cursor::MoveTo(i as u16, s.1 as u16 - 1u16 - j as u16))
                    .unwrap();
                let c = color.rgba_u8();
                stdout
                    .queue(SetBackgroundColor(Color::Rgb {
                        r: c.0,
                        g: c.1,
                        b: c.2,
                    }))
                    .unwrap();
                stdout.queue(Print(" ")).unwrap();
            }
        }
        stdout.flush().unwrap();
        //std::thread::sleep(std::time::Duration::from_millis(84));
        //std::thread::sleep(std::time::Duration::from_millis(42));
    }
}

fn animate() -> Result<()> {
    let mut stdout = std::io::stdout();
    let gg = GridGradient2::from(vec![colorgrad::rainbow(), colorgrad::magma()]);

    let (tx, rx) = std::sync::mpsc::sync_channel::<()>(0);
    //ctrlc::set_handler(move || {
    //    tx.send(()).unwrap();
    //})
    //.unwrap();

    stdout.queue(cursor::Hide).unwrap();
    stdout.queue(terminal::EnterAlternateScreen).unwrap();
    stdout.flush().unwrap();

    let mut i: f64 = 0.;
    loop {
        if let Ok(_) = rx.try_recv() {
            break;
        }
        let v = i.sin();
        i += 0.1;

        //
        let g_top = colorgrad::CustomGradient::new()
            .colors(&[colorgrad::Color::from_rgb(1. * v, 1. * v, 1. * v)])
            .build()
            .unwrap();
        let gg = GridGradient2::from(vec![g_top, colorgrad::rainbow(), colorgrad::magma()]);
        //

        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        for y in 0..size().1 - 1 {
            for x in 0..size().0 {
                let c = gg.at(x, y).rgba_u8();
                stdout
                    .queue(SetBackgroundColor(Color::Rgb {
                        r: c.0,
                        g: c.1,
                        b: c.2,
                    }))
                    .unwrap();
                print!(" ");
            }
            println!();
        }
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    stdout.queue(terminal::LeaveAlternateScreen).unwrap();
    stdout.queue(cursor::Show).unwrap();
    stdout.flush().unwrap();

    Ok(())
}

struct GridGradient2 {
    size: (u32, u32),
    rows: Vec<Gradient>,
}

impl GridGradient2 {
    fn from(rows: Vec<Gradient>) -> Self {
        Self { size: size(), rows }
    }

    fn at(&self, x: u32, y: u32) -> colorgrad::Color {
        let norm = (x as f64 / self.size.0 as f64, y as f64 / self.size.1 as f64);
        let mut color_slice = Vec::new();
        for c in self.rows.iter() {
            color_slice.push(c.at(norm.0));
        }
        let grad = colorgrad::CustomGradient::new()
            .colors(&color_slice)
            .build()
            .unwrap();

        grad.at(norm.1)
    }
}

fn towers() -> Result<()> {
    let g_bottom = colorgrad::rainbow();

    let _g_top = colorgrad::blues();

    let g_top = colorgrad::CustomGradient::new()
        .colors(&[
            colorgrad::Color::from_rgb(0.8, 0.0, 0.1),
            colorgrad::Color::from_rgb(0.0, 0.1, 0.0),
        ])
        .build()
        .unwrap();

    let gg = GridGradient::new(size(), g_bottom, g_top);

    let mut stdout = std::io::stdout();
    for y in 0..size().1 - 1 {
        for x in 0..size().0 {
            let c = gg.at(x, y).rgba_u8();
            stdout
                .queue(SetBackgroundColor(Color::Rgb {
                    r: c.0,
                    g: c.1,
                    b: c.2,
                }))
                .unwrap();
            print!(" ");
        }
        println!();
    }
    stdout.flush().unwrap();

    Ok(())
}

fn bgbrb() -> Result<()> {
    let g = colorgrad::CustomGradient::new()
        .colors(&[
            colorgrad::Color::from_rgb(0., 0., 0.),
            colorgrad::Color::from_rgb(0., 1., 0.),
            colorgrad::Color::from_rgb(0., 0., 1.),
            colorgrad::Color::from_rgb(1., 0., 0.),
            colorgrad::Color::from_rgb(0., 0., 0.),
        ])
        .build()
        .unwrap();
    println!("{:?}", g);
    let mut stdout = std::io::stdout();
    let steps = size().0 * (size().1 - 1);
    for i in (0..steps).map(|i| i as f64 / steps as f64) {
        let g = g.at(i).rgba_u8();
        stdout
            .queue(SetBackgroundColor(Color::Rgb {
                r: g.0,
                g: g.1,
                b: g.2,
            }))
            .unwrap();
        print!(" ");
        stdout.flush().unwrap();
    }
    stdout.queue(ResetColor).unwrap();
    stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
    println!();
    stdout.flush().unwrap();

    Ok(())
}

fn size() -> (u32, u32) {
    if let Ok((x, y)) = terminal::size() {
        (x as u32, y as u32)
    } else {
        (0, 0)
    }
}
