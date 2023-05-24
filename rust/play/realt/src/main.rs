fn main() -> Result<(), Box<dyn std::error::Error>> {
    # std::env::set_var("SDL_AUDIODRIVER", "dummy");
    use std::time::Duration;
    use timbre::prelude::*;

    // SDL setup.
    let sdl = sdl2::init()?;
    let audio = sdl.audio()?;

    // Inputs
    let mut microphone = timbre::drivers::Sdl2Input::new(&audio)?;
    microphone.resume();
    let music = timbre::decoders::WavDecoder::from_file("./assets/music-stereo-f32.wav")?;

    // Apply effects
    let microphone = timbre::effects::Echo::new(microphone.source(),
            Duration::from_secs_f32(0.5), 0.6);
    let music = timbre::effects::LowPass::new(music, 200.0);

    // Mix them together
    let mut mixer = timbre::effects::BasicMixer::new();
    mixer.add_source(microphone.into_shared());
    mixer.add_source(music.into_shared());

    // Output
    let mut speaker = timbre::drivers::Sdl2Output::new(&audio)?;
    speaker.set_source(mixer.into_shared());
    speaker.resume();

    // std::thread::sleep(Duration::from_secs_f32(120.0));
    Ok(())
}
