macro_rules! deque {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_deque = crate::deque::Deque::create();
            $(
                temp_deque.push_right($x);
            )*
            temp_deque
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::deque::Deque;

    #[test]
    fn create_empty_deque() {
        let data: Deque<usize> = deque![];
        assert_eq!(data.first(), None);
        assert_eq!(data.last(), None);
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn create_deque_1() {
        let data = deque![9];
        assert_eq!(data.first().unwrap().value, 9);
        assert_eq!(data.last().unwrap().value, 9);
        assert_eq!(data.len(), 1);
    }

    #[test]
    fn create_deque_2() {
        let data = deque![1, 2, 4];
        assert_eq!(data.first().unwrap().value, 1);
        assert_eq!(data.last().unwrap().value, 4);
        assert_eq!(data.len(), 3);
    }
}
