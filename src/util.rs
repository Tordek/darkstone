pub enum Query<T, U> {
    Pending,
    Loaded(T),
    Error(U),
}

pub async fn read_file(filename: String) -> std::result::Result<String, std::io::ErrorKind> {
    let pathname = std::path::Path::new(&filename);
    tokio::fs::read_to_string(pathname)
        .await
        .map_err(|e| e.kind())
}
