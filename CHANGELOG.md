# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Unit tests for...
  - `BitSetN::complement` (on top of the ones that already existed)
  - `BitSetN::difference`
  - `BitSetN::symmetric_difference`
  - `BitSetN::is`
  - `BitSetN::is_not`
  - `BitSetN::is_disjoint`
  - `<BitSetN as Debug>::fmt`
- Eight shift methods for `BitSetN`
  - `shifted_up_by`
  - `shift_up_by`
  - `shifted_up_by_signed`
  - `shift_up_by_signed`
  - `shifted_down_by`
  - `shift_down_by`
  - `shifted_down_by_signed`
  - `shift_down_by_signed`
- Numeric formatting implementations
  - `<BitSetN as Binary>::fmt`
  - `<BitSetN as LowerHex>::fmt`
  - `<BitSetN as Octal>::fmt`
  - `<BitSetN as UpperHex>::fmt`

## [0.1.0-beta] - 2025-04-11

### Added

- Unit tests for iterating over `BitSet8`, `BitSet16`, `BitSet32`,
  and `BitSet64` in either direction
- A bunch of badges at top of README

### Fixed

- `BitSetIndicesN<'_, Ascending>` now works correctly
- `<style>` block no longer visible on [GitHub](https://github.com/RosieTheGhostie/rose-bitsets) or
  [crates.io](https://crates.io/crates/rose-bitsets/) (I deleted it)
- [docs.rs](https://docs.rs/rose-bitsets/latest/rose_bitsets/) now shows feature flag notices

## [0.1.0-alpha] - 2025-04-11

### Added

- README with a rough overview of the API
- Six bitset `struct`s
	- `BitSet8`
	- `BitSet16`
	- `BitSet32`
	- `BitSet64`
	- `BitSet128`
	- `BitSetSize`
- Iterators over the indices of a bitset
	- `BitSetIndices8`
	- `BitSetIndices16`
	- `BitSetIndices32`
	- `BitSetIndices64`
	- `BitSetIndices128`
	- `BitSetIndicesSize`
- Iterators over the bits of a bitset
	- `BitSetIter8`
	- `BitSetIter16`
	- `BitSetIter32`
	- `BitSetIter64`
	- `BitSetIter128`
	- `BitSetIterSize`
- Two unit `struct`s for specifying iteration order
	- `Ascending`
	- `Descending`
- A handful of unit tests
- Automated testing with GitHub Actions

[unreleased]: https://github.com/RosieTheGhostie/rose-bitsets/compare/v0.1.0-beta...HEAD
[0.1.0-beta]: https://github.com/RosieTheGhostie/rose-bitsets/compare/v0.1.0-alpha...v0.1.0-beta
[0.1.0-alpha]: https://github.com/RosieTheGhostie/rose-bitsets/releases/tag/v0.1.0-alpha
