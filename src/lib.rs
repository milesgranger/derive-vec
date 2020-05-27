pub use derive_vec_proc_macro::VecBehavior;


pub trait VecTrait<T> {
    fn push(&mut self, val: T);
}

