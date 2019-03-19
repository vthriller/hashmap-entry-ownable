#![feature(hash_raw_entry)]
use std::borrow::Borrow;
use std::collections::hash_map::{
	HashMap,
	RawEntryMut,
};
use std::hash::{Hash, BuildHasher};

pub trait EntryAPI<K, Q, V, S>
where
	K: Borrow<Q>,
	Q: ToOwned<Owned = K>,
{
	fn entry_ownable<'a, 'q>(&'a mut self, key: &'q Q) -> Entry<'a, 'q, K, Q, V, S>;
}

impl<K, Q, V, S> EntryAPI<K, Q, V, S> for HashMap<K, V, S>
where
	K: Borrow<Q> + Hash + Eq,
	Q: ToOwned<Owned = K> + Hash + Eq,
	S: BuildHasher
{
	fn entry_ownable<'a, 'q>(&'a mut self, key: &'q Q) -> Entry<'a, 'q, K, Q, V, S> {
		Entry {
			key,
			raw: self.raw_entry_mut().from_key(key),
		}
	}
}

pub struct Entry<'a, 'q, K, Q, V, S>
where
	K: Borrow<Q>,
	Q: ToOwned<Owned = K>,
{
	key: &'q Q,
	raw: RawEntryMut<'a, K, V, S>,
}

impl<'a, 'q, K, Q, V, S> Entry<'a, 'q, K, Q, V, S>
where
	K: Borrow<Q> + Hash,
	Q: ToOwned<Owned = K>,
	S: BuildHasher
{
	pub fn or_insert(self, default: V) -> &'a mut V {
		match self.raw {
			RawEntryMut::Occupied(e) =>
				e.into_mut(),
			RawEntryMut::Vacant(e) =>
				e.insert(self.key.to_owned(), default).1,
		}
	}
}
