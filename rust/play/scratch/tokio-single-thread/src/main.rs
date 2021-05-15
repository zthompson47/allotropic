use tokio::{
    process::Command,
    runtime,
    time::{sleep, Duration},
};

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let timer = rt.spawn(async {
        for _ in 0..10 {
            sleep(Duration::from_millis(1000)).await;
            println!("SPAWN world!");
        }
    });
    rt.block_on(async move {
        sleep(Duration::from_millis(2500)).await;
        println!("Jello world!");
        std::thread::spawn(|| {
            bubbawubba();
            std::thread::park();
        });
        println!("{}", String::from_utf8(weather().await).unwrap());
        timer.await.unwrap();
    });
}

async fn weather() -> Vec<u8> {
    Command::new("curl")
        .arg("wttr.in/14850")
        .output()
        .await
        .unwrap()
        .stdout
}

fn bubbawubba() {
    println!("bubbbbbbbb");
}
