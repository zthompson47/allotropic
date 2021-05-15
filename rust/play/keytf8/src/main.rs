use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::style::Colorize;
use rodio::{Decoder, OutputStream, Sink};

use keytf8::terminal::with_raw_mode;

enum Set {
    Animal,
    All,
    AllPlus,
    Symbol,
    Rando,
    Party,
    More,
    Text,
}

impl Set {
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            48 => Some(Self::All),
            49 => Some(Self::Animal),
            50 => Some(Self::Symbol),
            51 => Some(Self::Rando),
            52 => Some(Self::Party),
            53 => Some(Self::More),
            54 => Some(Self::AllPlus),
            57 => Some(Self::Text),
            _ => None,
        }
    }
}

enum Color {
    Green,
    Blue,
    Yellow,
    Magenta,
    White,
    Cyan,
    Red,
}

impl Color {
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            33 => Some(Self::Green),
            64 => Some(Self::Blue),
            35 => Some(Self::Yellow),
            36 => Some(Self::Magenta),
            37 => Some(Self::White),
            94 => Some(Self::Cyan),
            41 => Some(Self::Red),
            _ => None,
        }
    }
}

const WAV_PATH: &str = "/home/zach/.local/share/keytf8/sound.wav";

fn main() -> Result<(), anyhow::Error> {
    // Audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Audio recording
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();

    with_raw_mode(|| {
        let mut stdin = std::io::stdin();
        let mut buf: [u8; 1] = [0; 1];
        let mut set = Set::All;
        let mut color = Color::White;
        loop {
            stdin.read_exact(&mut buf).unwrap();
            if buf[0] == 27 {
                print!("\r\n");
                break;
            }
            if buf[0] == 55 {
                let sink = Sink::try_new(&stream_handle).unwrap();
                let source = Decoder::new_wav(
                    std::fs::File::open(WAV_PATH).unwrap(),
                )
                .unwrap();
                sink.append(source);
                sink.detach();
            }
            if buf[0] == 56 {
                let config = device.default_input_config().unwrap();
                let spec = wav_spec_from_config(&config);
                let writer = hound::WavWriter::create(WAV_PATH, spec).unwrap();
                let writer = Arc::new(Mutex::new(Some(writer)));
                let writer_2 = writer.clone();
                let err_fn = move |err| {
                    eprintln!("an error occurred on stream: {}", err);
                };
                let stream = match config.sample_format() {
                    cpal::SampleFormat::F32 => device.build_input_stream(
                        &config.into(),
                        move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
                        err_fn,
                    ).unwrap(),
                    cpal::SampleFormat::I16 => device.build_input_stream(
                        &config.into(),
                        move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
                        err_fn,
                    ).unwrap(),
                    cpal::SampleFormat::U16 => device.build_input_stream(
                        &config.into(),
                        move |data, _: &_| write_input_data::<u16, i16>(data, &writer_2),
                        err_fn,
                    ).unwrap(),
                };
                print!("RECORD on");
                std::io::stdout().flush().unwrap();
                stream.play().unwrap();
                std::thread::sleep(std::time::Duration::from_secs(2));
                print!("RECORD off");
                std::io::stdout().flush().unwrap();
                drop(stream);
                writer.lock().unwrap().take().unwrap().finalize().unwrap();
            }
            if let Some(new_set) = Set::from_byte(buf[0]) {
                set = new_set;
                continue;
            }
            if let Some(new_color) = Color::from_byte(buf[0]) {
                color = new_color;
                continue;
            }
            match set {
                Set::Animal => print!("{}", animal(buf[0])),
                Set::All => print!(
                    "{}{}{}{}{}",
                    animal(buf[0]),
                    symbol(buf[0]),
                    rando(buf[0]),
                    party(buf[0]),
                    more(buf[0])
                ),
                Set::AllPlus => print!(
                    "[{}]{}{}{}{}{}",
                    buf[0],
                    animal(buf[0]),
                    symbol(buf[0]),
                    rando(buf[0]),
                    party(buf[0]),
                    more(buf[0])
                ),
                Set::Symbol => print!("{}", symbol(buf[0])),
                Set::Rando => print!("{}", rando(buf[0])),
                Set::Party => print!("{}", party(buf[0])),
                Set::More => print!("{}", more(buf[0])),
                Set::Text => print!("{}", match std::str::from_utf8(&[buf[0]]) {
                    Ok(s) => match color {
                        Color::Green => s.green().to_string(),
                        Color::Blue => s.blue().to_string(),
                        Color::Yellow => s.yellow().to_string(),
                        Color::Magenta => s.magenta().to_string(),
                        Color::Cyan => s.cyan().to_string(),
                        Color::Red => s.red().to_string(),
                        Color::White => s.to_string(),
                    }
                    Err(_) => "?".to_string(),
                }),
            }
            std::io::stdout().flush().unwrap();
        }
    })
}

fn animal(seed: u8) -> String {
    let v: Vec<u8> = vec![240, 159, 144, 128 + seed % 63];
    String::from_utf8(v).unwrap_or("0".into())
}

fn symbol(seed: u8) -> String {
    let v: Vec<u8> = vec![240, 159, 148, 133 + seed % 20];
    String::from_utf8(v).unwrap_or("0".into())
}

fn rando(seed: u8) -> String {
    let v: Vec<u8> = vec![240, 159, 145, 128 + seed % 63];
    String::from_utf8(v).unwrap_or("0".into())
}

fn party(seed: u8) -> String {
    let v: Vec<u8> = vec![240, 159, 146, 128 + seed % 63];
    String::from_utf8(v).unwrap_or("0".into())
}

fn more(seed: u8) -> String {
    let v: Vec<u8> = vec![240, 159, 147, 128 + seed % 63];
    String::from_utf8(v).unwrap_or("0".into())
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    match format {
        cpal::SampleFormat::U16 => hound::SampleFormat::Int,
        cpal::SampleFormat::I16 => hound::SampleFormat::Int,
        cpal::SampleFormat::F32 => hound::SampleFormat::Float,
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = cpal::Sample::from(&sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
