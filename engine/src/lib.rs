pub fn limless_lib_logm(a: i32) -> String {
    println!("LOG: limless: lib: logm(): {}", a);
    a.to_string()
}

// tests
#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn it_works() {
    //     let result = limless_lib_logm(13);
    //     assert_eq!(result, String::from("13"));
    // }
}
