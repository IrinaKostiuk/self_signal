use fundsp::hacker::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn play_background_audio() -> cpal::Stream {
    // 🔊 Richer ambient base (more tones)
    let tone = (
        sine_hz(220.0)
            + sine_hz(330.0) * 0.2  // harmonic
            + sine_hz(275.0) * 0.1, // a third
        sine_hz(224.0)
            + sine_hz(336.0) * 0.2
            + sine_hz(280.0) * 0.1,
    );

    // 🌬 Breath + gentle noise
    let mut breath = envelope(|t| (-t * 10.0).exp()) * noise() * 0.02;

    // 🎾 Occasional bouncing "ball" tone
    // This one rings in and out
    let mut ball = envelope(|t| (-t * 1.5).exp()) * sine_hz(440.0) * 0.3;

    // ✅ Final stereo synth
    let mut synth = (
        tone.0 * 0.3 + breath.clone() + ball.clone(),
        tone.1 * 0.3 + breath.clone() + ball,
    );
    // set up audio output
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().expect("No output config").into();

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                for frame in data.chunks_mut(2) {
                    let (l, r) = (synth.0.get_mono(), synth.1.get_mono());
                    frame[0] = l as f32;
                    frame[1] = r as f32;
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    // keep stream alive
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    });

    stream
}