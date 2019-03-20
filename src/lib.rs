#![feature(hash_raw_entry)]
#![feature(test)]

use std::borrow::Borrow;
use hashbrown::hash_map::{
	HashMap,
	RawEntryMut,
};
use std::hash::{Hash, Hasher, BuildHasher};

pub trait EntryAPI<K, Q, V, S>
where
	K: Borrow<Q>,
	Q: ToOwned<Owned = K> + ?Sized,
{
	/**
	Gets the given key's corresponding entry in the map for in-place manipulation, creating
	copy of the key if necessary.

	The key may be any borrowed form of the map's key type, but `Hash` and `Eq` on the
	borrowed form *must* match those for the key type.

	# Examples

	```
	use hashbrown::HashMap;
	use hashmap_entry_ownable::EntryAPI;

	let mut words: HashMap<String, _> = HashMap::new();

	let rhyme = vec![
		"Mary", "had", "a", "little", "lamb",
		"little", "lamb", "little", "lamb",
	];

	for w in rhyme {
		let counter = words.entry_ownable(w).or_insert(0);
		*counter += 1;
	}

	assert_eq!(words["Mary"], 1);
	assert_eq!(words["lamb"], 3);
	assert_eq!(words.get("fleece"), None);
	```
	*/
	fn entry_ownable<'a, 'q>(&'a mut self, key: &'q Q) -> Entry<'a, 'q, K, Q, V, S>;
}

impl<K, Q, V, S> EntryAPI<K, Q, V, S> for HashMap<K, V, S>
where
	K: Borrow<Q> + Hash + Eq,
	Q: ToOwned<Owned = K> + Hash + Eq + ?Sized,
	S: BuildHasher
{
	#[inline]
	fn entry_ownable<'a, 'q>(&'a mut self, key: &'q Q) -> Entry<'a, 'q, K, Q, V, S> {
		let mut hasher = self.hasher().build_hasher();
		key.hash(&mut hasher);
		let hash = hasher.finish();
		Entry {
			key,
			hash,
			raw: self.raw_entry_mut().from_key_hashed_nocheck(hash, key),
		}
	}
}

pub struct Entry<'a, 'q, K, Q, V, S>
where
	K: Borrow<Q>,
	Q: ToOwned<Owned = K> + ?Sized
{
	key: &'q Q,
	hash: u64,
	raw: RawEntryMut<'a, K, V, S>,
}

impl<'a, 'q, K, Q, V, S> Entry<'a, 'q, K, Q, V, S>
where
	K: Borrow<Q> + Hash,
	Q: ToOwned<Owned = K> + ?Sized,
	S: BuildHasher
{
	/**
	Ensures a value is in the entry by inserting the default if empty, and returns
	a mutable reference to the value in the entry.

	# Examples

	```
	use hashbrown::HashMap;
	use hashmap_entry_ownable::EntryAPI;

	let mut map: HashMap<String, u32> = HashMap::new();

	map.entry_ownable("poneyland").or_insert(3);
	assert_eq!(map["poneyland"], 3);

	*map.entry_ownable("poneyland").or_insert(10) *= 2;
	assert_eq!(map["poneyland"], 6);
	```
	*/
	#[inline]
	pub fn or_insert(self, default: V) -> &'a mut V {
		match self.raw {
			RawEntryMut::Occupied(e) =>
				e.into_mut(),
			RawEntryMut::Vacant(e) =>
				e.insert_hashed_nocheck(self.hash, self.key.to_owned(), default).1,
		}
	}

	/**
	Ensures a value is in the entry by inserting the result of the default function if empty,
	and returns a mutable reference to the value in the entry.

	# Examples

	```
	use hashbrown::HashMap;
	use hashmap_entry_ownable::EntryAPI;

	let mut map: HashMap<String, String> = HashMap::new();
	let s = "hoho".to_string();

	map.entry_ownable("poneyland").or_insert_with(|| s);

	assert_eq!(map["poneyland"], "hoho".to_string());
	```
	*/
	#[inline]
	pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
		match self.raw {
			RawEntryMut::Occupied(e) =>
				e.into_mut(),
			RawEntryMut::Vacant(e) =>
				e.insert_hashed_nocheck(self.hash, self.key.to_owned(), default()).1,
		}
	}

	/**
	Provides in-place mutable access to an occupied entry before any
	potential inserts into the map.

	# Examples

	```
	use hashbrown::HashMap;
	use hashmap_entry_ownable::EntryAPI;

	let mut map: HashMap<String, u32> = HashMap::new();

	map.entry_ownable("poneyland")
		.and_modify(|e| { *e += 1 })
		.or_insert(42);
	assert_eq!(map["poneyland"], 42);

	map.entry_ownable("poneyland")
		.and_modify(|e| { *e += 1 })
		.or_insert(42);
	assert_eq!(map["poneyland"], 43);
	```
	*/
	#[inline]
	pub fn and_modify<F>(mut self, f: F) -> Self
		where F: FnOnce(&mut V)
	{
		match self.raw {
			RawEntryMut::Occupied(ref mut e) =>
				f(e.get_mut()),
			RawEntryMut::Vacant(_) =>
				(),
		}
		self
	}
}

