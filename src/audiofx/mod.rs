use libdsp_sys::root::DSP::{BiQuadFilter, DigitalDelay};

const DELAY_AMOUNT: usize = 50;
const DELAY_FEEDBACK: f32 = 0.7;
const DELAY_WET: f32 = 0.7;
const I16_MAX: f32 = (i16::MAX as f32) + 1.0;

fn init_delay() -> DigitalDelay {
    unsafe { DigitalDelay::new(DELAY_AMOUNT, DELAY_FEEDBACK, DELAY_WET) }
}

pub fn process_delay(input: &mut [i16], delay: &mut DigitalDelay) {
    unsafe {
        for sample in input.iter_mut() {
            *sample = (delay.getNextSample(*sample as f32 / I16_MAX) * I16_MAX) as i16;
        }
    }
}

const CUTOFF_FREQUENCY: f32 = 1000.0;
const SAMPLE_FREQUENCY: f64 = 48000.0;

fn init_lowpass() -> BiQuadFilter {
    unsafe { BiQuadFilter::new(1, CUTOFF_FREQUENCY as i32, SAMPLE_FREQUENCY) }
}

pub fn process_lowpass(input: &mut [i16], lowpass: &mut BiQuadFilter) {
    unsafe {
        for sample in input.iter_mut() {
            *sample = (lowpass.nextSample(*sample as f32 / I16_MAX, 1) * I16_MAX) as i16;
        }
    }
}
