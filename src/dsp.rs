use nih_plug::prelude::*;
use std::f32::consts::PI;

/// A non-linear saturation module using 4th-order Chebyshev polynomials.
/// Designed for "tube-like" warmth.
#[derive(Debug, Default, Clone)]
pub struct Saturation {
    sample_rate: f32,
    // Add state for filtering/oversampling here in the future
}

impl Saturation {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate }
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }

    /// Process a block of audio.
    /// 
    /// # Arguments
    /// * `buffer` - The audio buffer to process in-place.
    /// * `drive` - Input drive amount (0.0 to 100.0).
    /// * `output_gain_db` - Output gain in decibels.
    pub fn process_block(&mut self, buffer: &mut Buffer, drive: f32, output_gain_db: f32) {
        // Prepare coefficients outside the loop
        let gain_linear = util::db_to_gain(output_gain_db);
        let drive_norm = 1.0 + (drive * 0.1); // Simple drive scaling

        // SIMD Optimization Note:
        // In a full nightly environment, we would use `std::simd::f32x8` here.
        // For portable logic, we iterate sample by sample or use auto-vectorization friendly loops.
        
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                let x = *sample * drive_norm;

                // 4th Order Chebyshev Approximation for Saturation
                // T0 = 1
                // T1 = x
                // T2 = 2x^2 - 1
                // T3 = 4x^3 - 3x
                // T4 = 8x^4 - 8x^2 + 1
                // We'll use a blend of fundamental and odd harmonics for "tube" sound.
                // Saturation function: f(x) = tanh(x) approximated or similar soft clipper.
                
                // Let's implement a specific characteristic curve:
                // Soft clipping with Chebyshev harmonic injection.
                
                let x_clamped = x.clamp(-1.0, 1.0); // Hard clip input to stable range for polynomial
                
                let t1 = x_clamped;
                let t2 = 2.0 * x_clamped * x_clamped - 1.0;
                let t3 = 4.0 * x_clamped.powi(3) - 3.0 * x_clamped;
                
                // Simple weighted sum for "color"
                // Mostly T1 (fundamental) + some T2 (even harmonics, warmth) + some T3 (odd, grit)
                let saturated = 0.8 * t1 + 0.15 * t2 + 0.05 * t3;

                *sample = saturated * gain_linear;
            }
        }
    }
}
