mod sound;

use anyhow::Result;
use iir_filters::filter::DirectForm2Transposed;
use iir_filters::filter::Filter;
use iir_filters::filter_design::butter;
use iir_filters::filter_design::FilterType;
use iir_filters::sos::zpk2sos;
use rand::prelude::*;
use rasp::delay::LinearDelay;
use rasp::traits::Processor;

const DELAY_SAMPLE_RATE: f32 = 48000.0;
const DELAY_MAX_SECONDS: usize = 3;
const DELAY_SAMPLES: f32 = 1300.5;
const I16_MAX: f32 = (i16::MAX as f32) + 1.0;
const I16_MAX_64: f64 = (i16::MAX as f64) + 1.0;
fn init_delay() -> LinearDelay<f32> {
    LinearDelay::new(
        DELAY_SAMPLES,
        DELAY_MAX_SECONDS * DELAY_SAMPLE_RATE as usize,
    )
}

pub fn process_delay(input: &mut [i16], delay: &mut LinearDelay<f32>) {
    for sample in input.iter_mut() {
        *sample = (delay.process(*sample as f32 / I16_MAX) * I16_MAX) as i16;
    }
}

const CUTOFF_FREQUENCY: f64 = 0.5;
const SAMPLE_FREQUENCY: f64 = 8000.0;

pub fn init_lowpass() -> Result<DirectForm2Transposed> {
    let order = 5;
    let zpk = butter(
        order,
        FilterType::LowPass(CUTOFF_FREQUENCY),
        SAMPLE_FREQUENCY,
    )?;
    let sos = zpk2sos(&zpk, None)?;

    Ok(DirectForm2Transposed::new(&sos))
}

pub fn process_lowpass(input: &mut [i16], dft2: &mut DirectForm2Transposed) {
    for sample in input.iter_mut() {
        *sample = (dft2.filter(*sample as f64 / I16_MAX_64) * I16_MAX_64) as i16;
    }
}

const BAND_PASS_LOW: f64 = 50.0;
const BAND_PASS_HIGH: f64 = 2600.0;

pub fn init_band_pass() -> Result<DirectForm2Transposed> {
    let order = 5;
    let zpk = butter(
        order,
        FilterType::BandPass(BAND_PASS_LOW, BAND_PASS_HIGH),
        SAMPLE_FREQUENCY,
    )?;
    let sos = zpk2sos(&zpk, None)?;

    Ok(DirectForm2Transposed::new(&sos))
}

const HIGH_PASS_FREQ: f64 = 2000.0;

pub fn init_high_pass() -> Result<DirectForm2Transposed> {
    let order = 5;
    let zpk = butter(
        order,
        FilterType::HighPass(HIGH_PASS_FREQ),
        SAMPLE_FREQUENCY,
    )?;
    let sos = zpk2sos(&zpk, None)?;

    Ok(DirectForm2Transposed::new(&sos))
}

const FUDGE: f32 = 10.0;

pub fn process_radio(input: &mut [i16], vol_follow: &mut f32) {
    let mut samples: Vec<f32> = input
        .iter()
        .map(|x| *x as f32 / I16_MAX)
        .collect::<Vec<_>>();

    let frame_count = input.len();
    let mut vol: f32 = 0.0;
    for i in 0..frame_count {
        vol += samples[i] * samples[i];
    }
    vol /= frame_count as f32;

    // Fudge factor, increase for more noise
    vol *= FUDGE;

    // Smooth follow from last frame, both multiplies add up to 1...
    *vol_follow = *vol_follow * 0.5 + vol * 0.5;

    // Between -1.0 and 1.0...
    let mut random = (rand::thread_rng().gen::<u16>() % 32768) as f32 / 16384.0 - 1.0;

    // Between 1 and 128...
    let mut count = (rand::thread_rng().gen::<u8>() % 128) + 1;
    let mut temp: f32;
    for i in 0..frame_count {
        if count == 0 {
            // Between -1.0 and 1.0...
            random = (rand::thread_rng().gen::<u16>() % 32768) as f32 / 16384.0 - 1.0;
            // Between 1 and 128...
            count = (rand::thread_rng().gen::<u8>() % 128) + 1;
        }
        // Add random to inputs multiplied by current volume;
        temp = samples[i] + random * *vol_follow;

        // Make it an integer between -60 and 60
        temp = (temp * 40.0) as i32 as f32;

        // Drop it back down but massively quantized and too high
        temp = temp / 40.0;
        temp *= 0.05 * FUDGE;
        temp += samples[i] * (1.0 - (0.05 * FUDGE));

        samples[i] = temp.clamp(-1.0, 1.0);
    }

    for (it, sample) in samples.iter().enumerate() {
        input[it] = (*sample * I16_MAX) as i16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_filter() {
        let mut delay = init_delay();

        let mut input = [2000_i16; 100];

        process_delay(&mut input, &mut delay);

        assert_ne!(input, [2000_i16; 100]);
    }

    #[test]
    fn test_lowpass_filter() {
        let mut lowpass = init_lowpass().unwrap();

        let mut input = [2000_i16; 100];

        process_lowpass(&mut input, &mut lowpass);

        assert_ne!(input, [2000_i16; 100]);
    }
}
