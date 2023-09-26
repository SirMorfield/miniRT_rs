use crate::helpers::AsFormattedString;
use crate::helpers::Precision;
use crate::num::{Float0to1, PositiveNonzeroF32, ToFixed};
use pad::{Alignment, PadStr};
use std::io::Write;

pub struct ProgressLogger {
    label: String,
    start_time: std::time::Instant,
    precision: PositiveNonzeroF32,
    mantissas: usize,

    last_progress: Float0to1,
}

impl ProgressLogger {
    /// precision is the minimum difference between two progress logs in percentage,
    ///
    /// so 0.1 will only log  0.1%, 0.2%, 0.3% etc.
    pub fn new(label: &str, precision: PositiveNonzeroF32, mantissas: usize) -> Self {
        Self {
            label: label.to_string(),
            start_time: std::time::Instant::now(),
            precision: PositiveNonzeroF32::new(precision.get() / 100.0).unwrap(),
            mantissas,
            last_progress: Float0to1::new(0.0).unwrap(),
        }
    }
    pub fn log(&mut self, progress: Float0to1) {
        if self.last_progress < progress
            && progress.get() - self.last_progress.get() < self.precision.get()
        {
            return;
        }
        self.log_ignore_precision(progress);
    }

    pub fn log_ignore_precision(&mut self, progress: Float0to1) {
        self.last_progress = progress;

        let elapsed = self.start_time.elapsed();
        let elapsed = elapsed.as_secs() as f32 + elapsed.subsec_millis() as f32 / 1000.0;

        let remaining = elapsed / progress.get() - elapsed;
        let remaining = std::time::Duration::from_secs_f32(remaining);
        let remaining = remaining.as_formatted_str(Precision::Milliseconds);

        let elapsed = std::time::Duration::from_secs_f32(elapsed);
        let elapsed = elapsed.as_formatted_str(Precision::Milliseconds);

        let progress = (progress.get() * 100.0)
            .to_fixed(self.mantissas)
            .to_string()
            .pad(
                3 + self.mantissas + (self.mantissas != 0) as usize,
                ' ',
                Alignment::Right,
                false,
            );
        print!(
            "\r{}: {}% ({} remaining, {} elapsed)",
            self.label, progress, remaining, elapsed,
        );
        std::io::stdout().flush().unwrap();
    }
    pub fn log_end(&mut self) {
        self.log_ignore_precision(Float0to1::new(1.0).unwrap());
        print!("\n");
    }
}
