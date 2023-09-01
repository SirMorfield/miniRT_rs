use std::os::raw::c_char;
// use std::os::raw::c_int;

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
