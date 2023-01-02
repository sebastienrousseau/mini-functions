#[cfg(test)]
mod tests {
    use mini_functions::uuid::UUID;
    #[test]
    fn test_new() {
        let version_3 = 3;
        let version_4 = 4;
        let version_5 = 5;
        let ns = &uuid::Uuid::new_v4();
        let name = "test";
        let uuid_v3 = UUID::new(version_3, ns, name);
        let uuid_v4 = UUID::new(version_4, ns, name);
        let uuid_v5 = UUID::new(version_5, ns, name);
        assert_eq!(uuid_v3.to_string().len(), 36);
        assert_eq!(uuid_v4.to_string().len(), 36);
        assert_eq!(uuid_v5.to_string().len(), 36);
        assert_eq!(version_3, 3);
        assert_eq!(version_4, 4);
        assert_eq!(version_5, 5);
        assert_eq!(name, "test");

        let version_4 = 4;
        let uuid = UUID::uuid_v4();
        assert_eq!(uuid.to_string().len(), 36);
        assert_eq!(version_4, 4);
    }

    #[test]
    fn test_uuid_v3() {
        let uuid = UUID::uuid_v3(&uuid::Uuid::new_v4(), "test");
        assert_eq!(uuid.to_string().len(), 36);
    }
    #[test]
    fn test_uuid_v4() {
        let uuid = UUID::uuid_v4();
        assert_eq!(uuid.to_string().len(), 36);
    }
    #[test]
    fn test_uuid_v5() {
        let uuid = UUID::uuid_v5(&uuid::Uuid::new_v4(), "test");
        assert_eq!(uuid.to_string().len(), 36);
    }
}
