#[cfg(test)]
mod tests {

    use mini_functions::uuid::UUID;

    #[test]
    fn test_uuid_v4() {
        let uuid = UUID::uuid_v4();
        assert_eq!(uuid.len(), 36);
    }
    #[test]
    fn test_clone_v4() {
        let uuid = UUID::uuid_v4();
        let uuid_clone = uuid.clone();
        assert_eq!(uuid, uuid_clone);
    }
    #[test]
    fn test_to_string_v4() {
        let uuid = UUID::uuid_v4();
        let uuid_string = uuid.to_string();
        assert_eq!(uuid, uuid_string);
    }
    #[test]
    fn test_default_v4() {
        let uuid_v4 = UUID::uuid_v4();
        let uuid_default = uuid_v4;
        assert_eq!(uuid_default.len(), 36);
    }

    #[test]
    fn test_uuid_v3() {
        let namespace = uuid::Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let name = b"test";
        let uuid = UUID::uuid_v3(namespace, name);

        assert!(uuid::Uuid::parse_str(&uuid).is_ok());
        assert_eq!(
            uuid::Uuid::parse_str(&uuid).unwrap(),
            uuid::Uuid::new_v3(&namespace, name)
        );
    }
    #[test]
    fn test_clone_v3() {
        let uuid = UUID::uuid_v3(
            uuid::Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
            b"hello",
        );
        let uuid_clone = uuid.clone();
        assert_eq!(uuid, uuid_clone);
    }
    #[test]
    fn test_to_string_v3() {
        let uuid = UUID::uuid_v3(
            uuid::Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
            b"hello",
        );
        let uuid_string = uuid.to_string();
        assert_eq!(uuid, uuid_string);
    }
    #[test]
    fn test_default_v3() {
        let uuid_v3 = UUID::uuid_v3(
            uuid::Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
            b"hello",
        );
        let uuid_default = uuid_v3;
        assert_eq!(uuid_default.len(), 36);
    }
}
