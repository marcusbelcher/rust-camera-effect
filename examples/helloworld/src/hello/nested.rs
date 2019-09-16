/// Output returns "Hello, world!"
///
/// N/A
///
/// ```
/// assert_eq!(output(), "Hello, world!".to_string());
/// ```
fn output() -> String {
    "Hello, world!".to_string()
}

/// Extern output returns "Hello, world!"
///
/// External version of output()
///
/// ```
/// assert_eq!(extern_output(), "Hello, world!".to_string());
/// ```
#[allow(dead_code)]
pub fn extern_output() -> String {
    output()
}

#[allow(dead_code)]
pub fn print() {
    println!("{}", output());
}

#[cfg(test)]
mod tests {
    use super::*;
    // use nested;

    #[test]
    fn test_output() {
        assert_eq!(output(), "Hello, world!".to_string());
    }

    #[test]
    fn test_extern_output() {
        assert_eq!(output(), extern_output());
    }
}
