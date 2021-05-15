use std::convert::TryInto;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use microfft::real::rfft_4096;

fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let config = device.default_input_config().unwrap();
    let err_fn = |_| {};
    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate().0 as f32;
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _timing_info: &_| {
                    let mut samples: [f32; 4096] = data
                        .iter()
                        .chain([0f32; 1].iter().cycle())
                        .take(4096)
                        .map(|x| *x)
                        .collect::<Vec<f32>>()
                        .try_into()
                        .unwrap();
                    let spectrum = rfft_4096(&mut samples);
                    let spectrum = spectrum
                        .iter()
                        .skip(1)
                        .map(|x| x.norm_sqr().sqrt())
                        .collect::<Vec<f32>>();
                    find_note(&spectrum, sample_rate);
                },
                err_fn,
            )
            .unwrap(),
        _ => {
            todo!()
        }
    };
    stream.play().unwrap();

    print!("Press return to quit...");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn find_note(spectrum: &[f32], sample_rate: f32) -> () {
    let threshold = 30.;//100_000.;
    let freq_interval = sample_rate / (spectrum.len() as f32 * 2.);
    let peaks: Vec<usize> =
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
                        println!("---------------------------------");
                        println!("{}: {} ({})", i as f32 * freq_interval, x, freq_interval);
                        acc.push(i);
                    }
                }

                acc
            });
    //println!("{:?}", peaks);
}

enum Note<'a> {
    C,
    G,
    D,
    A,
    Above(&'a Note<'a>),
    Below(&'a Note<'a>),
    Unknown,
}
