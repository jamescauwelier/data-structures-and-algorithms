pub trait Hasher {
    fn hash(&self, capacity: usize) -> usize;
}

impl Hasher for usize {
    fn hash(&self, capacity: usize) -> usize {
        self % capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn hashed_values_are_within_bounds(original: usize, capacity: usize) {
            let hashed = original.hash(capacity);
            assert!(hashed <= capacity);
        }

        #[test]
        fn hashing_is_deterministic(original: usize, capacity: usize) {
            let hashed = original.hash(capacity);
            let hashed_again = original.hash(capacity);
            assert_eq!(hashed, hashed_again);
        }

        #[test]
        fn multiple_hash_outputs_are_possible(first: usize, capacity in 2..=1000_usize) {
            let hashed_first = first.hash(capacity);
            let hashed_second = (first + 1).hash(capacity);
            assert_ne!(hashed_first, hashed_second);
        }
    }
}
