#[macro_export]
macro_rules! stack {
    () => {
        format!("File: {} ||| Line: {} ||| Col: {}", file!(), line!(), column!())
    }
}