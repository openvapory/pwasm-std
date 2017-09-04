use alloc::string::ToString;
use alloc::string::String;
use core::{ops, fmt, cmp, str};
use core::cmp::{min, Ordering};
use core::ops::{Deref, DerefMut, BitXor, BitAnd, BitOr, IndexMut, Index};
use core::hash::{Hash, Hasher};
use bigint::U256;
use hex::*;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[allow(non_camel_case_types)]
extern {
    fn memcmp(cx: *const c_void, ct: *const c_void, n: usize) -> i32;
}

/// Return `s` without the `0x` at the beginning of it, if any.
fn clean_0x(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

macro_rules! impl_hash {
    ($from: ident, $size: expr) => {
        #[repr(C)]
        /// Unformatted binary data of fixed length.
        pub struct $from (pub [u8; $size]);


        impl From<[u8; $size]> for $from {
            fn from(bytes: [u8; $size]) -> Self {
                $from(bytes)
            }
        }

        impl From<$from> for [u8; $size] {
            fn from(s: $from) -> Self {
                s.0
            }
        }

        impl Deref for $from {
            type Target = [u8];

            #[inline]
            fn deref(&self) -> &[u8] {
                &self.0
            }
        }

        impl AsRef<[u8]> for $from {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl DerefMut for $from {
            #[inline]
            fn deref_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        impl $from {
            /// Create a new, zero-initialised, instance.
            pub fn new() -> $from {
                $from([0; $size])
            }

            /// Synonym for `new()`. Prefer to new as it's more readable.
            pub fn zero() -> $from {
                $from([0; $size])
            }

            /// Get the size of this object in bytes.
            pub fn len() -> usize {
                $size
            }

            #[inline]
            /// Assign self to be of the same value as a slice of bytes of length `len()`.
            pub fn clone_from_slice(&mut self, src: &[u8]) -> usize {
                let min = cmp::min($size, src.len());
                self.0[..min].copy_from_slice(&src[..min]);
                min
            }

            /// Convert a slice of bytes of length `len()` to an instance of this type.
            pub fn from_slice(src: &[u8]) -> Self {
                let mut r = Self::new();
                r.clone_from_slice(src);
                r
            }

            /// Copy the data of this object into some mutable slice of length `len()`.
            pub fn copy_to(&self, dest: &mut[u8]) {
                let min = cmp::min($size, dest.len());
                dest[..min].copy_from_slice(&self.0[..min]);
            }

            /// Returns `true` if all bits set in `b` are also set in `self`.
            pub fn contains<'a>(&'a self, b: &'a Self) -> bool {
                &(b & self) == b
            }

            /// Returns `true` if no bits are set.
            pub fn is_zero(&self) -> bool {
                self.eq(&Self::new())
            }

            /// Returns the lowest 8 bytes interpreted as a BigEndian integer.
            pub fn low_u64(&self) -> u64 {
                let mut ret = 0u64;
                for i in 0..min($size, 8) {
                    ret |= (self.0[$size - 1 - i] as u64) << (i * 8);
                }
                ret
            }
        }

        impl str::FromStr for $from {
            type Err = FromHexError;

            fn from_str(s: &str) -> Result<$from, FromHexError> {
                let a = s.from_hex()?;
                if a.len() != $size {
                    return Err(FromHexError::InvalidHexLength);
                }

                let mut ret = [0;$size];
                ret.copy_from_slice(&a);
                Ok($from(ret))
            }
        }

        impl fmt::Debug for $from {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for i in &self.0[..] {
                    write!(f, "{:02x}", i)?;
                }
                Ok(())
            }
        }

        impl fmt::Display for $from {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for i in &self.0[..] {
                    write!(f, "{:02x}", i)?;
                }
                Ok(())
            }
        }

        impl Copy for $from {}

        impl Clone for $from {
            fn clone(&self) -> $from {
                let mut ret = $from::new();
                ret.0.copy_from_slice(&self.0);
                ret
            }
        }

        impl Eq for $from {}

        impl PartialEq for $from {
            fn eq(&self, other: &Self) -> bool {
                unsafe { memcmp(self.0.as_ptr() as *const c_void, other.0.as_ptr() as *const c_void, $size) == 0 }
            }
        }

        impl Ord for $from {
            fn cmp(&self, other: &Self) -> Ordering {
                let r = unsafe { memcmp(self.0.as_ptr() as *const c_void, other.0.as_ptr() as *const c_void, $size) };
                if r < 0 { return Ordering::Less }
                if r > 0 { return Ordering::Greater }
                return Ordering::Equal;
            }
        }

        impl PartialOrd for $from {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Hash for $from {
            fn hash<H>(&self, state: &mut H) where H: Hasher {
                state.write(&self.0);
                state.finish();
            }
        }

        impl Index<usize> for $from {
            type Output = u8;

            fn index(&self, index: usize) -> &u8 {
                &self.0[index]
            }
        }
        impl IndexMut<usize> for $from {
            fn index_mut(&mut self, index: usize) -> &mut u8 {
                &mut self.0[index]
            }
        }
        impl Index<ops::Range<usize>> for $from {
            type Output = [u8];

            fn index(&self, index: ops::Range<usize>) -> &[u8] {
                &self.0[index]
            }
        }
        impl IndexMut<ops::Range<usize>> for $from {
            fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [u8] {
                &mut self.0[index]
            }
        }
        impl Index<ops::RangeFull> for $from {
            type Output = [u8];

            fn index(&self, _index: ops::RangeFull) -> &[u8] {
                &self.0
            }
        }
        impl IndexMut<ops::RangeFull> for $from {
            fn index_mut(&mut self, _index: ops::RangeFull) -> &mut [u8] {
                &mut self.0
            }
        }

        /// `BitOr` on references
        impl<'a> BitOr for &'a $from {
            type Output = $from;

            fn bitor(self, rhs: Self) -> Self::Output {
                let mut ret: $from = $from::default();
                for i in 0..$size {
                    ret.0[i] = self.0[i] | rhs.0[i];
                }
                ret
            }
        }

        /// Moving `BitOr`
        impl BitOr for $from {
            type Output = $from;

            fn bitor(self, rhs: Self) -> Self::Output {
                &self | &rhs
            }
        }

        /// `BitAnd` on references
        impl <'a> BitAnd for &'a $from {
            type Output = $from;

            fn bitand(self, rhs: Self) -> Self::Output {
                let mut ret: $from = $from::default();
                for i in 0..$size {
                    ret.0[i] = self.0[i] & rhs.0[i];
                }
                ret
            }
        }

        /// Moving `BitAnd`
        impl BitAnd for $from {
            type Output = $from;

            fn bitand(self, rhs: Self) -> Self::Output {
                &self & &rhs
            }
        }

        /// `BitXor` on references
        impl <'a> BitXor for &'a $from {
            type Output = $from;

            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut ret: $from = $from::default();
                for i in 0..$size {
                    ret.0[i] = self.0[i] ^ rhs.0[i];
                }
                ret
            }
        }

        /// Moving `BitXor`
        impl BitXor for $from {
            type Output = $from;

            fn bitxor(self, rhs: Self) -> Self::Output {
                &self ^ &rhs
            }
        }

        impl $from {
            /// Get a hex representation.
            pub fn hex(&self) -> String {
                format!("{:?}", self)
            }
        }

        impl Default for $from {
            fn default() -> Self { $from::new() }
        }

        impl From<u64> for $from {
            fn from(mut value: u64) -> $from {
                let mut ret = $from::new();
                for i in 0..8 {
                    if i < $size {
                        ret.0[$size - i - 1] = (value & 0xff) as u8;
                        value >>= 8;
                    }
                }
                ret
            }
        }

        impl From<&'static str> for $from {
            fn from(s: &'static str) -> $from {
                let s = clean_0x(s);
                if s.len() % 2 == 1 {
                    ("0".to_string() + s).parse().unwrap()
                } else {
                    s.parse().unwrap()
                }
            }
        }

        impl<'a> From<&'a [u8]> for $from {
            fn from(s: &'a [u8]) -> $from {
                $from::from_slice(s)
            }
        }
    }
}

