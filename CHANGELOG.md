# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/RosieTheGhostie/rose-bitsets/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/RosieTheGhostie/rose-bitsets/releases/tag/v0.1.0-alpha
