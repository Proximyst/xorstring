//! XOR encrypted strings for Rust.
//!
//! These are simply a port of LeFF's C++11 compile-time hacky XOR strings.

#![feature(proc_macro_hygiene)]

use std::num::Wrapping;

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
        let key = xorstring_procmacro::xorstring!();
        let mut decrypted = Vec::with_capacity(self.encrypted.len());

        for (i, c) in self.encrypted.iter().enumerate() {
            decrypted.push(c ^ (Wrapping(key as usize) + Wrapping(i)).0 as u8);
        }

        String::from_utf8(decrypted)
            .expect("XORSTRING error on decrypting String! File an issue at https://github.com/Proximyst/xorstring")
    }
}

/// Create and encrypt a string at compile-time and prepare it for decryption
/// on runtime.
///
/// This must be called with a proper string, such as `"Hello, World!"`.
#[macro_export]
macro_rules! xorstring {
    ($str:literal) => {
        $crate::XorString::new($crate::xorstring_procmacro::xorstring!($str).0).decrypt()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        let xorred = super::xorstring_procmacro::xorstring!("abc").0;
        assert_ne!(xorred, "abc".as_bytes());

        let decrypted = super::XorString::new(xorred);
        let decrypted: String = decrypted.decrypt();

        assert_eq!(decrypted, "abc");
    }

    #[test]
    fn test_macro() {
        assert_eq!(
            super::xorstring!("Hello, World!"),
            String::from("Hello, World!"),
        );
        assert_ne!(
            super::xorstring_procmacro::xorstring!("Hello, World!").0,
            super::xorstring!("Hello, World!").as_bytes(),
        );
        assert_eq!(
            super::xorstring!("👀🥺"),
            String::from("👀🥺"),
        );
    }
}
