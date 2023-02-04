#[derive(Clone, Debug, Ord, PartialEq, PartialOrd, Hash)]
/// The property of the error.
pub struct Property {
    /// The label of the property.
    pub label: String,
    /// Whether the property is printable.
    pub printable: bool,
}

impl Eq for Property {}

impl Property {
    /// Creates a new property.
    pub fn new(label: &str, printable: bool) -> Property {
        Property {
            label: label.to_owned(),
            printable,
        }
    }
}
