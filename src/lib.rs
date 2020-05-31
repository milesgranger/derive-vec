pub use derive_vec_proc_macro::VecBehavior;

pub trait VecTrait<T>: Default {
    fn append(&mut self, other: &mut Vec<T>);
    fn as_slice(&self) -> &[T];
    fn capacity(&self) -> usize;
    fn with_capacity(capacity: usize) -> Self;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, val: T);
}
