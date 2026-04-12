use raylib::prelude::*;
use std::sync::{Arc, Mutex};
mod fft;
use fft::*;
use num_complex::Complex;
pub struct RaylibStuff {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub audio: RaylibAudio,
}

pub struct MusicStuff<'a> {
    pub music: Music<'a>,
    pub samples: Arc<Mutex<Vec<f32>>>,
}

#[unsafe(no_mangle)]
pub fn get_box_color() -> Color {
    Color::YELLOWGREEN
}

#[unsafe(no_mangle)]
pub fn init_raylib_and_audio() -> RaylibStuff {
    let (rl, thread) = raylib::init().size(1280, 720).title("Ruzik").build();
    let audio = RaylibAudio::init_audio_device().expect("Audio failed");
    return RaylibStuff { rl, thread, audio };
}
#[unsafe(no_mangle)]
pub fn load_music<'a>(audio: &'a RaylibAudio, filepath: String, size: usize) -> MusicStuff<'a> {
    let music = audio.new_music(&filepath).expect("File not found");
    return MusicStuff {
        music,
        samples: Arc::new(Mutex::new(vec![0.0; size])),
    };
}

pub struct SampleHandler {
    pub samples: Arc<Mutex<Vec<f32>>>,
}
impl SampleHandler {
    #[unsafe(no_mangle)]
    pub fn clone_samples(&self) -> Arc<Mutex<Vec<f32>>> {
        return Arc::clone(&self.samples);
    }
    #[unsafe(no_mangle)]
    pub fn fft_wrapper(&self) -> (Vec<f32>, f32, usize) {
        let mut com_samples: Vec<Complex<f32>> = Vec::new();
        let mut max_amp = 0.0;
        if let Ok(samples) = self.samples.lock() {
            com_samples = f32_to_complex_mutex(&samples);
            com_samples = fft(&com_samples);
            max_amp = 0.0;
            for i in 0..com_samples.len() {
                if max_amp < com_samples[i].re.abs() && max_amp < com_samples[i].im.abs() {
                    max_amp = com_samples[i].re.abs();
                }
            }
        }
        let step = 1.06;
        let mut m: usize = 0;
        let mut f = 20.0;
        while f < com_samples.len() as f32 {
            m += 1;
            f *= step;
        }
        return (complex_to_f32(&com_samples), max_amp, m);
    }
}
#[unsafe(no_mangle)]
pub fn init_sample_handler(size: usize) -> SampleHandler {
    SampleHandler {
        samples: Arc::new(Mutex::new(vec![0.0; size])),
    }
}
#[unsafe(no_mangle)]
pub fn render_frame(d: &mut RaylibDrawHandle, sample_handler: &SampleHandler) {
    let (samples, max_amp, mut m) = sample_handler.fft_wrapper();
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
            Color::RED,
        );
        m += 1;
        f *= step;
    }
}
