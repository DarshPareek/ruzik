use raylib::prelude::*;
mod fft;
mod music;
use fft::*;
use music::*;
#[hot_lib_reloader::hot_module(dylib = "plugin")]
mod hot_lib {
    use raylib::prelude::*;
    hot_functions_from_file!("plugin/src/lib.rs");
}
fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Ruzik").build();
    let audio = RaylibAudio::init_audio_device().expect("Audio failed");
    let music = load_music(&audio, "assets/Let It Happen.flac".to_string());
    let sample_handler = init_sample_handler();
    let mut processor = processor(sample_handler.clone_samples());
    let _processor_handle = attach_audio_stream_processor_to_music(&music, &mut processor);
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
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let (com_samples, max_amp) = sample_handler.fft_wrapper();
        let sw = d.get_render_width() as f32;
        let sh = d.get_render_height() as f32;
        let step = sw / com_samples.len() as f32;
        println!("{:?}", com_samples.len());
        for i in 0..com_samples.len() {
            let t = amp(com_samples[i]) / max_amp;
            let x = (i as f32 * step) as i32;
            let bar_h = (t * sh) as i32;
            d.draw_rectangle(
                x,
                (sh as i32) - 10 - (bar_h / 2),
                2,
                bar_h / 2 as i32,
                current_color,
            );
        }
    }
}
