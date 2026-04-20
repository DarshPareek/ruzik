use plugin::calc_frame_vars;
// dylib: the platform independent library name, typically the crate name
// lib_dir: where to find the library file. Defaults to "target/debug" and "target/release" for debug / release builds
// file_watch_debounce: Debounce duration in milliseconds for the file watcher checking for library changes 500ms is the default.
use raylib::prelude::*;
mod fft;
mod music;
use music::*;
const SAMPLE_SIZE: usize = 1 << 13;

#[hot_lib_reloader::hot_module(
    dylib = "plugin",
    lib_dir = if cfg!(debug_assertions) { "target/debug" } else { "target/release" },
    file_watch_debounce = 500,
    loaded_lib_name_template = "{lib_name}_hot_{pid}_{load_counter}"
)]
mod hot_lib {
    pub use plugin::*;
    use raylib::prelude::Color;
    // embeds hot reloadable proxy functions for all public functions, even
    // those that are not #[unsafe(no_mangle)] in that rust source file
    hot_functions_from_file!("plugin/src/lib.rs", ignore_no_mangle = true);

    // manually expose functions. Note there actually isn't such a function in lib.
    #[hot_functions]
    extern "Rust" {
        pub fn do_stuff2(arg: &str) -> u32;
    }

    // allows you to wait for about-to-reload and reloaded events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    // a monotonically increasing counter (starting with 0) that counts library reloads
    #[lib_version]
    pub fn version() -> usize {}

    // Expose a query function to test if the lib was reloaded. Note that this
    // function will return true only _once_ after a reload.
    #[lib_updated]
    pub fn was_updated() -> bool {}
}

fn main() {
    // let update_blocker = hot_lib::subscribe().wait_for_about_to_reload();
    // println!("about to reload...");
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // drop(update_blocker);
    // println!("read for reload...");

    // hot_lib::subscribe().wait_for_reload();
    // println!("reloaded at version {} now", hot_lib::version());

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Ruzik")
        .resizable()
        .build();

    let audio = RaylibAudio::init_audio_device().expect("Audio failed");
    let mut music = load_music(&audio, "assets/test.mp3".to_string());
    let sample_handler = init_sample_handler(SAMPLE_SIZE, music.stream.channels);
    let mut processor = processor(sample_handler.clone_samples());
    let mut processor_handle = Some(attach_audio_stream_processor_to_music(
        &music,
        &mut processor,
    ));

    music.play_stream();
    let mut pause: bool = false;
    let mut samples: Vec<f32>;
    let mut max_amp: f32;
    let mut total_valid_samples: usize;
    let mut draw_state = hot_lib::DrawState {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
        c: Color::BLACK,
    };
    while !rl.window_should_close() {
        if rl.is_file_dropped() {
            let dropped_files = rl.load_dropped_files();
            let file_paths = dropped_files.paths();
            if file_paths.len() > 0 {
                let file_path = file_paths[0];
                if file_path.contains(".mp3")
                    || file_path.contains(".flac")
                    || file_path.contains(".wav")
                    || file_path.contains(".opus")
                    || file_path.contains(".ogg")
                    || file_path.contains(".wav")
                {
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
        // if p % 75 == 0 {
        (samples, max_amp, total_valid_samples) = sample_handler.fft_wrapper();
        // }
        let sw = d.get_render_width() as f32;
        let sh = d.get_render_height() as f32;
        let cell_width = sw / total_valid_samples as f32;
        let step = 1.059;
        total_valid_samples = 0;
        let mut f = 20.0;
        let mut draw_data: Vec<f32> = vec![];
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
            draw_data.push(t);

            hot_lib::calc_frame_vars(&mut draw_state, t, total_valid_samples, cell_width, sh);
            d.draw_rectangle(
                draw_state.x, //(total_valid_samples as f32 * cell_width) as i32,
                draw_state.y, //((sh) - 10.0 - (sh * t / 2.0)) as i32,
                draw_state.w, //cell_width as i32,
                draw_state.h, //(sh * t / 2.0) as i32,
                draw_state.c, //Color::YELLOW,
            );
            total_valid_samples += 1;
            f *= step;
        }
    }
}

// use raylib::prelude::*;
// mod fft;
// mod music;
// use music::*;

// const SAMPLE_SIZE: usize = 1 << 13;

// fn main() {
//     let (mut rl, thread) = raylib::init()
//         .size(1280, 720)
//         .title("Ruzik")
//         .resizable()
//         .build();
//     let audio = RaylibAudio::init_audio_device().expect("Audio failed");
//     let mut music = load_music(&audio, "assets/test.mp3".to_string());
//     let sample_handler = init_sample_handler(SAMPLE_SIZE, music.stream.channels);
//     let mut processor = processor(sample_handler.clone_samples());
//     let mut processor_handle = Some(attach_audio_stream_processor_to_music(
//         &music,
//         &mut processor,
//     ));
//     music.play_stream();
//     let mut pause: bool = false;
//     let mut samples: Vec<f32>;
//     let mut max_amp: f32;
//     let mut total_valid_samples: usize; //sample_handler.fft_wrapper();
//     while !rl.window_should_close() {
//         if rl.is_file_dropped() {
//             let dropped_files = rl.load_dropped_files();
//             let file_paths = dropped_files.paths();
//             if file_paths.len() > 0 {
//                 let file_path = file_paths[0];
//                 if file_path.contains(".mp3")
//                     || file_path.contains(".flac")
//                     || file_path.contains(".wav")
//                     || file_path.contains(".opus")
//                     || file_path.contains(".ogg")
//                     || file_path.contains(".wav")
//                 {
//                     println!("New Music Dropped: {}", file_path);
//                     music.stop_stream();
//                     processor_handle = None;
//                     music = load_music(&audio, file_path.to_string());
//                     processor_handle = Some(attach_audio_stream_processor_to_music(
//                         &music,
//                         &mut processor,
//                     ));
//                     music.play_stream();
//                     pause = false;
//                 }
//             }
//             for path in file_paths {
//                 println!("Dropped path: {}", path);
//             }
//         }
//         music.update_stream();
//         if pause {
//             music.pause_stream();
//         } else {
//             music.resume_stream();
//         }
//         if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
//             pause = !pause;
//         }
//         let mut d = rl.begin_drawing(&thread);
//         d.clear_background(Color::BLACK);
//         // if p % 75 == 0 {
//         (samples, max_amp, total_valid_samples) = sample_handler.fft_wrapper();
//         // }
//         let sw = d.get_render_width() as f32;
//         let sh = d.get_render_height() as f32;
//         let cell_width = sw / total_valid_samples as f32;
//         let step = 1.059;
//         total_valid_samples = 0;
//         let mut f = 20.0;
//         let mut draw_data: Vec<f32> = vec![];
//         while f < samples.len() as f32 {
//             let f1 = f * step;
//             let mut a = 0.0;
//             let mut q: usize = f as usize;
//             while q < samples.len() && q < (f1 as usize) {
//                 a += samples[q];
//                 q += 1;
//             }
//             a /= f1 - f + 1.0;
//             let t = a / max_amp;
//             draw_data.push(t);
//             d.draw_rectangle(
//                 (total_valid_samples as f32 * cell_width) as i32,
//                 ((sh) - 10.0 - (sh * t / 2.0)) as i32,
//                 cell_width as i32,
//                 (sh * t / 2.0) as i32,
//                 Color::YELLOW,
//             );
//             total_valid_samples += 1;
//             f *= step;
//         }
//     }
// }
