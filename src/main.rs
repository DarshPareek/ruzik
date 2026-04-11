use raylib::prelude::*;
mod fft;
use fft::*;
#[hot_lib_reloader::hot_module(dylib = "plugin")]
mod hot_lib {
    use raylib::prelude::*;
    use std::sync::{Arc, Mutex};
    hot_functions_from_file!("plugin/src/lib.rs");
}
use std::sync::{Arc, Mutex};
fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Ruzik").build();
    let audio = RaylibAudio::init_audio_device().expect("Audio failed");
    let music = audio
        .new_music("assets/Let It Happen.flac")
        .expect("File not found");
    let shared_samples = Arc::new(Mutex::new(vec![]));
    let samples_for_callback = Arc::clone(&shared_samples);
    let mut my_processor = move |buffer: &mut [f32], _frames: u32| {
        if let Ok(mut data) = samples_for_callback.lock() {
            data.clear();
            let limit = buffer.len().min(1280);
            data.extend_from_slice(&buffer[..limit]);
        }
    };
    let _processor_handle = attach_audio_stream_processor_to_music(&music, &mut my_processor);
    music.play_stream();
    let mut pause: bool = false;
    while !rl.window_should_close() {
        music.update_stream();
        let current_color = hot_lib::get_box_color();
        if pause {
            music.pause_stream();
        } else {
            music.resume_stream();
        }
        // hot_lib::plugin_update(&mut state);
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        if let Ok(samples) = shared_samples.lock() {
            // hot_lib::plugin_update(&mut state, samples.as_ptr(), samples.len());
            let mut com_samples = f32_to_complex_mutex(&samples);
            com_samples = fft(&com_samples);
            let mut max_amp = 0.0;
            for i in 0..com_samples.len() {
                if max_amp < com_samples[i].re.abs() {
                    max_amp = com_samples[i].re.abs();
                }
                if max_amp < com_samples[i].im.abs() {
                    max_amp = com_samples[i].im.abs();
                }
            }
            let sw = d.get_render_width() as f32;
            let sh = d.get_render_height() as f32;
            let step = sw / samples.len() as f32;
            for i in 0..com_samples.len() {
                let t = amp(com_samples[i]) / max_amp;
                let x = (i as f32 * step) as i32;
                let bar_h = (t * sh) as i32;
                let color: Color = Color::RED;
                d.draw_rectangle(
                    x,
                    (sh as i32) - 10 - (bar_h / 2),
                    step.ceil() as i32,
                    bar_h / 2 as i32,
                    current_color,
                );
            }
        }
    }
}
