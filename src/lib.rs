pub use derive_vec_proc_macro::VecBehavior;

pub trait VecTrait<T> {
    fn append(&mut self, other: &mut Vec<T>);
    fn as_slice(&self) -> &[T];
    fn push(&mut self, val: T);
}
