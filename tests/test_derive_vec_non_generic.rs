use derive_vec::{VecBehavior, VecTrait};
use std::ops::RangeBounds;
use std::vec::Drain;

#[derive(VecBehavior, Default)]
struct Foo {
    #[vec]
    pub values: Vec<usize>,
}

#[derive(VecBehavior, Default)]
struct Bar {
    #[vec]
    pub values: Vec<usize>,
}

#[test]
fn test_push() {
    let mut foo = Foo::default();
    foo.push(1);
    assert_eq!(foo.values.len(), 1);
}

#[test]
fn test_append() {
    let mut foo = Foo::default();
    foo.append(&mut vec![1, 2, 3]);
    assert_eq!(foo.values.len(), 3);
}

#[test]
fn test_slice() {
    let mut foo = Foo::default();
    foo.append(&mut vec![1, 2, 3]);
    assert_eq!(foo.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_capacity() {
    let foo = Foo::with_capacity(2);
    assert_eq!(foo.capacity(), 2);
}

#[test]
fn test_clear() {
    let mut foo = Foo::default();
    foo.append(&mut vec![1, 2, 3]);
    assert_eq!(foo.len(), 3);
    foo.clear();
    assert_eq!(foo.len(), 0);
}

#[test]
fn test_dedup() {
    let mut foo = Foo::default();
    let mut values = vec![1, 2, 3, 3];
    foo.append(&mut values);
    assert_eq!(&foo.values, &vec![1, 2, 3, 3]);
    foo.dedup();
    assert_eq!(&foo.values, &vec![1, 2, 3]);
}

#[test]
fn test_dedup_by() {
    #[derive(VecBehavior, Default)]
    struct FooBar {
        #[vec]
        pub values: Vec<String>,
    }
    let mut foo = FooBar::default();
    ["foo", "bar", "Bar", "baz", "bar"]
        .iter()
        .for_each(|v| foo.push(v.to_string()));
    foo.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    assert_eq!(foo.values, ["foo", "bar", "baz", "bar"]);
}

#[test]
fn test_dedup_by_key() {
    let mut foo = Foo::default();
    [10, 20, 21, 30, 20].iter().for_each(|v| foo.push(*v));
    foo.dedup_by_key(|i| *i / 10);
    assert_eq!(foo.values, [10, 20, 30, 20]);
}

#[test]
fn test_len_is_empty() {
    let mut foo = Foo::default();
    assert_eq!(foo.len(), 0);
    assert!(foo.is_empty());
    foo.append(&mut vec![1, 2, 3]);
    assert_eq!(foo.len(), 3);
    assert_eq!(foo.is_empty(), false);
}

#[test]
fn test_drain() {
    let mut foo = Foo::default();
    foo.append(&mut vec![1, 2, 3]);

    let u: Vec<_> = foo.drain(1..).collect();
    assert_eq!(&foo.values, &[1]);
    assert_eq!(&u, &[2, 3]);

    foo.drain(..);
    assert_eq!(&foo.values, &[]);
}
