use raylib::core::audio::RaylibAudio;
use raylib::prelude::*;
use std::env;
use std::process::exit;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("raylib [shaders] example")
        .build();
    let audio_init = RaylibAudio::init_audio_device();
    let audio: RaylibAudio;
    match audio_init {
        Ok(value) => {
            println!("Audio Loading Success");
            audio = value;
        }
        Err(e) => {
            println!("Audio Loading Failed with {e}");
            exit(1);
        }
    }
    // let new_sound = audio.new_sound("assets/Let It Happen.flac");
    let new_music = audio.new_music("assets/Let It Happen.flac");
    let sound: Music;
    match new_music {
        Ok(value) => {
            println!("Sound Loaded");
            sound = value;
        }
        Err(e) => {
            let path = env::current_dir().unwrap();
            println!("Couldn't find sound file {e} in path {}", path.display());
            exit(1);
        }
    }
    let mut pause = true;
    sound.play_stream();
    while !rl.window_should_close() {
        sound.update_stream();
        // sound.is_music_valid()
        // new_music.resume_stream();
        let played = sound.get_time_played();
        let valid = sound.is_music_valid();
        // println!("Already played: {played} {valid}");
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }
        if pause {
            sound.pause_stream();
        } else {
            sound.resume_stream();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
    }
}
