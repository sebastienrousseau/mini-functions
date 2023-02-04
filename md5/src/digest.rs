use crate::{INITIAL_STATE, MD5};
use std::{fs::File, io::Read};

/// The `Digest` trait.
pub trait Digest {
    /// reset the internal state of the object
    fn reset(&mut self) -> &mut Self;
    /// update the internal state of the object with new data
    fn update(&mut self, value: &[u8]) -> &mut Self;
    /// update the internal state of the object with new data from a file
    fn update_file(&mut self, path: &str) -> &mut Self;
    /// return the digest value as a string of hexadecimal digits
    fn hexdigest(value: &str) -> String;
    /// return the digest value as a string of hexadecimal digits from a file
    fn hexdigest_file(path: &str) -> String;
    /// reset the internal state of the object and update it with new data from a file
    fn reset_file(&mut self, path: &str) -> &mut Self;
}

impl Digest for MD5 {
    /// Reset the internal state of the MD5 object.
    fn reset(&mut self) -> &mut Self {
        self.state = INITIAL_STATE;
        self.count.fill(0);
        self.buffer.fill(0);
        self.digest.fill(0);

        self
    }
    /// Update the internal state of the MD5 object with new data.
    fn update(&mut self, value: &[u8]) -> &mut Self {
        self.update_with_len(value, value.len())
    }
    /// Update the internal state of the MD5 object with new data from a file.
    fn update_file(&mut self, path: &str) -> &mut Self {
        let mut file = File::open(path).expect("Couldn't open file");
        let mut buffer = [0; 1024];

        loop {
            let nbytes = file.read(&mut buffer).expect("Couldn't read file");
            if nbytes == 0 {
                break;
            }
            self.update_with_len(&buffer, nbytes);
        }

        self
    }
    /// Return the digest value as a string of hexadecimal digits.
    fn hexdigest(value: &str) -> String {
        Self::new().update(value.as_bytes()).finalize().to_string()
    }

    /// Return the digest value as a string of hexadecimal digits from a file.
    fn hexdigest_file(path: &str) -> String {
        let mut file = File::open(path).expect("Couldn't open file");
        let mut buffer = [0; 1024];
        let mut md5 = Self::new();

        loop {
            let nbytes = file.read(&mut buffer).expect("Couldn't read file");
            if nbytes == 0 {
                break;
            }
            md5.update_with_len(&buffer, nbytes);
        }

        md5.finalize().to_string()
    }
    /// Reset the internal state of the MD5 object and update it with new data from a file.
    fn reset_file(&mut self, path: &str) -> &mut Self {
        self.reset();
        self.update_file(path)
    }
}
