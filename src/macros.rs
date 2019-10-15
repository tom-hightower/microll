#[macro_export]
macro_rules! string {
    ($input:expr) => {
        String::from($input)
    };
}