use crate::deque::Deque;

impl<T: Clone> Clone for Deque<T>
where
    T: PartialEq,
{
    fn clone(&self) -> Self {
        let mut cloned = Deque::create();

        let mut current = self.first();
        while let Some(item) = current {
            cloned.push_right(item.value().clone());

            current = item.right();
        }

        cloned
    }
}

#[cfg(test)]
mod tests {
    use crate::deque;

    #[test]
    fn cloning_an_empty_deque() {
        let a = deque![1, 2, 3];
        let b = a.clone();

        assert_eq!(a.first(), b.first());
        assert_eq!(a.last(), b.last());
    }
}
