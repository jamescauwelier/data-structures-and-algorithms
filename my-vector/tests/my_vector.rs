use my_vector::my_vector::MyVector;
use my_vector::myvec;

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

#[test]
fn a_vector_can_be_iterated_on() {
    let mut v = MyVector::new();
    v.add(1);
    v.add(2);
    v.add(3);
    v.add(4);

    let mut counter = 0;
    for _ in v.into_iter() {
        counter += 1;
    }

    assert_eq!(4, counter);
}

#[test]
fn elements_can_be_dropped() {
    let mut v = MyVector::new();
    v.add(1);
    v.add(2);
    v.add(3);
    v.add(4);
    v.drop(1);
    v.drop(0);

    assert_eq!(&3, v.get(0));
    assert_eq!(&4, v.get(1));
    assert!(!v.has(2));
}

#[test]
fn elements_can_be_added_with_macro() {
    let v: MyVector = myvec! [1, 2, 3, 4, 5];
    assert_eq!(&5, v.get(4));
    assert_eq!(5, v.len());
}