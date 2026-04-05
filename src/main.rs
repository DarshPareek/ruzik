use raylib::ffi;
use raylib::prelude::*;
use std::env;
use std::process::exit;

static mut GLOBAL_FRAMES: [f32; 1024] = [0.0; 1024];
static mut GLOBAL_FRAMES_LEN: usize = 0;

unsafe extern "C" fn audio_callback(buffer_data: *mut std::ffi::c_void, frames: u32) {
    let f32_ptr = buffer_data as *const f32;
    let channels = 2;
    let total_samples = (frames * channels) as usize;
    let limit = total_samples.min(1024);
    for i in 0..limit {
        GLOBAL_FRAMES[i] = *f32_ptr.add(i);
    }
    GLOBAL_FRAMES_LEN = limit;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .resizable()
        .size(1980, 1080)
        .title("Ruzik")
        .fullscreen()
        .build();
    let audio = RaylibAudio::init_audio_device().unwrap_or_else(|e| {
        println!("Audio Loading Failed with {e}");
        exit(1);
    });

    let mut music = audio
        .new_music("assets/One More Hour.flac")
        .unwrap_or_else(|e| {
            let path = env::current_dir().unwrap();
            println!("Couldn't find music file {e} in path {}", path.display());
            exit(1);
        });

    let mut pause = false;
    music.play_stream();

    unsafe {
        ffi::AttachAudioStreamProcessor(music.stream, Some(audio_callback));
    }

    let sw = rl.get_render_width();
    let sh = rl.get_render_height() as i32;

    println!("FrameCount: {}", music.frameCount);
    println!("Samples per second: {}", music.stream.sampleRate);
    println!("Channels: {}", music.stream.channels);

    while !rl.window_should_close() {
        music.update_stream();

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }

        if pause {
            music.pause_stream();
        } else {
            music.resume_stream();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        unsafe {
            if GLOBAL_FRAMES_LEN > 0 {
                let step = (sw as f32) / (GLOBAL_FRAMES_LEN as f32);
                for i in 0..GLOBAL_FRAMES_LEN {
                    let sample = GLOBAL_FRAMES[i];
                    let bar_height = (sample.abs() * (sh as f32)) as i32;
                    let width = step.ceil() as i32;
                    let x = (i as f32 * step) as i32;
                    let y = sh / 2 - bar_height / 2;
                    let color = if sample > 0.0 {
                        Color::LIGHTGREEN
                    } else {
                        Color::MAROON
                    };
                    d.draw_rectangle(x, y, width, bar_height, color);
                }
            }
        }
    }
    unsafe {
        ffi::DetachAudioStreamProcessor(music.stream, Some(audio_callback));
    }
}
