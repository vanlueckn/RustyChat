use aych_delay::{Delay, Settings};
use biquad::*;
pub fn process_echo(input: &[f32], output: &mut [f32]) {
    let mut delay = Delay::new(Settings {
        delay_time: 166.66,
        feedback: 0.75,
        width: 0.5,
        lowpass_filter: 22000.0,
        highpass_filter: 300.0,
        dry_wet_mix: 0.5,
        output_level: 0.75,
        ..Settings::default()
    });

    delay.process(input, output);
}

pub fn process_lowpass(input: &mut [f32]) {
    // Cutoff and sampling frequencies
    let f0 = 10.hz();
    let fs = 1.khz();

    // Create coefficients for the biquads
    let coeffs =
        Coefficients::<f32>::from_params(Type::LowPass, fs, f0, Q_BUTTERWORTH_F32).unwrap();

    // Create two different biquads
    let mut biquad1 = DirectForm1::<f32>::new(coeffs);

    let input_cloned = input.to_vec();

    // Run for all the inputs
    for (index, t_elem) in input_cloned.iter().enumerate() {
        input[index] = biquad1.run(*t_elem);
    }
}
