#![feature(hash_raw_entry)]
#![feature(test)]

pub mod std_hash;
#[cfg(feature = "hashbrown")]
pub mod hashbrown;
