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

    /// Calculate greatest common divisor
    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
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

pub mod letters;
pub mod grids;
