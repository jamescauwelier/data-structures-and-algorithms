use double_linked_list::deque::Deque;
use double_linked_list::deque::item::Item;

#[test]
fn creates_an_empty_deque() {
    let d: Deque<usize> = Deque::create();
    assert_eq!(0, d.len());
    assert_eq!(None, d.first());
    assert_eq!(None, d.last());
}

#[test]
fn push_left_once() {
    let mut data: Deque<usize> = Deque::create();
    data.push_left(1);

    let first = data.first().unwrap();
    assert_eq!(&0, first.value());
    assert_eq!(None, first.left());
    assert_eq!(None, first.right());

    let last = data.last().unwrap();
    assert_eq!(&0, last.value());
    assert_eq!(None, last.left());
    assert_eq!(None, last.right());
}

