pub use derive_vec_proc_macro::VecBehavior;

pub trait VecTrait<T> {
    fn append(&mut self, other: &mut Vec<T>);
    fn push(&mut self, val: T);
}
