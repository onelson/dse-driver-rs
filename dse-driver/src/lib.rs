pub mod graph;

pub(crate) trait PtrProxy {
    type T;
    fn ptr(&self) -> *const Self::T;
    fn ptr_mut(&mut self) -> *mut Self::T {
        self.ptr() as *mut Self::T
    }
}
