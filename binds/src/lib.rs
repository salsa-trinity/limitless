use std::ffi::*;

#[unsafe(no_mangle)]
pub extern "C" fn __limless_lib_logm_ffi(a: c_int) -> *const c_char {
    let res = CString::new(engine::limless_lib_logm(a)).unwrap();
    res.as_ptr()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = ;
//         assert_eq!(result, 4);
//     }
// }
