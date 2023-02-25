use crate::wav;
use fon::chan::{Ch16, Ch32};
use fon::{Audio, Frame};
use twang::ops::Gain;
use twang::osc::Sine;
use twang::osc::Triangle;
use twang::Synth;

struct Processors {
    triangle: Triangle,
}

pub fn generate_triangle_wave() {
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 1); // 1 second
    let proc = Processors {
        triangle: Triangle::new(),
    };

    Synth::new(proc, |proc, frame: Frame<_, 2>| {
        let triangle = proc.triangle.step(440.0);
        //let sin = Sine::new();

        frame.pan(Gain.step(triangle, Ch32::new(1.0 / 12.0)), 0.0)
    })
    .stream(audio.sink());
    wav::write(audio, "triangle.wav").expect("Failed to write WAV file");
}
