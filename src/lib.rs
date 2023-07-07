#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

/// Computes the factorial of the input!
///
/// ```ignore
/// use testing_rust::factorial;
///
/// assert_eq!(2, factorial(2));
/// assert_eq!(24, factorial(4));
/// ```
pub fn factorial(input: i64) -> i64 {
    let mut f: i64 = 1;
    for i in 1..input {
        f = f.saturating_add(f.saturating_mul(i));
    }
    f
}

pub fn factorial_of_str(input: &[u8]) -> Result<i64, &'static str> {
    let string_input = std::str::from_utf8(input).map_err(|_| "input not a valid UTF-8 string")?;
    let int = string_input.parse().map_err(|_| "input not a valid integer")?;
    Ok(factorial(int))
}

/// this is a test module
/// which is a way to Isolate build and test code
#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use testdir::testdir;

    #[test]
    fn test_testdir_write() {
        let dir: PathBuf = testdir!();
        let path = dir.join("hello.txt");
        std::fs::write(&path, "hi there").ok();
        assert!(path.exists());
    }

    #[test]
    fn test_testdir_read() {
        let dir: PathBuf = testdir!();
        let path = dir.join("hello.txt");
        std::fs::write(&path, "hi there").ok();
        assert!(path.exists());
        let read_ctx = std::fs::read_to_string(path.to_str().unwrap()).unwrap();
        assert_eq!(read_ctx, "hi there");
    }

    #[test]
    fn factorial_of_5_is_120(){
        assert_eq!(120, factorial(5));
    }

    // Uses the quickcheck crate for property-based tests
    // https://github.com/BurntSushi/quickcheck
    //
    // This function will be automatically called with a variety of inputs. Since this is a unit function,
    // the test will pass as long as it doesn't panic. You could also return a `Testable` value like a `bool`
    #[ignore]
    #[quickcheck]
    fn factorial_does_not_panic(input: i64) {
        factorial(input);
    }

    #[test]
    fn factorial_returns_max_value_for_any_input_over_20() {
        assert_eq!(i64::max_value(), factorial(21));
        assert_eq!(i64::max_value(), factorial(42));
        assert_eq!(i64::max_value(), factorial(900));
    }
}
