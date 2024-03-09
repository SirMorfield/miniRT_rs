use pad::{Alignment, PadStr};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Precision {
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}
pub trait AsFormattedString {
    fn as_formatted_str(&self, precision: Precision) -> String;
}

impl AsFormattedString for std::time::Duration {
    fn as_formatted_str(&self, precision: Precision) -> String {
        let mut result = String::new();
        result.reserve(22); // "00h 00m 04.668 668 668".len() == 22

        let seconds = self.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = std::time::Duration::from_secs(seconds % 60);

        if precision >= Precision::Hours {
            if hours > 0 {
                result.push_str(&hours.to_string().pad(2, ' ', Alignment::Right, false));
                result.push_str("h");
            } else {
                result.push_str("   ");
            }
        }

        if precision >= Precision::Minutes {
            if minutes > 0 {
                result.push_str(" ");
                result.push_str(&minutes.to_string().pad(2, ' ', Alignment::Right, true));
                result.push_str("m");
            } else {
                result.push_str("    ");
            }
        }

        if precision >= Precision::Seconds {
            result.push_str(" ");
            result.push_str(&seconds.as_secs().to_string().pad(2, '0', Alignment::Right, true));
        }

        if precision >= Precision::Milliseconds {
            result.push_str(".");
            result.push_str(&self.subsec_millis().to_string().pad(3, '0', Alignment::Right, true));
        }

        if precision >= Precision::Microseconds {
            result.push_str(" ");
            result.push_str(&self.subsec_micros().to_string().pad(3, '0', Alignment::Right, true));
        }

        if precision >= Precision::Nanoseconds {
            result.push_str(" ");
            result.push_str(&self.subsec_nanos().to_string().pad(3, '0', Alignment::Right, true));
        }

        if precision >= Precision::Seconds {
            result.push_str("s");
        }

        return result;
    }
}

pub fn contains_duplicates<T>(arr: &[T]) -> bool
where
    T: PartialEq,
{
    for (i, a) in arr.iter().enumerate() {
        for b in arr.iter().skip(i + 1) {
            if a == b {
                return true;
            }
        }
    }
    return false;
}
