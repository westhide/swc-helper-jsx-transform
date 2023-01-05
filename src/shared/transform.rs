pub trait Transform<'a, T> {
    fn transform(&'a self) -> T;
}

impl<'a, T: Transform<'a, U>, U> Transform<'a, Vec<U>> for [T] {
    fn transform(&'a self) -> Vec<U> {
        self.iter().map(T::transform).collect()
    }
}
