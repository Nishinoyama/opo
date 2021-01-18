
pub mod matching;
pub mod players;
pub mod swiss_system_tournament;
mod matching_algorithm;

/// assert approximately equal
#[macro_export]
macro_rules! assert_ap {
    ($left:expr, $right:expr, $eps:expr $(,)?) => {
        assert!($left > $right - $eps);
        assert!($left < $right + $eps);
    };
    ($left:expr, $right:expr, $eps:expr, $($arg:tt)?) => {
        assert!($left > $right - $eps, $arg);
        assert!($left < $right + $eps, $arg);
    };
}
