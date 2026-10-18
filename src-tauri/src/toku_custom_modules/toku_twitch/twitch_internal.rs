use std::fs::File;
use std::io::BufReader;

pub async fn reproduce_sound(sound_dir: String, sound_volume: u8) -> Result<String, ()> {
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let file = BufReader::new(File::open(sound_dir).unwrap());
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();
    let volume_float: f32 = sound_volume as f32;
    player.set_volume(volume_float / 100.0);
    player.play();
    player.sleep_until_end();
    Ok(String::from("Sound ended"))
}