use double_linked_list::deque::Deque;

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

    assert_eq!(1, data.len());

    let first = data.first().unwrap();
    assert_eq!(&1, first.value());
    assert_eq!(None, first.left());
    assert_eq!(None, first.right());

    let last = data.last().unwrap();
    assert_eq!(&1, last.value());
    assert_eq!(None, last.left());
    assert_eq!(None, last.right());
}

#[test]
fn push_right_once() {
    let mut data: Deque<usize> = Deque::create();
    data.push_right(1);

    assert_eq!(1, data.len());

    let first = data.first().unwrap();
    assert_eq!(&1, first.value());
    assert_eq!(None, first.left());
    assert_eq!(None, first.right());

    let last = data.last().unwrap();
    assert_eq!(&1, last.value());
    assert_eq!(None, last.left());
    assert_eq!(None, last.right());
}
