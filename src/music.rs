use crate::fft::*;
use num_complex::Complex;
use raylib::prelude::*;
use std::sync::{Arc, Mutex};
pub struct SampleHandler {
    pub samples: Arc<Mutex<Vec<f32>>>,
}
impl SampleHandler {
    pub fn clone_samples(&self) -> Arc<Mutex<Vec<f32>>> {
        return Arc::clone(&self.samples);
    }
    pub fn fft_wrapper(&self) -> (Vec<Complex<f32>>, f32) {
        let mut com_samples: Vec<Complex<f32>> = Vec::new();
        let mut max_amp = 0.0;
        if let Ok(samples) = self.samples.lock() {
            com_samples = f32_to_complex_mutex(&samples);
            com_samples = fft(&com_samples);
            max_amp = 0.0;
            for i in 0..com_samples.len() {
                if max_amp < com_samples[i].re.abs() {
                    max_amp = com_samples[i].re.abs();
                }
                if max_amp < com_samples[i].im.abs() {
                    max_amp = com_samples[i].im.abs();
                }
            }
        }
        return (com_samples, max_amp);
    }
}
pub fn load_music<'a>(audio: &'a RaylibAudio, filepath: String) -> Music<'a> {
    // let audio = RaylibAudio::init_audio_device().expect("Audio failed");
    let music = audio.new_music(&filepath).expect("File not found");
    return music;
}
pub fn init_sample_handler() -> SampleHandler {
    SampleHandler {
        samples: Arc::new(Mutex::new(vec![])),
    }
}
pub fn processor(samples: Arc<Mutex<Vec<f32>>>) -> impl FnMut(&mut [f32], u32) + Send + 'static {
    let my_processor = move |buffer: &mut [f32], _frames: u32| {
        if let Ok(mut data) = samples.lock() {
            data.clear();
            let limit = buffer.len().min(1 << 14);
            data.extend_from_slice(&buffer[..limit])
        }
    };
    return my_processor;
}
