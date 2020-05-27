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
