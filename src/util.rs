pub enum Query<T, U> {
    Pending,
    Loaded(T),
    Error(U),
}