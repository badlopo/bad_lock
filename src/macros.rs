#[macro_export]
macro_rules! get_md5 {
    ($source:expr) => {
        md5::compute($source).0
    };
}

#[macro_export]
macro_rules! timestamp {
    () => {
        std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    };
}