impl From<U256> for H256 {
    fn from(value: U256) -> H256 {
        let mut ret = H256::new();
        value.to_big_endian(&mut ret);
        ret
    }
}

impl<'a> From<&'a U256> for H256 {
    fn from(value: &'a U256) -> H256 {
        let mut ret: H256 = H256::new();
        value.to_big_endian(&mut ret);
        ret
    }
}

impl From<H256> for U256 {
    fn from(value: H256) -> U256 {
        U256::from(&value)
    }
}

impl<'a> From<&'a H256> for U256 {
    fn from(value: &'a H256) -> U256 {
        U256::from(value.as_ref() as &[u8])
    }
}

impl From<H256> for H160 {
    fn from(value: H256) -> H160 {
        let mut ret = H160::new();
        ret.0.copy_from_slice(&value[12..32]);
        ret
    }
}

impl From<H256> for H64 {
    fn from(value: H256) -> H64 {
        let mut ret = H64::new();
        ret.0.copy_from_slice(&value[20..28]);
        ret
    }
}

impl From<H160> for H256 {
    fn from(value: H160) -> H256 {
        let mut ret = H256::new();
        ret.0[12..32].copy_from_slice(&value);
        ret
    }
}

impl<'a> From<&'a H160> for H256 {
    fn from(value: &'a H160) -> H256 {
        let mut ret = H256::new();
        ret.0[12..32].copy_from_slice(value);
        ret
    }
}

impl_hash!(H32, 4);
impl_hash!(H64, 8);
impl_hash!(H128, 16);
impl_hash!(H160, 20);
impl_hash!(H256, 32);
impl_hash!(H264, 33);
impl_hash!(H512, 64);
impl_hash!(H520, 65);
impl_hash!(H1024, 128);
impl_hash!(H2048, 256);
