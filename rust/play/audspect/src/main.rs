use std::{convert::TryInto, f32::consts::PI, str::FromStr, time::Instant};

use anyhow::Result;
use crossterm::style::Colorize;
use decorum::Total;
use microfft::real::*;
use num_traits::Zero;
use plotters::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    analyze: bool,
    #[structopt(short, long)]
    backend: Option<Backend>,
    #[structopt(short, long)]
    compare: bool,
}

enum Backend {
    MicroFft,
    RustFft,
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum SignalComponent {
    Sin(f32, f32),
    Dc(f32),
}

enum Window {
    Unit,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::from_args();
    let signal: Vec<f32> = Signal::new(
        44100.,
        vec![
            SignalComponent::Dc(1.0),
            //SignalComponent::Dc(0.),
            SignalComponent::Sin(1., 440.),
            SignalComponent::Sin(1., 11025.),
            SignalComponent::Sin(1., 342.),
        ],
    )
    .take(4096) // TODO: fix
    .collect();

    if args.compare {
        compare_backends(&signal)?;
    }

    if args.analyze {
        analyze(&signal, None, args.backend)?;
    }

    Ok(())
}

impl FromStr for Backend {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mfft" => Ok(Self::MicroFft),
            "rfft" => Ok(Self::RustFft),
            _ => Err("Bad backend from_str"),
        }
    }
}

fn analyze(signal: &[f32], w: Option<Window>, b: Option<Backend>) -> anyhow::Result<()> {
    println!("?????????????????????????????????????????");
    let _window = w.unwrap_or(Window::Unit);
    let backend = b.unwrap_or(Backend::RustFft);
    let spectrum = match backend {
        Backend::MicroFft => microfft_fft_4096(signal)?,
        Backend::RustFft => rustfft_fft(signal),
    };
    let nyquist = 44100. / 2.; // TODO: pass Signal instead of &f[32] to get sample_frequency
    let freq_interval = nyquist / spectrum.len() as f32;
    let log_spectrum: Vec<Total<f32>> = spectrum
        .iter()
        .skip(1)
        .map(|x| 20. * x.log10())
        .map(|x| x.into())
        .collect();

    // Make plots
    //let size = (2560, 1600);
    let size = (5120, 3200);
    let root = SVGBackend::new("results.svg", size).into_drawing_area();
    let areas = root.split_evenly((2, 1));
    root.fill(&WHITE)?;

    // ---- Power spectrum chart
    // Try to construct range from totally ordered floats.
    // I don't think infinite or nan values are possible in spectrum,
    // so unwrap shouldn't panic..?
    let min = f32::from(*log_spectrum.iter().min().unwrap());
    let max = f32::from(*log_spectrum.iter().max().unwrap());
    let y_range = min..max;
    let mut spectrum_chart = ChartBuilder::on(&areas[1])
        .caption("Power Spectrum", ("sans-serif", 42))
        .x_label_area_size(60)
        .y_label_area_size(90)
        .build_cartesian_2d(freq_interval..nyquist, y_range)?;
    spectrum_chart
        .configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Power (Db)")
        .label_style(("sans-serif", 30))
        .draw()?;
    let points = log_spectrum
        .iter()
        .enumerate()
        .map(|(i, x)| ((i + 1) as f32 * freq_interval, f32::from(*x)));
    spectrum_chart.draw_series(LineSeries::new(points, &RED))?;

    let float_log_spectrum: Vec<f32> = log_spectrum.iter().map(|x| x.into_inner()).collect();
    let peaks = peaks(&float_log_spectrum[..], 44100.);
    println!("------>>>>>>>{:?}", peaks);
    spectrum_chart.draw_series(
        peaks
            .iter()
            //.map(|x| Circle::new((*x as f32, 0f32), 3, BLUE.filled())),
            .map(|(x, y)| {
                let pt = ((*x + 1.) as f32 * freq_interval, f32::from(*y));
                println!("{:?}", pt);
                Circle::new(
                    pt,
                    3,
                    BLUE.filled(),
                )
            }),
    )?;

    /*
    spectrum_chart.draw_series(PointSeries::of_element(
        peaks.iter().map(|x| (*x, float_log_spectrum[*x])),
        peaks.len() as i32,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size as i32, style)
                + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))?;
    */

    // ---- Signal amplitude chart
    let min = f32::from(signal.iter().map(|x| Total::from(*x)).min().unwrap());
    let max = f32::from(signal.iter().map(|x| Total::from(*x)).max().unwrap());
    // Center amplitude on zero
    let absmax = match min.abs() < max.abs() {
        true => max,
        false => min,
    };
    let y_range = (-absmax)..absmax;
    let mut amplitude_chart = ChartBuilder::on(&areas[0])
        .caption("Signal Amplitude", ("sans-serif", 42))
        .x_label_area_size(60)
        .y_label_area_size(90)
        .build_cartesian_2d(0f32..(signal.len() as f32 / 44100.), y_range)?;
    amplitude_chart
        .configure_mesh()
        .x_desc("Time (Sec)")
        .y_desc("Amplitude")
        .label_style(("sans-serif", 30))
        .draw()?;
    let points = signal
        .iter()
        .enumerate()
        .map(|(i, x)| (i as f32 / 44100., *x));
    amplitude_chart.draw_series(LineSeries::new(points, &RED))?;

    Ok(())
}

