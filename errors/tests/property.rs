#[cfg(test)]

mod tests {
    extern crate errors;
    use errors::property::Property;

    #[test]
    fn test_property_error() {
        let err = Property::new("test", true);
        assert_eq!(err.label, "test");
        assert!(err.printable);
    }
}
