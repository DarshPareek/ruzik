use crate::fft::*;
use num_complex::Complex;
use raylib::prelude::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
pub struct Frames {
    pub frame: Vec<f32>,
}
impl Frames {
    pub fn init_frames(channel_count: usize) -> Frames {
        return Frames {
            frame: Vec::with_capacity(channel_count),
        };
    }
}
pub struct SampleHandler {
    pub samples: Arc<Mutex<VecDeque<Frames>>>,
}
impl SampleHandler {
    pub fn clone_samples(&self) -> Arc<Mutex<VecDeque<Frames>>> {
        return Arc::clone(&self.samples);
    }
    pub fn fft_wrapper(&self) -> (Vec<f32>, f32, usize) {
        let mut com_samples: Vec<Complex<f32>> = Vec::new();
        let mut max_amp = 0.0;
        if let Ok(mut samples) = self.samples.lock() {
            com_samples = fft(&f32_to_complex_mutex(&mut samples));
            // println!("{:?}", com_samples.len());
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
pub fn init_sample_handler(size: usize, num_channels: u32) -> SampleHandler {
    let mut temp: VecDeque<Frames> = VecDeque::new();
    for _ in 0..size {
        temp.push_back(Frames::init_frames(num_channels as usize));
    }
    SampleHandler {
        samples: Arc::new(Mutex::new(temp)),
    }
}
pub fn processor(
    samples: Arc<Mutex<VecDeque<Frames>>>,
) -> impl FnMut(&mut [f32], u32) + Send + 'static {
    let my_processor = move |buffer: &mut [f32], _frames: u32| {
        let limit = buffer.len();
        // let mut left: Vec<f32> = Vec::with_capacity(limit / 2);
        // let mut right: Vec<f32> = Vec::with_capacity(limit / 2);
        let mut i = 0;
        while i < limit {
            let mut j = 0;
            if let Ok(mut data) = samples.lock() {
                if data.capacity() == data.len() {
                    data.pop_front();
                }
                let mut f = Frames::init_frames(2);
                while j < _frames as usize {
                    f.frame.push(buffer[i]);
                    j += 1;
                }
                data.push_back(f);
            }
            i += 1;
        }
        // if let Ok(data) = samples.lock() {
        //     println!(
        //         "Adding a total of {:?}, each has frame of {:?}",
        //         data.len(),
        //         data[0].frame.len()
        //     );
        // }
        // for i in 0..limit {
        //     if i % 2 == 0 {
        //         left.push(buffer[i]);
        //     } else {
        //         right.push(buffer[i]);
        //     }
        // }
        // if let Ok(mut data) = samples.lock() {
        //     // for i in 0..left.len() {
        //     //     if data.len() == data.capacity() {
        //     //         data.pop_front();
        //     //     }
        //     //     data.push_back(left[i]);
        //     // }
        //     // for i in 0..right.len() {
        //     //     if data.len() == data.capacity() {
        //     //         data.pop_front();
        //     //     }
        //     //     data.push_back(right[i]);
        //     // }
        //     // println!("Filled total {:?}", data.len());
        //     // println!("Filled {:?} right", right.len());
        //     // println!("Filled {:?} left", left.len());
        //     for i in 0..left.len() {
        //         if data.len() == data.capacity() {
        //             data.pop_back();
        //         }
        //         data.push_front(left[i]);
        //     }
        //     // for i in 0..left.len() {
        //     //     if data.len() == data.capacity() {
        //     //         data.pop_back();
        //     //     }
        //     //     data.push_front(0.0);
        //     // }
        //     // for i in 0..right.len() {
        //     //     if data.len() == data.capacity() {
        //     //         data.pop_front();
        //     //     }
        //     //     data.push_back(right[i]);
        //     // }
        // }
    };
    return my_processor;
}
