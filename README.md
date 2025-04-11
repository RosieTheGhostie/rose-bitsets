<style>
  .math {
    font-size: larger;
  }
</style>

Small, fixed-size bitsets for storing integers/indices.

Provides up to six bitset types, one for each primitive unsigned integer. These types are:

|           Type | Underlying Type | Feature | Enabled by Default? |
| -------------: | :-------------: | :-----: | :-----------------: |
|    [`BitSet8`] |     [`u8`]      |   b8    |     **&check;**     |
|   [`BitSet16`] |     [`u16`]     |   b16   |     **&check;**     |
|   [`BitSet32`] |     [`u32`]     |   b32   |     **&check;**     |
|   [`BitSet64`] |     [`u64`]     |   b64   |     **&check;**     |
|  [`BitSet128`] |    [`u128`]     |  b128   |     **&cross;**     |
| [`BitSetSize`] |    [`usize`]    |  bsize  |     **&cross;**     |

# Operations

All the following operations are designed to be...

- **fast:** 𝒪(1) time complexity
- **memory-efficient:** 𝒪(1) space complexity
- **intuitive:** similar interface to [`std::collections::HashSet`]
- **`const`-friendly:** usable inside `const` contexts[^1]
- **safe:** no `unsafe` code

## The Fundamentals

The following operators are fundamental enough to
[set theory] that they warrant operator overloads.

<table>
  <tr>
    <th>Math</th>
    <th>Method Call</th>
    <th>Overloaded Operators</th>
  </tr>
  <tr>
    <td class="math">𝐴<sup>𝑐</sup></td>
    <td><code>a.complement()</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/ops/trait.Neg.html#tymethod.neg">Neg::neg</a></code>
      (<code>-a</code>)
      <br/>
      <code><a href="https://doc.rust-lang.org/core/ops/trait.Not.html#tymethod.not">Not::not</a></code>
      (<code>!a</code>)
    </td>
  </tr>
  <tr>
    <td class="math">𝐴 &cap; 𝐵</td>
    <td><code>a.intersection(b)</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/ops/trait.BitAnd.html#tymethod.bitand">BitAnd::bitand</a></code>
      (<code>a & b</code>)
    </td>
  </tr>
  <tr>
    <td class="math">𝐴 &cup; 𝐵</td>
    <td><code>a.union(b)</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/ops/trait.BitOr.html#tymethod.bitor">BitOr::bitor</a></code>
      (<code>a | b</code>)
    </td>
  </tr>
  <tr>
    <td class="math">𝐴 &setminus; 𝐵</td>
    <td><code>a.difference(b)</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/ops/trait.Sub.html#tymethod.sub">Sub::sub</a></code>
      (<code>a - b</code>)
    </td>
  </tr>
  <tr>
    <td class="math">𝐴 &Delta; 𝐵</td>
    <td><code>a.symmetric_difference(b)</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/ops/trait.BitXor.html#tymethod.bitxor">BitXor::bitxor</a></code>
      (<code>a ^ b</code>)
    </td>
  </tr>
  <tr>
    <td class="math">𝐴 = 𝐵</td>
    <td><code>a.is(b)</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/cmp/trait.PartialEq.html#tymethod.eq">PartialEq::eq</a></code>
      (<code>a == b</code>)
    </td>
  </tr>
  <tr>
    <td class="math">𝐴 &ne; 𝐵</td>
    <td><code>a.is_not(b)</code></td>
    <td>
      <code><a href="https://doc.rust-lang.org/core/cmp/trait.PartialEq.html#tymethod.ne">PartialEq::ne</a></code>
      (<code>a != b</code>)
    </td>
  </tr>
</table>

## Comparisons and Bitset Metadata

Bitsets support a variety of comparison operators. Though they aren't similar enough to methods in
[`core::cmp::PartialOrd`] to warrant operator overloads, they are still very useful tools for
working with sets.

Metadata-like methods (e.g., `len`) are lumped in with the comparisons because it is sometimes hard
to draw a line between them.

