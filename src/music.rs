use crate::fft::*;
use num_complex::Complex;
use raylib::prelude::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
pub struct SampleHandler {
    pub samples: Arc<Mutex<VecDeque<f32>>>,
}
impl SampleHandler {
    pub fn clone_samples(&self) -> Arc<Mutex<VecDeque<f32>>> {
        return Arc::clone(&self.samples);
    }
    pub fn fft_wrapper(&self) -> (Vec<f32>, f32, usize) {
        let mut com_samples: Vec<Complex<f32>> = Vec::new();
        let mut max_amp = 0.0;
        if let Ok(samples) = self.samples.lock() {
            com_samples = fft(&f32_to_complex_mutex(&samples));
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
pub fn load_music<'a>(audio: &'a RaylibAudio, filepath: String) -> Music<'a> {
    let music = audio.new_music(&filepath).expect("File not found");
    return music;
}
pub fn init_sample_handler(size: usize) -> SampleHandler {
    SampleHandler {
        samples: Arc::new(Mutex::new(VecDeque::with_capacity(size))),
    }
}
pub fn processor(
    samples: Arc<Mutex<VecDeque<f32>>>,
) -> impl FnMut(&mut [f32], u32) + Send + 'static {
    let my_processor = move |buffer: &mut [f32], _frames: u32| {
        if let Ok(mut data) = samples.lock() {
            let limit = buffer.len();
            for i in 0..limit {
                if data.len() == data.capacity() {
                    data.pop_front();
                }
                data.push_back(buffer[i]);
            }
        }
    };
    return my_processor;
}
