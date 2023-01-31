use std::sync::atomic::AtomicU64;

static INTERNAL_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Trait {
    pub id: u64,
    pub label: String,
}

impl Trait {
    pub fn new(label: &str) -> Self {
        Trait {
            id: INTERNAL_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            label: label.to_string(),
        }
    }

    pub fn register_trait(label: &str) -> Trait {
        Trait::new(label)
    }
}

pub struct TraitBuilder {
    pub traits: Vec<Trait>,
}

impl TraitBuilder {
    pub fn new() -> Self {
        TraitBuilder { traits: Vec::new() }
    }

    pub fn add_trait(&mut self, label: &str) -> &Trait {
        let trait_ = Trait::new(label);
        self.traits.push(trait_);
        self.traits.last().unwrap()
    }

    pub fn get_trait(&self, label: &str) -> Option<&Trait> {
        self.traits.iter().find(|t| t.label == label)
    }

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
    fn default() -> Self {
        Self::new()
    }
}

pub struct HasTrait {
    pub trait_id: u64,
    pub has_trait: bool,
    pub add_trait: bool,
}
impl HasTrait {
    pub fn new(trait_id: u64) -> Self {
        HasTrait {
            trait_id,
            has_trait: false,
            add_trait: false,
        }
    }

    pub fn has_trait(&self, key: &Trait) -> bool {
        self.trait_id == key.id
    }

    pub fn add_trait(&mut self, key: &Trait) {
        self.trait_id = key.id;
        self.has_trait = true;
        self.add_trait = true;
    }

    pub fn remove_trait(&mut self, key: &Trait) {
        self.trait_id = key.id;
        self.has_trait = false;
        self.add_trait = false;
    }

    pub fn is_add_trait(&self) -> bool {
        self.add_trait
    }

    pub fn is_has_trait(&self) -> bool {
        self.has_trait
    }

    pub fn is_remove_trait(&self) -> bool {
        !self.add_trait
    }
}
