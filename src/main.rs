use raylib::prelude::*;

mod fft;
mod music;
use music::*;

const SAMPLE_SIZE: usize = 1 << 12;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Ruzik").build();
    let audio = RaylibAudio::init_audio_device().expect("Audio failed");

    // 1. Make music mutable so we can reassign it later
    let mut music = load_music(
        &audio,
        "/home/darsh/devel/ruzik/assets/Let It Happen.flac".to_string(),
    );
    let sample_handler = init_sample_handler(SAMPLE_SIZE);
    let mut processor = processor(sample_handler.clone_samples());
    let mut processor_handle = Some(attach_audio_stream_processor_to_music(
        &music,
        &mut processor,
    ));
    music.play_stream();
    let mut pause: bool = false;
    let mut p: i32 = 0;
    let (mut samples, mut max_amp, mut m) = sample_handler.fft_wrapper();
    while !rl.window_should_close() {
        if rl.is_file_dropped() {
            let dropped_files = rl.load_dropped_files();
            let file_paths = dropped_files.paths();
            if file_paths.len() > 0 {
                let file_path = file_paths[0];
                if file_path.contains(".mp3") || file_path.contains(".flac") {
                    println!("New Music Dropped: {}", file_path);
                    music.stop_stream();
                    processor_handle = None;
                    music = load_music(&audio, file_path.to_string());
                    processor_handle = Some(attach_audio_stream_processor_to_music(
                        &music,
                        &mut processor,
                    ));
                    music.play_stream();
                    pause = false;
                }
            }
            for path in file_paths {
                println!("Dropped path: {}", path);
            }
        }
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
        if p % 25 == 0 {
            (samples, max_amp, m) = sample_handler.fft_wrapper();
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