struct Signal {
    components: Vec<SignalComponent>,
    linspace: Box<dyn Iterator<Item = f32>>,
    sample_frequency: f32,
}

impl Signal {
    fn new(sample_frequency: f32, components: Vec<SignalComponent>) -> Self {
        Self {
            components,
            linspace: Box::new((0..usize::MAX).map(move |x| x as f32 / sample_frequency)),
            sample_frequency,
        }
    }
}

impl Iterator for Signal {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let step = self.linspace.next()?;
        let mut result = 0f32;
        for component in &self.components[..] {
            match component {
                SignalComponent::Sin(scale, freq) => {
                    result += scale * (2. * PI * step * freq).sin();
                }
                SignalComponent::Dc(level) => {
                    result += level;
                }
            }
        }

        // Scale down by number of sinusoids to keep volume normal
        let mut scale_factor = 0f32;
        for c in &self.components {
            match c {
                SignalComponent::Dc(_) => {}
                _ => scale_factor += 1.,
            }
        }

        if scale_factor == 0f32 {
            Some(result)
        } else {
            Some(result / scale_factor)
        }
    }
}

#[allow(dead_code)]
fn microfft_fft_64(data: &[f32]) -> anyhow::Result<Vec<f32>> {
    // Create buffer of real valued samples
    let mut samples: [f32; 64] = data.try_into()?;
    // Calculate scale factor before borrowning samples
    let scale_factor = 1. / (samples.len() as f32).sqrt();
    // Get the fft
    let spectrum = rfft_64(&mut samples);
    spectrum[0].re /= 2.;
    // microfft uses imginary part of dc bin to store nyquist freq coefficient
    // Need to clear it out before calculations
    let nyquist_coef = std::mem::take(&mut spectrum[0].im);
    assert_eq!(spectrum[0].im, 0.);
    println!("{}:{}", String::from("NYQ").yellow(), nyquist_coef);

    // Scaled magnitude
    Ok(spectrum
        .iter()
        .map(|x| x.norm_sqr().sqrt())
        .map(|x| x * scale_factor)
        .collect())
}

fn microfft_fft_4096(data: &[f32]) -> anyhow::Result<Vec<f32>> {
    // Create buffer of real valued samples
    let mut samples: [f32; 4096] = data.try_into()?;
    // Calculate scale factor before borrowning samples
    let scale_factor = 1. / (samples.len() as f32).sqrt();
    // Get the fft
    let spectrum = rfft_4096(&mut samples);
    spectrum[0].re /= 2.;
    // microfft uses imginary part of dc bin to store nyquist freq coefficient
    // Need to clear it out before calculations
    let nyquist_coef = std::mem::take(&mut spectrum[0].im);
    assert_eq!(spectrum[0].im, 0.);
    println!("{}:{}", String::from("NYQ").yellow(), nyquist_coef);

    // Scaled magnitude
    Ok(spectrum
        .iter()
        .map(|x| x.norm_sqr().sqrt())
        .map(|x| x * scale_factor)
        .collect())
}

