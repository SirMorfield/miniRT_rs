use pad::{Alignment, PadStr};
use std::os::raw::c_char;

pub fn vector_to_cstring_vector(vector: Vec<String>) -> *mut *mut c_char {
    let mut cstring_vector: Vec<std::ffi::CString> = Vec::new();
    for string in vector {
        cstring_vector.push(std::ffi::CString::new(string).unwrap());
    }
    let mut cstring_vector_ptr: Vec<*mut c_char> = Vec::new();
    for cstring in cstring_vector {
        cstring_vector_ptr.push(cstring.into_raw());
    }
    let cstring_vector_ptr_ptr: *mut *mut c_char = cstring_vector_ptr.as_mut_ptr();
    std::mem::forget(cstring_vector_ptr);
    cstring_vector_ptr_ptr
}

pub trait AsFormattedString {
    fn as_formatted_str(&self) -> String;
}

impl AsFormattedString for std::time::Duration {
    fn as_formatted_str(&self) -> String {
        let mut result = String::new();
        result.reserve(22); // "00h 00m 04.668 668 668".len() == 22

        let seconds = self.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = std::time::Duration::from_secs(seconds % 60);

        result.push_str(&hours.to_string().pad(2, '0', Alignment::Right, false));
        result.push_str("h ");
        result.push_str(&minutes.to_string().pad(2, '0', Alignment::Right, true));
        result.push_str("m ");
        result.push_str(
            &seconds
                .as_secs()
                .to_string()
                .pad(2, '0', Alignment::Right, true),
        );
        result.push_str(".");

        result.push_str(
            &self
                .subsec_millis()
                .to_string()
                .pad(3, '0', Alignment::Right, true),
        );
        result.push_str(" ");
        result.push_str(
            &self
                .subsec_micros()
                .to_string()
                .pad(3, '0', Alignment::Right, true),
        );
        result.push_str(" ");
        result.push_str(
            &self
                .subsec_nanos()
                .to_string()
                .pad(3, '0', Alignment::Right, true),
        );
        result.push_str("s");

        return result;
    }
}
