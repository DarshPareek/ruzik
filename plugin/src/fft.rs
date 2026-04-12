use num_complex::Complex;
use std::f32::consts::PI;
use std::sync::MutexGuard;
const I: Complex<f32> = Complex { re: 0.0, im: 1.0 };

pub fn fft(input: &Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    fn fft_inner(buf_a: &mut [Complex<f32>], buf_b: &mut [Complex<f32>], n: usize, step: usize) {
        if step >= n {
            return;
        }
        fft_inner(buf_b, buf_a, n, step * 2);
        fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);
        let (left, right) = buf_a.split_at_mut(n / 2);
        for i in (0..n).step_by(step * 2) {
            let t = (-I * PI * (i as f32) / (n as f32)).exp() * buf_b[i + step];
            left[i / 2] = buf_b[i] + t;
            right[i / 2] = buf_b[i] - t;
        }
    }
    let n_orig = input.len();
    let n = n_orig.next_power_of_two();
    let mut buf_a = input.to_vec();
    buf_a.append(&mut vec![Complex { re: 0.0, im: 0.0 }; n - n_orig]);
    let mut buf_b = buf_a.clone();
    fft_inner(&mut buf_a, &mut buf_b, n, 1);
    buf_a
}
pub fn complex_to_f32(input: &Vec<Complex<f32>>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    for i in 0..input.len() {
        res.push(amp(input[i]));
    }
    return res;
}
// pub fn f32_to_complex(input: &Vec<f32>) -> Vec<Complex<f32>> {
//     let mut out_complex: Vec<Complex<f32>> = vec![];
//     for i in 0..input.len() {
//         out_complex.push(Complex {
//             re: input[i],
//             im: 0.0,
//         });
//     }
//     return out_complex;
// }

pub fn f32_to_complex_mutex(input: &MutexGuard<'_, Vec<f32>>) -> Vec<Complex<f32>> {
    let mut out_complex: Vec<Complex<f32>> = vec![];
    for i in 0..input.len() {
        out_complex.push(Complex {
            re: input[i],
            im: 0.0,
        });
    }
    return out_complex;
}

pub fn amp(x: Complex<f32>) -> f32 {
    if x.re.abs() < x.im.abs() {
        return x.im.abs();
    }
    return x.re.abs();
}

// fn main() {
//     let n: usize = 16;
//     let mut inputs: Vec<Complex<f32>> = vec![];
//     let mut out_complex: Vec<Complex<f32>> = vec![];
//     for i in 0..n {
//         let t = (i as f32) / (n as f32);
//         let sample = (2.0 * PI * t).sin() + (2.0 * PI * t * 2.0).sin() + (2.0 * PI * t * 4.0).cos();
//         inputs.push(Complex {
//             re: sample,
//             im: 0.0,
//         });
//         out_complex.push(Complex { re: 0.0, im: 0.0 });
//     }

//     // for f in 0..n / 2 {
//     //     // out_complex[f] = Complex { re: 0.0, im: 0.0 };
//     //     // out_complex[f + n / 2] = Complex { re: 0.0, im: 0.0 };
//     //     //even
//     //     for i in (0..n).step_by(2) {
//     //         let t = (i as f32) / (n as f32);
//     //         let v = inputs[i] * (Complex::i() * 2.0 * PI * t * (f as f32)).exp();
//     //         out_complex[f] += v;
//     //         out_complex[f + n / 2] += v;
//     //         // println!("{i} {:?} {:?}", v.re, v.im);
//     //     }
//     //     for i in (1..n).step_by(2) {
//     //         let t = (i as f32) / (n as f32);
//     //         let v = inputs[i] * (Complex::i() * 2.0 * PI * t * (f as f32)).exp();
//     //         out_complex[f] += v;
//     //         out_complex[f + n / 2] -= v;
//     //     }
//     // }

//     // for f in 0..n {
//     //     out_sin.push(0.0);
//     //     out_cos.push(0.0);
//     //     out_complex.push(Complex { re: 0.0, im: 0.0 });
//     //     for i in 0..n {
//     //         let t = (i as f32) / (n as f32);
//     //         out_complex[f] += inputs[i] * (Complex::i() * 2.0 * PI * t * (f as f32)).exp();
//     //         out_sin[f] += inputs[i] * (2.0 * PI * t * (f as f32)).sin();
//     //         out_cos[f] += inputs[i] * (2.0 * PI * t * (f as f32)).cos();
//     //     }
//     // }
//     // println!("{:?}", out_sin);
//     // println!("{:?}", out_cos);
//     out_complex = fft(&inputs);
//     for i in 0..out_complex.len() {
//         println!("{:?} {:?}", out_complex[i].re, out_complex[i].im);
//     }
// }
