pub mod entity {
    pub struct AudioChunk<T: Clone> {
        data: Vec<T>,
    }

    impl<T: Clone> AudioChunk<T> {
        pub fn new(data: &[T]) -> Self {
            Self {
                data: data.to_vec(),
            }
        }
    }
}
