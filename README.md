# hashmap-entry-ownable

Variation of `HashMap::entry()` that accepts borrowed forms of keys.

## Compatibility

This crate requires nightly version of Rust (see [`hash_raw_entry`](https://github.com/rust-lang/rust/issues/56167)).

`entry_ownable()` can be used as a drop-in replacement for `entry()`, *unless*:

- `Entry::key()` is used,
- entry is matched as a enum (`Entry::Occupied`/`Entry::Vacant`).

## Description

Use this crate if you create/update a lot of entries from borrowed forms of keys (e.g. from `&str` instead of `String`):

```rust
use std::collections::HashMap;
use hashmap_entry_ownable::std_hash::EntryAPI;

let rhyme = vec![
	"Mary", "had", "a", "little", "lamb",
	"little", "lamb", "little", "lamb",
];

let mut words: HashMap<String, _> = HashMap::new();

for w in rhyme {
	// words.entry() accepts String but not &str
	// this version saves us 4 short-living allocations
	// for consecutive appearances of "little" and "lamb"
	let counter = words.entry_ownable(w).or_insert(0);
	*counter += 1;
}
```

This code can be emulated with `match map.get_mut(k)`/`map.insert(k.to_owned(), …)`.
That too would be faster than regular Entry API if keys are reused a lot,
but `entry_ownable()` variant:

- is as concise as regular `entry()` one,
- only hashes key once, while `get_mut()`/`insert()` does it twice if key is not in the map.

## [hashbrown](https://github.com/Amanieu/hashbrown/) support

`hashmap_entry_ownable::hashbrown::EntryAPI` can be enabled through crate feature aptly called `hashbrown`.

## Benchmarks

`std::collections::HashMap`:

```
test silly_bench::entry_1         ... bench:   2,162,452 ns/iter (+/- 17,586)
test silly_bench::get_or_insert_1 ... bench:   2,310,910 ns/iter (+/- 7,183)
test silly_bench::entry_ownable_1 ... bench:   1,586,380 ns/iter (+/- 7,156)

test silly_bench::entry_2         ... bench:   3,427,428 ns/iter (+/- 104,755)
test silly_bench::get_or_insert_2 ... bench:   2,926,719 ns/iter (+/- 11,077)
test silly_bench::entry_ownable_2 ... bench:   2,167,021 ns/iter (+/- 8,140)

test silly_bench::entry_4         ... bench:   6,068,954 ns/iter (+/- 56,549)
test silly_bench::get_or_insert_4 ... bench:   4,150,970 ns/iter (+/- 18,292)
test silly_bench::entry_ownable_4 ... bench:   3,321,019 ns/iter (+/- 18,487)

test silly_bench::entry_8         ... bench:  11,278,344 ns/iter (+/- 82,014)
test silly_bench::get_or_insert_8 ... bench:   6,602,424 ns/iter (+/- 33,360)
test silly_bench::entry_ownable_8 ... bench:   5,631,416 ns/iter (+/- 54,613)
```

`hashbrown::HashMap`:

```
test silly_bench::entry_1         ... bench:   1,469,848 ns/iter (+/- 9,229)
test silly_bench::get_or_insert_1 ... bench:   1,489,309 ns/iter (+/- 10,865)
test silly_bench::entry_ownable_1 ... bench:   1,370,580 ns/iter (+/- 5,152)

test silly_bench::entry_2         ... bench:   2,351,940 ns/iter (+/- 10,374)
test silly_bench::get_or_insert_2 ... bench:   1,801,549 ns/iter (+/- 12,360)
test silly_bench::entry_ownable_2 ... bench:   1,634,317 ns/iter (+/- 7,421)

test silly_bench::entry_4         ... bench:   4,418,648 ns/iter (+/- 18,059)
test silly_bench::get_or_insert_4 ... bench:   2,451,798 ns/iter (+/- 10,702)
test silly_bench::entry_ownable_4 ... bench:   2,292,556 ns/iter (+/- 16,042)

test silly_bench::entry_8         ... bench:   8,350,365 ns/iter (+/- 39,678)
test silly_bench::get_or_insert_8 ... bench:   3,837,979 ns/iter (+/- 20,885)
test silly_bench::entry_ownable_8 ... bench:   3,749,296 ns/iter (+/- 5,678)
```

## License

This crate borrows heavily from Rust's own libstd, hence [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT).