<table>
  <tr>
    <th>Math</th>
    <th>Method Calls</th>
  </tr>
  <tr>
    <td class="math">𝐴 &cap; 𝐵 = &empty;</td>
    <td><code>a.is_disjoint(b)</code></td>
  </tr>
  <tr>
    <td class="math">𝐴 &subseteq; 𝐵</td>
    <td><code>a.is_subset(b)</code></td>
  </tr>
  <tr>
    <td class="math">𝐴 &subset; 𝐵</td>
    <td><code>a.is_strict_subset(b)</code></td>
  </tr>
  <tr>
    <td class="math">𝐴 &supseteq; 𝐵</td>
    <td><code>a.is_superset(b)</code></td>
  </tr>
  <tr>
    <td class="math">𝐴 &supset; 𝐵</td>
    <td><code>a.is_strict_superset(b)</code></td>
  </tr>
  <tr>
    <td class="math">𝐴 = &empty;</td>
    <td><code>a.is_empty()</code></td>
  </tr>
  <tr>
    <td class="math">𝐴 = 𝑈</td>
    <td><code>a.is_full()</code></td>
  </tr>
  <tr>
    <td class="math">|𝐴|</td>
    <td><code>a.len()</code></td>
  </tr>
  <tr>
    <td class="math">𝑥 &in; 𝐴</td>
    <td><code>a.contains(x)</code></td>
  </tr>
  <tr>
    <td class="math">min(𝐴)</td>
    <td><code>a.min_index()</code></td>
  </tr>
  <tr>
    <td class="math">max(𝐴)</td>
    <td>
      <code>a.max_index()</code>
      <br/>
      <code>a.max_index_checked()</code>
    </td>
  </tr>
</table>

## Miscellaneous

These don't have a direct connection to [set theory], but they are nice to have when working with
bitsets.

<table>
  <tr>
    <th>Math</th>
    <th>Method Calls</th>
  </tr>
  <tr>
    <td class="math">{ 𝑥 ∈ 𝐴 | 𝑥 < 𝑖 }</td>
    <td>
      <code>a.masked_0_to_i(i)</code>
      <br/>
      <code>a.cleared_i_to_N(i)</code>
      <sup id="fnref2"><a href="#fn2">2</a></sup>
    </td>
  </tr>
  <tr>
    <td class="math">{ 𝑥 ∈ 𝐴 | 𝑥 ≥ 𝑖 }</td>
    <td>
      <code>a.masked_i_to_N(i)</code>
      <sup id="fnref2"><a href="#fn2">2</a></sup>
      <br/>
      <code>a.cleared_0_to_i(i)</code>
    </td>
  </tr>
</table>

## Modification Methods

Because bitsets are meant to act like sets, they share many methods with
[`std::collections::HashSet`]. Some have been added as well for those who like to aggressively
optimize their code.

- `clear`
- `clear_0_to_i`
- `clear_i_to_N`[^2]
- `mask_0_to_i`
- `mask_i_to_N`[^2]
- `insert`
- `insert_quiet`
- `replace`
- `replace_quiet`
- `remove`
- `remove_quiet`

# Iteration

Each bitset also comes with two kinds of iterators:

- `BitSetIndices`: Iterates over the **indices** of the **enabled** bits.
- `BitSetIter`: Iterates over the **values** of **all** bits.

Both iterators can be used to traverse a set in either direction[^3]. For example, the following
code would iterate over the indices in ascending order:

```rust
use rose_bitsets::{Ascending, BitSet8};

let set = BitSet8::from_bits(0b00101110);
let mut indices = set.iter_indices::<Ascending>();

assert_eq!(indices.next(), Some(1));
assert_eq!(indices.next(), Some(2));
assert_eq!(indices.next(), Some(3));
assert_eq!(indices.next(), Some(5));
assert_eq!(indices.next(), None);
```

[^1]: Because operator overloading is achieved via traits, it isn't currently possible to use the
overloads inside `const` contexts.
[^2]: The `N` is a placeholder for the set's capacity (e.g., `16` for a `BitSet16`).
[^3]: By direction, I mean whether the significance increases or decreases as the iteration
progresses. The `Ascending` mode iterates starting from the least significant end and works towards
the most significant, whereas the `Descending` mode iterates starting from the most
significant end and works towards the least significant.

[`u8`]: https://doc.rust-lang.org/core/primitive.u8.html
[`u16`]: https://doc.rust-lang.org/core/primitive.u16.html
[`u32`]: https://doc.rust-lang.org/core/primitive.u32.html
[`u64`]: https://doc.rust-lang.org/core/primitive.u64.html
[`u128`]: https://doc.rust-lang.org/core/primitive.u128.html
[`usize`]: https://doc.rust-lang.org/core/primitive.usize.html
[`BitSet8`]: https://docs.rs/rose-bitsets/latest/rose_bitsets/struct.BitSet8.html
[`BitSet16`]: https://docs.rs/rose-bitsets/latest/rose_bitsets/struct.BitSet16.html
[`BitSet32`]: https://docs.rs/rose-bitsets/latest/rose_bitsets/struct.BitSet32.html
[`BitSet64`]: https://docs.rs/rose-bitsets/latest/rose_bitsets/struct.BitSet64.html
[`BitSet128`]: https://docs.rs/rose-bitsets/latest/rose_bitsets/struct.BitSet128.html
[`BitSetSize`]: https://docs.rs/rose-bitsets/latest/rose_bitsets/struct.BitSetSize.html
[`core::cmp::PartialOrd`]: https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html
[`std::collections::HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[set theory]: https://en.wikipedia.org/wiki/Set_(mathematics)