impl<'a, 'q, K, Q, V, S> Entry<'a, 'q, K, Q, V, S>
where
	K: Borrow<Q> + Hash,
	Q: ToOwned<Owned = K> + ?Sized,
	V: Default,
	S: BuildHasher
{
	/**
	Ensures a value is in the entry by inserting the default value if empty,
	and returns a mutable reference to the value in the entry.

	# Examples

	```
	use hashbrown::HashMap;
	use hashmap_entry_ownable::EntryAPI;

	let mut map: HashMap<String, Option<u32>> = HashMap::new();
	map.entry_ownable("poneyland").or_default();

	assert_eq!(map["poneyland"], None);
	```
	*/
	#[inline]
	pub fn or_default(self) -> &'a mut V {
		match self.raw {
			RawEntryMut::Occupied(e) =>
				e.into_mut(),
			RawEntryMut::Vacant(e) =>
				e.insert_hashed_nocheck(self.hash, self.key.to_owned(), Default::default()).1,
		}
	}
}

#[cfg(test)]
mod silly_bench {
	extern crate test;
	use test::Bencher;

	use hashbrown::HashMap;
	use super::EntryAPI;

	fn data() -> Vec<String> {
		(1..8192)
			.map(|n| format!("{:013b}", n))
			.collect()
	}

	fn entry(b: &mut Bencher, n: usize) {
		let data = data();
		let data: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
		b.iter(|| {
			let mut map: HashMap<String, _> = HashMap::with_capacity(data.len() * 2);
			for _ in 0..n {
				for &i in &data {
					let counter = map.entry(i.to_string()).or_insert(0);
					*counter += 1;
				}
			}
		})
	}

	#[bench] fn entry_1(b: &mut Bencher) { entry(b, 1) }
	#[bench] fn entry_2(b: &mut Bencher) { entry(b, 2) }
	#[bench] fn entry_4(b: &mut Bencher) { entry(b, 4) }
	#[bench] fn entry_8(b: &mut Bencher) { entry(b, 8) }

	fn entry_ownable(b: &mut Bencher, n: usize) {
		let data = data();
		let data: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
		b.iter(|| {
			let mut map: HashMap<String, _> = HashMap::with_capacity(data.len() * 2);
			for _ in 0..n {
				for &i in &data {
					let counter = map.entry_ownable(i).or_insert(0);
					*counter += 1;
				}
			}
		})
	}

	#[bench] fn entry_ownable_1(b: &mut Bencher) { entry_ownable(b, 1) }
	#[bench] fn entry_ownable_2(b: &mut Bencher) { entry_ownable(b, 2) }
	#[bench] fn entry_ownable_4(b: &mut Bencher) { entry_ownable(b, 4) }
	#[bench] fn entry_ownable_8(b: &mut Bencher) { entry_ownable(b, 8) }

	fn get_or_insert(b: &mut Bencher, n: usize) {
		let data = data();
		let data: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
		b.iter(|| {
			let mut map: HashMap<String, _> = HashMap::with_capacity(data.len() * 2);
			for _ in 0..n {
				for &i in &data {
					match map.get_mut(i) {
						Some(v) => { *v += 1; },
						None => { map.insert(i.to_string(), 1); },
					}
				}
			}
		})
	}

	#[bench] fn get_or_insert_1(b: &mut Bencher) { get_or_insert(b, 1) }
	#[bench] fn get_or_insert_2(b: &mut Bencher) { get_or_insert(b, 2) }
	#[bench] fn get_or_insert_4(b: &mut Bencher) { get_or_insert(b, 4) }
	#[bench] fn get_or_insert_8(b: &mut Bencher) { get_or_insert(b, 8) }
}
