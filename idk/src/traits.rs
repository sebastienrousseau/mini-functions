use std::sync::atomic::AtomicU64;

static INTERNAL_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// The `Trait` struct with id as a unique identifier and label as a
/// string representation of the trait.
pub struct Trait {
    /// The `id` field holds the unique identifier of the trait.
    pub id: u64,
    /// The `label` field holds the string representation of the trait.
    pub label: String,
}

impl Trait {
    /// Creates a new instance of Trait with a unique identifier and the
    /// given label string.
    pub fn new(label: &str) -> Self {
        Trait {
            id: INTERNAL_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            label: label.to_string(),
        }
    }
    /// A convenient alias to new().
    pub fn register_trait(label: &str) -> Trait {
        Trait::new(label)
    }
}
/// A struct to manage a collection of Trait instances.
pub struct TraitBuilder {
    /// A vector of Trait instances
    pub traits: Vec<Trait>,
}

impl TraitBuilder {
    /// Creates a new instance of TraitBuilder.
    pub fn new() -> Self {
        TraitBuilder { traits: Vec::new() }
    }
    /// Adds a new Trait instance with the given label to the collection.
    pub fn add_trait(&mut self, label: &str) -> &Trait {
        let trait_ = Trait::new(label);
        self.traits.push(trait_);
        self.traits.last().unwrap()
    }
    /// Finds and returns the Trait instance with the given label.
    pub fn get_trait(&self, label: &str) -> Option<&Trait> {
        self.traits.iter().find(|t| t.label == label)
    }
    /// Deletes the Trait instance with the given label from the
    /// collection.
    pub fn delete_trait(&mut self, label: &str) -> bool {
        let pos = self.traits.iter().position(|t| t.label == label);
        if let Some(pos) = pos {
            self.traits.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for TraitBuilder {
    /// A default implementation of TraitBuilder, creating a new
    /// instance of TraitBuilder.
    fn default() -> Self {
        Self::new()
    }
}
/// A struct to represent the presence of a Trait instance.
pub struct HasTrait {
    /// The identifier of the Trait instance.
    pub trait_id: u64,
    /// Whether the Trait instance is present.
    pub has_trait: bool,
    /// Whether the Trait instance was added.
    pub add_trait: bool,
}
impl HasTrait {
    /// Creates a new instance of HasTrait with the given trait_id.
    pub fn new(trait_id: u64) -> Self {
        HasTrait {
            trait_id,
            has_trait: false,
            add_trait: false,
        }
    }
    /// Returns true if the HasTrait instance has the given Trait key.
    pub fn has_trait(&self, key: &Trait) -> bool {
        self.trait_id == key.id
    }
    /// Adds the given Trait key to the HasTrait instance.
    pub fn add_trait(&mut self, key: &Trait) {
        self.trait_id = key.id;
        self.has_trait = true;
        self.add_trait = true;
    }
    /// Removes the given Trait key from the HasTrait instance.
    pub fn remove_trait(&mut self, key: &Trait) {
        self.trait_id = key.id;
        self.has_trait = false;
        self.add_trait = false;
    }
    /// Returns true if the Trait instance was added to the HasTrait
    /// instance.
    pub fn is_add_trait(&self) -> bool {
        self.add_trait
    }
    /// Returns true if the HasTrait instance has the Trait instance.
    pub fn is_has_trait(&self) -> bool {
        self.has_trait
    }
    /// Returns true if the Trait instance was removed from the HasTrait
    /// instance.
    pub fn is_remove_trait(&self) -> bool {
        !self.add_trait
    }
}
