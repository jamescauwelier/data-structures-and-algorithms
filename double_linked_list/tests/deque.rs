use double_linked_list::deque::Deque;

#[test]
fn creates_a_deque() {
    let d = Deque::create();
    assert_eq!(0, d.len());
}