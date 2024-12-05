pub enum Query<T, U> {
    Pending,
    Loaded(T),
    Error(U),
}

pub async fn read_file(
    pathname: std::path::PathBuf,
) -> std::result::Result<String, std::io::ErrorKind> {
    tokio::fs::read_to_string(pathname)
        .await
        .map_err(|e| e.kind())
}
