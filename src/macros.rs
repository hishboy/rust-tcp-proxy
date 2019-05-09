#[macro_export]
macro_rules! try_or_continue {
    ($r:expr) => {
        if let Ok(val) = $r {
            val
        } else {
            continue;
        }
    };
}
