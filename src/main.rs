use raylib::prelude::*;
mod fft;
mod music;
use music::*;
const SAMPLE_SIZE: usize = 1 << 14;
fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Ruzik").build();
    let audio = RaylibAudio::init_audio_device().expect("Audio failed");
    let music = load_music(
        &audio,
        "/home/darsh/devel/ruzik/assets/TestKick.mp3".to_string(),
    );
    let sample_handler = init_sample_handler(SAMPLE_SIZE);
    let mut processor = processor(sample_handler.clone_samples());
    let _processor_handle = attach_audio_stream_processor_to_music(&music, &mut processor);
    music.play_stream();
    let mut pause: bool = false;
    let mut p: i32 = 0;
    let (mut samples, mut max_amp, mut m) = sample_handler.fft_wrapper();
    while !rl.window_should_close() {
        music.update_stream();
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
        // // println!("***{:?}***", samples);
        if p % 25 == 0 {
            (samples, max_amp, m) = sample_handler.fft_wrapper();
            println!("{:?}", samples.len());
        }
        let sw = d.get_render_width() as f32;
        let sh = d.get_render_height() as f32;
        let cell_width = sw / m as f32;
        let step = 1.059;
        m = 0;
        let mut f = 20.0;
        while f < samples.len() as f32 {
            let f1 = f * step;
            let mut a = 0.0;
            let mut q: usize = f as usize;
            while q < samples.len() && q < (f1 as usize) {
                a += samples[q];
                q += 1;
            }
            a /= f1 - f + 1.0;
            let t = a / max_amp;
            d.draw_rectangle(
                (m as f32 * cell_width) as i32,
                ((sh) - 10.0 - (sh * t / 2.0)) as i32,
                cell_width as i32,
                (sh * t / 2.0) as i32,
                Color::YELLOW,
            );
            m += 1;
            f *= step;
        }
        p += 1;
    }
}
