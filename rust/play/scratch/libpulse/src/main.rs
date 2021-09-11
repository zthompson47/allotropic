use libpulse_binding as pulse;
use libpulse_simple_binding as psimple;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample::{Spec, Format};
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

use tapedeck::cmd::ffmpeg;

#[tokio::main]
async fn main() {
    let (tx, mut rx): (mpsc::Sender<[u8; 4096]>, _) = mpsc::channel(8);

    let t = std::thread::spawn(move || {
        let spec = Spec {
            format: Format::S16le,
            channels: 2,
            rate: 44100,
        };
        assert!(spec.is_valid());

        let s = Simple::new(
            None,                // Use the default server
            "FooApp",            // Our application’s name
            Direction::Playback, // We want a playback stream
            None,                // Use the default device
            "Music",             // Description of our stream
            &spec,               // Our sample format
            None,                // Use default channel map
            None                 // Use default buffering attributes
        ).unwrap();

        while let Some(buf) = rx.blocking_recv() {
            s.write(&buf).unwrap();
        }
    });

    let music = "https://somafm.com/defcon.pls";
    let mut source = ffmpeg::read(music).await.spawn().unwrap();
    let mut reader = source.stdout.take().unwrap();
    let mut buf = [0u8; 4096];

    while let Ok(_len) = reader.read_exact(&mut buf).await {
        tx.send(buf).await.unwrap();
    }

    t.join().unwrap();
}
