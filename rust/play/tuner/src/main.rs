use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

use log::error;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//use crossbeam_channel::unbounded;
use microfft::real::rfft_256;
use microfft::real::rfft_4096;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 256;

#[derive(Clone, Default)]
struct Sample {
    rms: f32,
    rms_hist: Vec<f32>,
    spectrum: Vec<f32>,
    notes: Vec<usize>,
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let sample = Arc::new(Mutex::new(Sample::default()));
    let sample_in = sample.clone();

    // Audio stream
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let config = device.default_input_config().unwrap();
    let err_fn = |_| {};
    let sample_rate = config.sample_rate().0 as f32;
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _timing_info: &_| {
                    //println!("{},", data.len());
                    let mut samples: [f32; 256] = data
                        .iter()
                        //.rev()
                        .chain([0f32; 1].iter().cycle())
                        .step_by(2) // two channel interlaced?
                        //.take(4096)
                        .take(256)
                        .copied()
                        .collect::<Vec<f32>>()
                        .try_into()
                        .unwrap();
                    //let spectrum = rfft_4096(&mut samples);
                    let spectrum = rfft_256(&mut samples);
                    let spectrum = spectrum
                        .iter()
                        .skip(1)
                        .map(|x| x.norm_sqr().sqrt())
                        .collect::<Vec<f32>>();
                    let notes = find_note(&spectrum, sample_rate);
                    let sum: f32 = data.iter().map(|x| x.abs()).sum();
                    let avg = sum / data.len() as f32;
                    {
                        let mut s_in = sample_in.lock().unwrap();
                        s_in.rms = avg;
                        s_in.rms_hist.push(avg);
                        if s_in.rms_hist.len() > SCREEN_WIDTH as usize {
                            s_in.rms_hist.resize(SCREEN_WIDTH as usize, 0.);
                        }
                        s_in.spectrum = spectrum;
                        s_in.notes = notes;

                        /*
                        *sample_in.lock().unwrap() = Sample {
                            rms: avg,
                            spectrum,
                            notes,
                        };
                        */
                    }
                },
                err_fn,
            )
            .unwrap(),
        _ => {
            todo!()
        }
    };
    stream.play().unwrap();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Eat Shit Asshole!", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    //let mut pixels = Pixels::new(2047, 400, surface_texture)?;
    let mut pixels = Pixels::new(255, 255, surface_texture)?;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // Grab the frame buffer
            let screen = pixels.get_frame();
            //let spec = spec.lock().unwrap().clone();
            let sample = sample.lock().unwrap().clone();

            //if sample.spectrum.len() >= 128 {
            for (i, pixel) in screen.chunks_exact_mut(4).enumerate() {
                //let show = (sample.spectrum[i % 127] * 10.) as u8;
                //pixel.copy_from_slice(&[0, 0, show, 0]);

                //let gr = (sample.rms * 255.) as u8;
                //pixel.copy_from_slice(&[0, gr, 0, 0]);

                let col = i as u32 % SCREEN_WIDTH;
                let row = i as u32 / SCREEN_WIDTH;
                println!("--->>{}<<>>{}<<", sample.rms_hist.len(), col);
                let show = if sample.rms_hist.len() > col as usize
                    && sample.rms_hist[col as usize] * 255. > row as f32
                {
                    1
                } else {
                    0
                };
                println!("--->>{}<<", show);
                pixel.copy_from_slice(&[0, show, 0, 0]);

                //pixel.copy_from_slice(&[0, 255, 0, 0]);
            }
            //}

            // Show it
            if let Err(e) = pixels.render() {
                error!("pixels.render() failed: {}", e);
                *control_flow = ControlFlow::Exit;
            }
        }

        if input.update(&event) {
            // Exit
            if input.key_pressed(VirtualKeyCode::Q)
                || input.key_pressed(VirtualKeyCode::Escape)
                || input.quit()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

fn find_note(spectrum: &[f32], sample_rate: f32) -> Vec<usize> {
    let _threshold = 30.; // 30.; //100_000.;
    let freq_interval = sample_rate / (spectrum.len() as f32 * 2.);
    spectrum
        .iter()
        .map(|x| 20. * x.log10())
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, x)| {
            // No peaks at boundaries
            if i > 0
                && i < spectrum.len() - 1
                && x > spectrum[i - 1]
                && x > spectrum[i + 1]
                && x > _threshold
            {
                // println!("{} {} {}", &spectrum[i - 1], x, &spectrum[i + 1]);
                // println!("{}: {}", i as f32 * sample_rate, x);
                //println!("---------------------------------");
                //println!("{}: {} ({})", i as f32 * freq_interval, x, freq_interval);
                acc.push(i);
            }

            acc
        })
}

#[allow(dead_code)]
enum Note<'a> {
    C,
    G,
    D,
    A,
    Above(&'a Note<'a>),
    Below(&'a Note<'a>),
    Unknown,
}

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}
