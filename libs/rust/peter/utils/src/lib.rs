pub mod permutations {

    /// An iterator over all permutations using Heap's algorithm in a non-recursive form.
    ///
    /// See https://en.wikipedia.org/wiki/Heap%27s_algorithm
    pub struct Permutations<T: Copy> {
        items: Vec<T>,
        c: Vec<usize>,
        k: usize,
        count: usize,
    }

    impl<T: Copy, D: IntoIterator<Item = T>> From<D> for Permutations<T> {
        fn from(value: D) -> Self {
            let items = Vec::from_iter(value);
            let c = vec![0; items.len()];
            Self {
                items,
                c,
                k: 0,
                count: 0,
            }
        }
    }

    impl<T: Copy> Iterator for Permutations<T> {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count = 1;
                return Some(self.items.clone());
            }

            while self.k < self.items.len() {
                if self.c[self.k] < self.k {
                    if self.k & 1 == 0 {
                        (self.items[0], self.items[self.k]) = (self.items[self.k], self.items[0]);
                    } else {
                        (self.items[self.c[self.k]], self.items[self.k]) =
                            (self.items[self.k], self.items[self.c[self.k]]);
                    }
                    self.c[self.k] += 1;
                    self.k = 0;
                    self.count += 1;
                    return Some(self.items.clone());
                } else {
                    self.c[self.k] = 0;
                    self.k += 1;
                }
            }
            None
        }
    }
}

pub mod euclid {
    use std::ops::Rem;

    /// Calculate multiplicate inverse of `a` modulo `m`
    /// with the [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
    ///
    /// The result is in the range `0..m` (inclusive lower, exclusive upper bound)
    ///
    /// # Examples
    ///
    /// ```
    /// let a = 2;
    /// let m = 13;
    ///
    /// let a_inv = mr_kaffee_utils::euclid::mul_inverse_mod(a, m);
    ///
    /// assert_eq!((a_inv * a) % m, 1)
    /// ```
    ///
    /// # Panics
    /// The function panics if `a` and `m` are not co-prime
    /// ```should_panic
    /// let a = 15;
    /// let m = 27;
    /// let a_inv = mr_kaffee_utils::euclid::mul_inverse_mod(a, m);
    /// ```
    pub fn mul_inverse_mod(a: i64, m: i64) -> i64 {
        let (mut t, mut old_t) = (0, 1);
        let (mut r, mut old_r) = (m, a);

        while old_r != 0 {
            let quotient = r / old_r;

            (r, old_r) = (old_r, r - quotient * old_r);
            (t, old_t) = (old_t, t - quotient * old_t);
        }

        // if GCD != 1, there is no inverse
        assert_eq!(
            1,
            r,
            "a = {} = {} * {} and m = {} = {} * {} are not co-prime",
            a,
            a / r,
            r,
            m,
            m / r,
            r
        );

        if t < 0 {
            t + m
        } else {
            t
        }
    }

    pub trait Zero {
        const ZERO: Self;
    }

    macro_rules! impl_zero {
        ($($t:ty),+) => {$(
            impl Zero for $t {
                const ZERO: Self = 0;
            }
        )*};
    }

    impl_zero!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

    /// Calculate greatest common divisor
    pub fn gcd<T>(mut a: T, mut b: T) -> T
    where
        T: Eq + Rem<T, Output = T> + Zero + Copy,
    {
        while b != T::ZERO {
            (a, b) = (b, a % b);
        }
        a
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn test_mul_inv_mod() {
            assert_eq!(mul_inverse_mod(1, 10), 1);
        }
    }
}

pub mod iterators {
    pub trait FoldWith<T> {
        /// Use this to compute the initial value of a fold from the iterator
        /// that is about being fold.
        ///
        /// # Examples
        /// ```
        /// # use mr_kaffee_utils::iterators::*;
        ///
        /// let data: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        /// let res = data.iter().fold_with(
        ///     |it| Vec::with_capacity(it.size_hint().0),
        ///     |mut acc, v| {
        ///         let upd = acc.last().unwrap_or(&0) + v;
        ///         acc.push(upd);
        ///         acc
        ///     },
        /// );
        /// assert_eq!(vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55], res);
        /// ```
        fn fold_with<I, S, R>(self, init: I, step: S) -> R
        where
            I: FnMut(&Self) -> R,
            S: FnMut(R, T) -> R;
    }

    impl<T, It> FoldWith<T> for It
    where
        It: Iterator<Item = T>,
    {
        fn fold_with<I, S, R>(self, mut init: I, step: S) -> R
        where
            I: FnMut(&Self) -> R,
            S: FnMut(R, T) -> R,
        {
            let initial = init(&self);
            self.fold(initial, step)
        }
    }

    /// Alias for [`Result`] with same type used for [`Result::Ok`] and
    /// [`Result::Err`]
    pub type Alt<T> = Result<T, T>;

    /// Convenience method to construct a [`Result::Ok`] value of the first
    /// parameter if the second parameter is true, otherwise construct and
    /// [`Result::Err`] value.
    pub fn ok_if<T>(value: T, ok: bool) -> Alt<T> {
        if ok {
            Ok(value)
        } else {
            Err(value)
        }
    }

    pub trait FoldShortCuttable<T> {
        /// Use this for folds where all relevant information may be available
        /// before the iterator is exhausted.
        ///
        /// # Examples
        /// ```
        /// # use mr_kaffee_utils::iterators::*;
        ///
        /// let data: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        /// let threshold = 30;
        ///
        /// let big = data.iter().fold_short_cuttable(0, |mut sum, v| {
        ///     sum += v;
        ///     ok_if(sum, sum < threshold)
        /// }) > threshold;
        ///
        /// assert_eq!(true, big);
        /// ```
        fn fold_short_cuttable<S, R>(self, initial: R, step: S) -> R
        where
            S: FnMut(R, T) -> Alt<R>;

        fn fold_short_cuttable_with<I, S, R>(self, mut init: I, step: S) -> R
        where
            I: FnMut(&Self) -> R,
            S: FnMut(R, T) -> Alt<R>,
            Self: Sized,
        {
            let initial = init(&self);
            self.fold_short_cuttable(initial, step)
        }
    }

    impl<T, It> FoldShortCuttable<T> for It
    where
        It: Iterator<Item = T>,
    {
        fn fold_short_cuttable<S, R>(mut self, initial: R, mut step: S) -> R
        where
            S: FnMut(R, T) -> Alt<R>,
        {
            let mut acc = initial;
            while let Some(value) = self.next() {
                acc = match step(acc, value) {
                    Ok(upd) => upd,
                    Err(acc) => return acc,
                };
            }
            acc
        }
    }
}

pub mod grids;
pub mod letters;
