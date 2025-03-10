pub mod rot13;
pub mod rsa;

use std::error::Error;

pub trait Cipher {
    fn original_string(&self) -> Result<String, Box<dyn Error>>;
    fn encrypted_string(&self) -> Result<String, Box<dyn Error>>;
}
