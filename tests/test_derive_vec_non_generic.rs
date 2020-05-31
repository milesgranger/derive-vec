use derive_vec::{VecBehavior, VecTrait};

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
fn test_len_is_empty() {
    let mut foo = Foo::default();
    assert_eq!(foo.len(), 0);
    assert!(foo.is_empty());
    foo.append(&mut vec![1, 2, 3]);
    assert_eq!(foo.len(), 3);
    assert_eq!(foo.is_empty(), false);
}
