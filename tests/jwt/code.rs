#[cfg(test)]

mod tests {
    extern crate mini_functions;

    use mini_functions::jwt::code;

    #[test]
    fn test_code() {
        let err = code::SIGNATURE_INVALID;
        println!("Error: {}", err);
        assert_eq!(err, code::SIGNATURE_INVALID);
    }
}
