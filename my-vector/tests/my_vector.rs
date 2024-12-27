use my_vector::my_vector::MyVector;

#[test]
fn an_new_vector_starts_out_empty() {
    let v = MyVector::new();
    assert_eq!(0, v.len());
}

#[test]
fn an_element_can_be_added_to_a_vector() {
    let mut v = MyVector::new();
    v.add(1);
    assert_eq!(1, v.len());
    assert_eq!(&1, v.get(0));
}

#[test]
fn two_elements_can_be_added_to_a_vector() {
    let mut v = MyVector::new();
    v.add(1);
    v.add(2);
    assert_eq!(2, v.len());
    assert_eq!(&1, v.get(0));
    assert_eq!(&2, v.get(1));
}

#[test]
fn after_adding_an_element_can_be_found() {
    let mut v = MyVector::new();
    v.add(1);

    let result = v.find(|x| x == &1);
    assert_eq!(Some(&1), result);
}

#[test]
fn without_adding_an_element_cannot_be_found() {
    let mut v = MyVector::new();
    v.add(2);

    let result = v.find(|x| x == &1);
    assert_eq!(None, result);
}