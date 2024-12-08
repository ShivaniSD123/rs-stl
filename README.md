# rs-stl

Porting STL to rust.

Rust has great set of algorithms based on `rust iterators`. However, many
algorithms like `std::rotate` is still missing. This is because we don't have
concept of `C++-like iterators`.

C++ STL is a brilliant piece of work by Alex Stepanov and provides highly
composable algorithms over C++ iterators.

C++ iterators are generalization over pointers. However, pointers would be too
unsafe for using as API in rust. Instead, rs-stl work over `Positions`. Positions
are generalization over indexes.

## rs-stl model

rs-stl defines concept of ranges. A range can be of following types:

- **InputRange**: Models single pass ranges.
- **ForwardRange**: Models multi-pass ranges. Automatically an input range.
- **BidirectionalRange**: Models range that also supports backward iteration. Automatically a forward range.
- **RandomAccessRange**: Models range that supports random access iteration. Automatically a bidirectional range.
- **OutputRange**: Models mutable range. Automatically an input range.

```rust
pub trait InputRange {
    type Element;
    type Position: SemiRegular;
    fn start(&self) -> Self::Position;
    fn end(&self) -> Self::Position;
    fn after(&self, i: Self::Position) -> Self::Position;
    fn at(&self, i: &Self::Position) -> &Self::Element;
}
```

Every range has 2 typedefs namely `Element` and `Position`.

`let i = rng.start()` gives start position/index of range. This is analogous to
`auto itr = rng.begin()` of C++ STL.

Similarily `let end = rng.end()` gives end position/index of range. This is analogous
to `auto end = rng.end()` of C++ STL.

`[start, end)` position defines possible set of valid positions in range. As
STL convention, `start` is position of first element and `end` is position past to last element.

`++itr` for iterator is replaced with `i = rng.after(i)`.
And `*itr` for iterator is replaced with `rng.at(i)` to access ith element.

This way positions are not the owner of elements and have not borrowed from
original range. Ownership of range still lies with range only and range would
be necessary to access the element at any time.

This is important because
with iterator model, iterators used to borrow from ranges and becasue we need
2 iterators for most of algorithms, that would not be possible.

And surprisingly, this is just enough for porting most of STL algorihtms to rust.

### Sample usage

```rust
use stl::*;
use rng::infix::*;

let vec = vec![1, 2, 3];
let cnt = vec.count_if(|x| x % 2 == 1);
// Or use `algo` or `rng` namespace functions:
// let cnt = algo::count_if(&vec, vec.start(), vec.end(), |x| x % 2 == 1);
// let cnt = rng::count_if(&vec, |x| x % 2 == 1);
assert_eq!(cnt, 2);
```

Similarily, stl defines ForwardRange which just mandates Position to be `Regular`.

```rust
pub trait ForwardRange: InputRange<Position: Regular> {}
```

BidirectionalRange defines a before function in addition to after function that
returns position before current position.

```rust
pub trait BidirectionalRange: ForwardRange
{
    fn before(&self, i: Self::Position) -> Self::Position;
}
```

RandomAccessRange mandates `Position` type to be `Ord` as well as supports
function to iterate n steps forward or backward.

```rust
pub trait RandomAccessRange: BidirectionalRange<Position: Regular + Ord>
{
    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position;
    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position;
}
```

And at last, to support mutation, OutputRange defines at_mut function on `InputRange`.

```rust
pub trait OutputRange: InputRange {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}
```

### Support for standard library

Currently `Vec<T>` implements `RandomAccessRange` and `OutputRange`. In future,
we plan to support more containers and if not possible for any container, provide
alernative for the same.

## Algorithms Implemented / Planned to Implement

The names mentioned here are C++ standard library names. If any algorithm is
implemented with other name as I would feel that name would be more suitable
then, that name would be mentioned explicitly in front of C++ name as well as
required description would be provided.

<details>

<summary>Algorithms List</summary>

### Search Operations

- [x] find_if
- [x] find_if_not
- [x] find
- [x] count_if
- [x] count
- [x] all_of
- [x] any_of
- [x] none_of
- [x] mismatch -> mismatch_unbounded, mismatch_unbounded_by, mismatch, mismatch_by
- [ ] find_end
- [x] adjacent_find -> adjacent_find_if
- [x] equal -> equals, equals_by, equals_unbounded, equals_unbounded_by
- [ ] search
- [ ] search_n

### Copy Operations

- [x] copy
- [x] copy_if
- [x] copy_n
- [ ] ~~copy_backward~~ -> **NOT PLANNED:** Unlikely to be useful in rust?
- [ ] ~~move~~ -> **NOT POSSIBLE:** Due to rust ownership model.
- [ ] ~~move_backward~~ -> **NOT PLANNED:** Unlikely to be useful in rust?

### Swap Operations

- [ ] swap_ranges

### Transformation Operations

- [ ] transform
- [ ] replace
- [ ] replace_if
- [ ] replace_copy_if

### Generation Operations

- [ ] fill
- [ ] fill_n
- [ ] generate
- [ ] generate_n

### Removing Operations

- [ ] remove
- [ ] remove_if
- [ ] remove_copy
- [ ] remove_copy_if
- [ ] unique
- [ ] unique_copy

### Permutation Operations

- [ ] reverse
- [ ] reverse_copy
- [ ] rotate
- [ ] rotate_copy
- [ ] shift_left
- [ ] shift_right
- [ ] next_permutation
- [ ] prev_permutation
- [ ] is_permutation

### Partitioning Operations

- [ ] is_partitioned
- [ ] partition
- [ ] partition_copy
- [ ] stable_partition
- [ ] partition_point

### Sorting Operations

- [ ] sort
- [ ] stable_sort
- [ ] partial_sort
- [ ] partial_sort_copy
- [ ] is_sorted
- [ ] is_sorted_until
- [ ] nth_element

### Binary Search Operations

- [ ] partition_point
- [ ] lower_bound
- [ ] upper_bound
- [ ] equal_range
- [ ] binary_search

### Merge Operations

- [ ] merge
- [ ] inplace_merge

### Minimum/Maximum Operations

- [ ] min_element
- [ ] max_element
- [ ] minmax_element

### Numeric Operations

- [ ] iota
- [ ] accumulate/reduce
- [ ] inner_product
- [ ] adjacent_difference
- [ ] partial_sum
- [ ] exclusive_scan
- [ ] inclusive_scan
- [ ] transform_reduce
- [ ] transform_exclusive_scan
- [ ] transform_inclusive_scan

</details>

## Motivations

The idea of generalization of indexes are not new and motivated from:

- [Hylo Programming Language](https://github.com/hylo-lang/hylo)
- [Flux](https://github.com/tcbrindle/flux)
