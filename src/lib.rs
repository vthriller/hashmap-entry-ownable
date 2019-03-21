#![cfg_attr(feature = "nightly", feature(hash_raw_entry))]
#![cfg_attr(feature = "nightly", feature(test))]

/*
Yes, the following mods are copy-pasted.
There is no common crate that we can depend on. We can write one for ourselves with thin-wrapping `impl`, but is it worth it?
And don't even tell me about macros. I've been here, it falls apart when there's need to `impl` for both `Foo<A, B>` and `Bar<A, B, C>` (note the extra type argument).
*/

#[cfg(feature = "nightly")]
pub mod std_hash;
#[cfg(feature = "hashbrown")]
pub mod hashbrown;
