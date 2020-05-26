use derive_vec::VecLike;

#[derive(VecLike)]
struct Foo<T> {
    #[vec]
    values: Vec<T>,
}

#[test]
fn test_new() {}
