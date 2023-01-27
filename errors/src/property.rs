#[derive(Clone, Debug, Ord, PartialEq, PartialOrd, Hash)]
pub struct Property {
    pub label: String,
    pub printable: bool,
}

impl Eq for Property {}

impl Property {
    pub fn new(label: &str, printable: bool) -> Property {
        Property {
            label: label.to_owned(),
            printable,
        }
    }
}
