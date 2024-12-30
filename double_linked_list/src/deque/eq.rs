use crate::deque::Deque;

impl<T> PartialEq for Deque<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let mut iter_self = self.into_iter();
        let mut iter_other = other.into_iter();

        loop {
            return match (iter_self.next(), iter_other.next()) {
                // both are of equal length and saw no elements with different values
                // ends the loop !!
                (None, None) => true,
                // self is longer than other, and so not equal
                (Some(_), None) => false,
                // self is shorter than other, so not equal
                (None, Some(_)) => false,
                // not equal if values don't match
                (Some(x1), Some(x2)) if x1.value() != x2.value() => false,
                // no reason to think they're different, so keep looking
                _ => continue,
            };
        }
    }
}

#[cfg(test)]
mod tests {

    mod unit_tests {
        use crate::deque;
        use crate::deque::Deque;

        #[test]
        fn empty_deques_are_equal() {
            let x1: Deque<i64> = deque![];
            let x2: Deque<i64> = deque![];

            assert_eq!(x1, x2);
        }

        #[test]
        fn cloned_deques_are_equal() {
            let x1: Deque<i64> = deque![];
            let x2: Deque<i64> = x1.clone();

            assert_eq!(x1, x2);
        }

        #[test]
        fn equal_deques() {
            let x1: Deque<i64> = deque![1, 2, 3];
            let x2: Deque<i64> = deque![1, 2, 3];

            assert_eq!(x1, x2);
        }

        #[test]
        fn different_deques_1() {
            let x1: Deque<i64> = deque![1, 2, 3];
            let x2: Deque<i64> = deque![3, 2, 1];

            assert_ne!(x1, x2);
        }

        #[test]
        fn different_deques_2() {
            let x1: Deque<i64> = deque![1, 2, 3];
            let x2: Deque<i64> = deque![1, 2];

            assert_ne!(x1, x2);
        }

        #[test]
        fn different_deques_3() {
            let x1: Deque<i64> = deque![1, 2, 3];
            let x2: Deque<i64> = deque![1, 2, 3, 4];

            assert_ne!(x1, x2);
        }
    }

    mod property_tests {
        use crate::deque::Deque;
        use proptest::prelude::*;

        proptest! {
            #[test]
            #[cfg_attr(miri, ignore)]
            fn cloned_deques_are_the_same(original: Deque<usize>){
                let duplicate = original.clone();
                assert_eq! (original, duplicate);
            }
        }
    }
}
