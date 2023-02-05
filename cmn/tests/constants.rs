#[cfg(test)]
mod tests {
    extern crate cmn;
    pub use cmn::Constants;

    #[test]
    fn test_constant() {
        let constants = Constants.constants();
        for constant in constants {
            println!("Name: {} Value: {}", constant.name, constant.value);
        }
    }
}
