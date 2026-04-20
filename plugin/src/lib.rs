use std::thread;

use raylib::prelude::*;
mod fft;
mod music;
use music::*;
pub struct State {
    pub window_closed: bool,
}

pub struct DrawState {
    pub y: i32,
    pub w: i32,
    pub x: i32,
    pub h: i32,
    pub c: Color,
}

const SAMPLE_SIZE: usize = 1 << 13;

// #[unsafe(no_mangle)]
// pub fn draw_something_on_raylib<'a>() -> Music<'a> {
//     // let audio = RaylibAudio::init_audio_device().expect("Audio failed");
//     // let music = audio.new_music("assets/test.mp3").expect("File not found");
//     let audio = RaylibAudio::init_audio_device().expect("Audio failed");
//     let mut music = load_music(&audio, "assets/test.mp3".to_string());
//     return music;
//     // let mut music = load_music(audio, "assets/test.mp3".to_string());
//     // music.play_stream();
// }

#[unsafe(no_mangle)]
pub fn calc_frame_vars(
    state: &mut DrawState,
    t: f32,
    total_valid_samples: usize,
    cell_width: f32,
    screen_height: f32,
) {
    state.x = (total_valid_samples as f32 * cell_width) as i32;
    state.y = ((screen_height) - 10.0 - (screen_height * t / 2.0)) as i32;
    state.w = cell_width as i32;
    state.h = (screen_height * t / 2.0) as i32;
    state.c = Color::color_from_hsv(257.0, 0.71, 0.93);
}
