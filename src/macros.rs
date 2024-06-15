#[macro_export]
macro_rules! get_md5 {
    ($source:expr) => {
        md5::compute($source).0
    };
}