fn rustfft_fft(data: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(data.len());
    let mut samples = vec![Complex::zero(); data.len()];

    // Fill buffer with real values
    for (i, val) in data
        .iter()
        .chain([0.].iter().cycle())
        .take(data.len())
        .enumerate()
    {
        samples[i].re = *val;
    }

    fft.process(&mut samples);

    samples[0] /= 2.;
    samples
        .iter()
        .map(|x| (x.re.powi(2) + x.im.powi(2)).sqrt())
        .map(|x| x / (samples.len() as f32).sqrt())
        .take(data.len() / 2)
        .collect()
}

fn compare_backends(signal: &[f32]) -> anyhow::Result<()> {
    //------~=~ microfft
    let now = Instant::now();
    let spectrum_mfft = microfft_fft_4096(signal)?;
    println!(
        "{}[{}]->{:?}",
        String::from("MICROFFT").green(),
        now.elapsed().as_micros(),
        spectrum_mfft
    );
    println!(
        "{}[{}]->dc{} = {}?",
        String::from("SUM").magenta(),
        spectrum_mfft.len(),
        spectrum_mfft[0],
        spectrum_mfft.iter().skip(1).sum::<f32>(),
    );

    //------~=~ rustfft
    let now = Instant::now();
    let spectrum_rfft = rustfft_fft(signal);
    println!(
        "{}[{}]->{:?}",
        String::from("RUSTFFT").green(),
        now.elapsed().as_micros(),
        spectrum_rfft
    );
    println!(
        "{}[{}]->dc{} = {}?",
        String::from("SUM").magenta(),
        spectrum_rfft.len(),
        spectrum_rfft[0],
        spectrum_rfft.iter().skip(1).sum::<f32>()
    );

    // Plot something!
    let root_drawing_area = SVGBackend::new("results.svg", (2560, 1600)).into_drawing_area();
    root_drawing_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Power Spectrum", ("sans-serif", 42))
        .x_label_area_size(60)
        .y_label_area_size(90)
        .build_cartesian_2d(0f32..2048., 0f32..3.)?;

    chart
        .configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Power (??)")
        .label_style(("sans-serif", 30))
        .draw()?;

    let points_mfft = spectrum_mfft
        .iter()
        .enumerate()
        .map(|(i, val)| (i as f32, *val));

    let points_rfft = spectrum_rfft
        .iter()
        .enumerate()
        .map(|(i, val)| (i as f32, *val));

    chart.draw_series(LineSeries::new(points_mfft, &RED))?;
    chart.draw_series(LineSeries::new(points_rfft, &GREEN))?;

    Ok(())
}

fn peaks(spectrum: &[f32], sample_rate: f32) -> Vec<(f32, f32)> {
    let threshold = 20.;
    let freq_interval = sample_rate / (2. * spectrum.len() as f32);
    spectrum
        .iter()
        .map(|x| 20. * x.log10())
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, x)| {
            // No peaks at boundaries
            if i > 0 && i < spectrum.len() - 1 {
                if x > spectrum[i - 1] && x > spectrum[i + 1] && x > threshold {
                    // println!("{} {} {}", &spectrum[i - 1], x, &spectrum[i + 1]);
                    // println!("{}: {}", i as f32 * sample_rate, x);
                    println!("{}: {} ({})", i as f32 * freq_interval, x, freq_interval);
                    //acc.push((i as f32 * freq_interval, x));
                    //acc.push(i);
                    acc.push((i as f32, spectrum[i]));
                }
            }

            acc
        })
}
