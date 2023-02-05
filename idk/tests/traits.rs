#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate idk;
    use idk::traits::{HasTrait, Trait, TraitBuilder};

    #[test]
    fn test_trait_new() {
        let trait_test = Trait::new("test");
        assert_eq!(trait_test.label, "test");
    }

    #[test]
    fn register_trait() {
        let trait_test = Trait::new("test");
        assert_eq!(trait_test.label, "test");
        Trait::register_trait("test");
    }

    #[test]
    fn test_traits_builder() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        trait_builder.delete_trait("test");
        assert_eq!(trait_builder.traits.len(), 0);
    }

    #[test]
    fn test_trait_new_builder() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
    }

    #[test]
    fn test_get_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
    }

    #[test]
    fn test_has_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
        let has_trait = HasTrait::new(trait_test.unwrap().id);
        assert!(has_trait.has_trait(trait_test.unwrap()));
    }
    #[test]
    fn test_add_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
        let mut has_trait = HasTrait::new(trait_test.unwrap().id);
        assert!(has_trait.has_trait(trait_test.unwrap()));
        has_trait.add_trait(trait_test.unwrap());
        assert!(has_trait.add_trait);
    }
    #[test]
    fn test_remove_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
        let mut has_trait = HasTrait::new(trait_test.unwrap().id);
        assert!(has_trait.has_trait(trait_test.unwrap()));
        has_trait.remove_trait(trait_test.unwrap());
    }
    #[test]
    fn test_is_add_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
        let mut has_trait = HasTrait::new(trait_test.unwrap().id);
        assert!(has_trait.has_trait(trait_test.unwrap()));
        has_trait.add_trait(trait_test.unwrap());
        assert!(has_trait.is_add_trait());
    }
    #[test]
    fn test_is_has_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
        let mut has_trait = HasTrait::new(trait_test.unwrap().id);
        assert!(has_trait.has_trait(trait_test.unwrap()));
        has_trait.add_trait(trait_test.unwrap());
        assert!(has_trait.is_has_trait());
    }
    #[test]
    fn is_remove_trait() {
        let mut trait_builder = TraitBuilder::new();
        trait_builder.add_trait("test");
        assert_eq!(trait_builder.traits[0].label, "test");
        let trait_test = trait_builder.get_trait("test");
        assert_eq!(trait_test.unwrap().label, "test");
        let mut has_trait = HasTrait::new(trait_test.unwrap().id);
        assert!(has_trait.has_trait(trait_test.unwrap()));
        has_trait.remove_trait(trait_test.unwrap());
        assert!(has_trait.is_remove_trait());
    }
}
