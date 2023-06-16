use aych_delay::{Delay, Settings};

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
