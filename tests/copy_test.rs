// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    pub fn copy_if_to_end() {
        let vec1 = vec![1, 2, 3, 4];
        let mut vec2 = vec![0, 0, 0];

        let j =
            algo::copy_if(&vec1, 2, vec1.end(), &mut vec2, 1, |x| x % 2 == 0);

        assert!(vec2.equals(&vec![0, 4, 0]));
        assert_eq!(j, 2);

        let j = rng::copy_if(&vec1, &mut vec2, 1, |x| x % 2 == 0);
        assert!(vec2.equals(&vec![0, 2, 4]));
        assert_eq!(j, 3);
    }

    #[test]
    pub fn copy_to_end() {
        let vec1 = vec![1, 2, 3, 4];
        let mut vec2 = vec![0, 0, 0, 0, 0];

        let j = algo::copy(&vec1, 2, vec1.end(), &mut vec2, 1);

        assert!(vec2.equals(&vec![0, 3, 4, 0, 0]));
        assert_eq!(j, 3);

        let j = rng::copy(&vec1, &mut vec2, 1);
        assert!(vec2.equals(&vec![0, 1, 2, 3, 4]));
        assert_eq!(j, 5);
    }

    #[test]
    pub fn copy_if_empty_range() {
        let vec1 = vec![];
        let mut vec2 = vec![0, 0, 0];

        let j =
            algo::copy_if(&vec1, 0, vec1.end(), &mut vec2, 1, |x| x % 2 == 0);

        assert!(vec2.equals(&vec![0, 0, 0]));
        assert_eq!(j, 1);

        let j = rng::copy_if(&vec1, &mut vec2, 1, |x| x % 2 == 0);
        assert!(vec2.equals(&vec![0, 0, 0]));
        assert_eq!(j, 1);
    }

    #[test]
    pub fn copy_empty_range() {
        let vec1 = vec![];
        let mut vec2 = vec![0, 0, 0, 0, 0];

        let j = algo::copy(&vec1, 0, vec1.end(), &mut vec2, 1);

        assert!(vec2.equals(&vec![0, 0, 0, 0, 0]));
        assert_eq!(j, 1);

        let j = rng::copy(&vec1, &mut vec2, 1);
        assert!(vec2.equals(&vec![0, 0, 0, 0, 0]));
        assert_eq!(j, 1);
    }
}
