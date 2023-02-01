#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod constants;
pub mod words;
pub use {constants::*, words::*};
pub struct Common;

impl Common {
    pub fn new() -> Self {
        Common
    }
    pub fn constants(&self) -> Constants {
        Constants
    }
    pub fn words(&self) -> Words {
        Words::new()
    }
}

impl Default for Common {
    fn default() -> Self {
        Self::new()
    }
}
