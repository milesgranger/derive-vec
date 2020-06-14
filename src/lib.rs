pub use derive_vec_proc_macro::VecBehavior;
use std::ops::RangeBounds;
use std::vec::Drain;

pub trait VecTrait<T>: Default {
    fn append(&mut self, other: &mut Vec<T>);
    fn as_slice(&self) -> &[T];
    fn capacity(&self) -> usize;
    fn with_capacity(capacity: usize) -> Self;
    fn clear(&mut self);
    fn dedup(&mut self);
    fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool;
    fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>;
    fn drain<R>(&mut self, range: R) -> Drain<T>
    where
        R: RangeBounds<usize>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, val: T);
}
