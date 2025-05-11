#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// A data structure for representing a Swon document without any span information.
pub mod tree;

/// Identifier type and parser.
pub mod identifier;

/// A type-safe data-type of SWON data-model.
pub mod value;

#[cfg(feature = "std")]
use ahash::AHashMap as Map;
#[cfg(not(feature = "std"))]
type Map<K, V> = alloc::collections::BTreeMap<K, V>;
