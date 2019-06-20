//! XOR encrypted strings for Rust.
//!
//! These are simply a port of LeFF's C++11 compile-time hacky XOR strings.

#![feature(proc_macro_hygiene)]

/// The XOR string proc-macro used under the hood.
///
/// This is available in the case that a raw byte array of the encrypted string
/// is preferred over an auto-decrypted one.
pub use xorstring_procmacro;

/// A string which has been encrypted on compile-time and is now to be decrypted
/// on runtime.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct XorString<'a> {
    /// The encrypted string as raw bytes.
    encrypted: &'a [u8],
}

impl<'a> XorString<'a> {
    /// Creates a new instance of a XOR string.
    ///
    /// This must not have a runtime calculated value, because that would defeat
    /// the entire point of this.
    pub const fn new(encrypted: &'a [u8]) -> Self {
        XorString { encrypted }
    }

    /// Decrypt the XorString into a proper string.
    ///
    /// This is done at runtime such that this crate has any use.
    pub fn decrypt(&self) -> String {
        let mut string = String::with_capacity(self.encrypted.len());
        let key = xorstring_procmacro::xorstring!();

        for (i, c) in self.encrypted.iter().enumerate() {
            string.push((c ^ (key as usize + i) as u8) as char);
        }

        string
    }
}

/// Create and encrypt a string at compile-time and prepare it for decryption
/// on runtime.
///
/// This must be called with a byte-string, such as `b"Hello, World!"`.
#[macro_export]
macro_rules! xorstring {
    ($str:literal) => {
        $crate::XorString::new($crate::xorstring_procmacro::xorstring!($str)).decrypt()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        let xorred = super::xorstring_procmacro::xorstring!(b"abc");
        assert_ne!(xorred, b"abc");

        let decrypted = super::XorString::new(xorred);
        let decrypted: String = decrypted.decrypt();
        let decrypted = decrypted.as_bytes();

        assert_eq!(decrypted, b"abc");
    }

    #[test]
    fn test_macro() {
        assert_eq!(
            super::xorstring!(b"Hello, World!"),
            String::from("Hello, World!"),
        );
        assert_ne!(
            super::xorstring_procmacro::xorstring!(b"Hello, World!"),
            super::xorstring!(b"Hello, World!").as_bytes(),
        );
    }
}
