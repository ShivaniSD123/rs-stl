// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{algo, InputRange, OutputRange, Regular, SemiRegular};

/// Moves all except first of adjacent equivalent elements by given equivalence relationship to end
/// of range.
///
/// # Precondition
///   - binary_pred should follow equivalence relationship, otherwise behavior
///     is undefined.
///
/// # Postcondition
///   - Removes all the adjacent equivalent elements from rng by moving them to
///     end of rng.
///   - NOTE: rng size would not be changed by this.
///   - Relative order of preserved elements are maintained.
///   - Relative order of removed elements are NOT guaranteed.
///   - Returns the position to new end of rng.
///   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
///
///   Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [(1, 2), (1, 3), (2, 3)];
/// let i = rng::unique_by(&mut arr, |x, y| x.0 == y.0);
/// assert_eq!(i, 2);
/// assert!(arr[..i].equals(&[(1, 2), (2, 3)]));
///
/// let mut arr = [(1, 2), (1, 3), (2, 3)];
/// let i = arr.unique_by(|x, y| x.0 == y.0);
/// assert_eq!(i, 2);
/// assert!(arr[..i].equals(&[(1, 2), (2, 3)]));
/// ```
pub fn unique_by<Range, BinaryPred>(
    rng: &mut Range,
    bi_pred: BinaryPred,
) -> Range::Position
where
    Range: OutputRange + ?Sized,
    BinaryPred: Fn(&Range::Element, &Range::Element) -> bool,
{
    algo::unique_by(rng, rng.start(), rng.end(), bi_pred)
}

/// Moves all except first of adjacent equal elements to end of range.
///
/// # Precondition
///
/// # Postcondition
///   - Removes all the adjacent equivalent elements from rng by moving them to
///     end of rng.
///   - NOTE: rng size would not be changed by this.
///   - Relative order of preserved elements are maintained.
///   - Relative order of removed elements are NOT guaranteed.
///   - Returns the position to new end of rng.
///   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
///
///   Where n is number of elements in rng.
///
/// #### Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let mut arr = [1, 1, 2];
/// let i = rng::unique(&mut arr);
/// assert_eq!(i, 2);
/// assert!(arr[..i].equals(&[1, 2]));
///
/// let mut arr = [1, 1, 2];
/// let i = arr.unique();
/// assert_eq!(i, 2);
/// assert!(arr[..i].equals(&[1, 2]));
/// ```
pub fn unique<Range>(rng: &mut Range) -> Range::Position
where
    Range: OutputRange + ?Sized,
    Range::Element: SemiRegular,
{
    algo::unique(rng, rng.start(), rng.end())
}

pub mod infix {
    use crate::{rng, OutputRange, SemiRegular};

    /// `unique`, `unique_by`.
    pub trait STLUniqueExt: OutputRange {
        fn unique_by<F>(&mut self, bi_pred: F) -> Self::Position
        where
            F: Fn(&Self::Element, &Self::Element) -> bool;

        fn unique(&mut self) -> Self::Position
        where
            Self::Element: SemiRegular;
    }

    impl<R> STLUniqueExt for R
    where
        R: OutputRange + ?Sized,
    {
        fn unique_by<F>(&mut self, bi_pred: F) -> Self::Position
        where
            F: Fn(&Self::Element, &Self::Element) -> bool,
        {
            rng::unique_by(self, bi_pred)
        }

        fn unique(&mut self) -> Self::Position
        where
            Self::Element: SemiRegular,
        {
            rng::unique(self)
        }
    }
}

/// Copies all elements from src to dest with copying only first of adjacent equivalent elements by
/// given equivalence relationship.
///
/// # Precondition
///   - dest can accomodate copied elements.
///   - binary_pred should follow equivalence relationship, otherwise behavior
///     is undefined.
///
/// # Postcondition
///   - Copies elements from src with omitting adjacent equivalent elements
///     by bi_pred to dest.
///   - Returns the position of past to last copied element in dest.
///   - Complexity: O(n). Exactly max(0, n - 1) applications of bi_pred.
///
///   Where n is number of elements in src.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [(1, 2), (1, 3), (2, 3)];
/// let mut dest = [(0, 0), (0, 0)];
/// let i = rng::unique_copy_by(&src, &mut dest, |x, y| x.0 == y.0);
/// assert_eq!(i, 2);
/// assert!(dest.equals(&[(1, 2), (2, 3)]));
/// ```
pub fn unique_copy_by<SrcRange, DestRange, BinaryPred>(
    src: &SrcRange,
    dest: &mut DestRange,
    bi_pred: BinaryPred,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Clone,
    BinaryPred: Fn(&SrcRange::Element, &SrcRange::Element) -> bool,
{
    algo::unique_copy_by(
        src,
        src.start(),
        src.end(),
        dest,
        dest.start(),
        bi_pred,
    )
}

/// Copies all elements from src to dest with copying only first of adjacent equal elements.
///
/// # Precondition
///   - dest can accomodate copied elements.
///
/// # Postcondition
///   - Copies elements from src with omitting adjacent equal elements to dest.
///   - Returns the position of past to last copied element in dest.
///   - Complexity: O(n). Exactly max(0, n - 1) equality comparisions.
///
///   Where n is number of elements in src.
///
/// #### Infix Supported
/// NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
///
/// let src = [1, 1, 2];
/// let mut dest = [0, 0];
/// let i = rng::unique_copy(&src, &mut dest);
/// assert_eq!(i, 2);
/// assert!(dest.equals(&[1, 2]));
/// ```
pub fn unique_copy<SrcRange, DestRange>(
    src: &SrcRange,
    dest: &mut DestRange,
) -> DestRange::Position
where
    SrcRange: InputRange + ?Sized,
    DestRange: OutputRange<Element = SrcRange::Element> + ?Sized,
    SrcRange::Element: Regular,
{
    algo::unique_copy(src, src.start(), src.end(), dest, dest.start())
}
