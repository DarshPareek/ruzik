use raylib::core::audio::RaylibAudio;
use raylib::prelude::*;
use std::env;
use std::i16::{MAX, MIN};
use std::process::exit;
static mut GLOBAL_FRAMES: [f32; 1024] = [0.0; 1024];
static mut GLOBAL_FRAMES_LEN: usize = 0;
fn callback(buffer_data: &mut [f32], frame: u32) {
    // println!("Inside");
    unsafe {
        for i in 0..buffer_data.len() {
            GLOBAL_FRAMES[i] = buffer_data[i];
        }
        GLOBAL_FRAMES_LEN = buffer_data.len();
    }
    // println!("Frame Data: {frame}");
}
fn main() {
    let (mut rl, thread) = raylib::init().size(1600, 450).title("Ruzik").build();
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
    let new_music = audio.new_music("assets/Let It Happen.flac");
    let music: Music;
    match new_music {
        Ok(value) => {
            println!("music Loaded");
            music = value;
        }
        Err(e) => {
            let path = env::current_dir().unwrap();
            println!("Couldn't find music file {e} in path {}", path.display());
            exit(1);
        }
    }
    let mut pause = false;
    music.play_stream();
    // attach_audio_stream_processor_to_music(&music, &mut callback);
    let music_stream = music.stream;
    let processor = &mut callback;
    let audioprocessor = attach_audio_stream_processor_to_music(&music, processor);
    let sw = rl.get_render_width();
    let sh = rl.get_render_height() as i16;
    println!("FrameCount: {}", music.frameCount);
    println!("Samples per second: {}", music.stream.sampleRate);
    println!("Samples per second: {}", music.stream.sampleSize);
    println!("Samples per second: {}", music.stream.channels);
    while !rl.window_should_close() {
        music.update_stream();
        let played = music.get_time_played();
        // println!("Already played: {played}");
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }
        if pause {
            music.pause_stream();
        } else {
            music.resume_stream();
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKBLUE);
        let mut cell_width: i32 = 0;
        unsafe {
            if GLOBAL_FRAMES_LEN > 0 {
                cell_width = sw / (GLOBAL_FRAMES_LEN as i32);
            }
            for i in 0..GLOBAL_FRAMES_LEN {
                let sample = (GLOBAL_FRAMES[i] * 1000.0) as i16;
                println!("Sample {sample} i {i}");
                if sample > 0 {
                    let t = (sample / MAX);
                    println!(
                        "{}, {}, {}, {}, {}",
                        (i as i32) * cell_width,
                        (sh / 2 - sh / 2 * t) as i32,
                        cell_width,
                        (sh / 2) as i32,
                        sample as f32 / 10.0
                    );
                    // d.draw_rectangle(x, y, width, height, color);
                    d.draw_rectangle(
                        (i as i32) * cell_width,
                        (sh / 2 - sh / 2 * t) as i32,
                        cell_width,
                        ((sh / 2) as f32 * (sample as f32) / 30.0) as i32,
                        Color::RED,
                    );
                } else {
                    let t = sample / MIN;
                    d.draw_rectangle(
                        (i as i32) * cell_width,
                        0,
                        cell_width,
                        ((sh / 2) as f32 * (sample as f32)) as i32,
                        Color::RED,
                    );
                }
            }
        }
    }
}
