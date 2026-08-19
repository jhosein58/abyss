pub trait ComptimeProvider {
    type FuncHandle: Copy + Clone + Send + Sync;

    fn eval_function(&mut self, handle: Self::FuncHandle, args: &[u64]) -> Option<u64>;
}
