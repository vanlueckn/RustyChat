use libdsp_sys::root::DSP::DigitalDelay;

struct DelayState {
    delay: Option<DigitalDelay>,
}

static mut DELAY_STATE: DelayState = DelayState { delay: None };

const DELAY_AMOUNT: usize = 20;
const DELAY_FEEDBACK: f32 = 0.5;
const DELAY_WET: f32 = 0.5;

pub fn init_delay() {
    unsafe {
        let mut delay: DigitalDelay = DigitalDelay::new(DELAY_AMOUNT, DELAY_FEEDBACK, DELAY_WET);
        DELAY_STATE.delay = Some(delay);
    }
}

pub fn process_delay(input: &mut [f32]) {
    unsafe {
        if DELAY_STATE.delay.is_none() {
            init_delay();
        }
        let mut delay = DELAY_STATE.delay.unwrap();

        for (i, &sample) in input.to_vec().iter().enumerate() {
            input[i] = delay.getNextSample(sample);
        }
    }
}

pub fn process_lowpass(input: &mut [f32], alpha: f32) {
    let mut prev_output = 0.0;

    for (i, &sample) in input.to_vec().iter().enumerate() {
        let filtered_sample = alpha * sample + (1.0 - alpha) * prev_output;
        input[i] = filtered_sample;
        prev_output = filtered_sample;
    }
}
