pub mod graph;

pub(crate) trait Ptr {
    type T;
    fn ptr(&self) -> *const Self::T;
}

pub(crate) trait PtrMut {
    type T;
    fn ptr_mut(&mut self) -> *mut Self::T;
}
