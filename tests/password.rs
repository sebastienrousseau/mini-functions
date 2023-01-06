#[cfg(test)]
mod tests {
    use mini_functions::password::Password;

    #[test]
    fn test_password() {
        let password = Password::new(3, "-");
        println!("{}", password);
    }
    #[test]
    fn test_passphrase() {
        let password = Password::new(3, "-");
        println!("{}", password.passphrase());
    }
    #[test]
    fn test_set_passphrase() {
        let mut password = Password::new(3, "-");
        password.set_passphrase("test");
        println!("{}", password.passphrase());
    }
    #[test]
    fn test_len() {
        let password = Password::new(3, "-");
        println!("{}", password.len());
    }
    #[test]
    fn test_is_empty() {
        let password = "";
        println!("{}", password.is_empty());
    }
}
