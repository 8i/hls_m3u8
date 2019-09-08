use crate::{ErrorKind, Result};
use trackable::error::ErrorKindExt;

pub(crate) fn parse_yes_or_no<T: AsRef<str>>(s: T) -> Result<bool> {
    match s.as_ref() {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => track_panic!(
            ErrorKind::InvalidInput,
            "Unexpected value: {:?}",
            s.as_ref()
        ),
    }
}

pub(crate) fn parse_u64<T: AsRef<str>>(s: T) -> Result<u64> {
    let n = track!(s
        .as_ref()
        .parse()
        .map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
    Ok(n)
}

/// According to the documentation the following characters are forbidden
/// inside a quoted string:
/// - carriage return (`\r`)
/// - new line (`\n`)
/// - double quotes (`"`)
///
/// Therefore it is safe to simply remove any occurence of those characters.
/// [rfc8216#section-4.2](https://tools.ietf.org/html/rfc8216#section-4.2)
pub(crate) fn unquote<T: ToString>(value: T) -> String {
    value
        .to_string()
        .replace("\"", "")
        .replace("\n", "")
        .replace("\r", "")
}

/// Puts a string inside quotes.
pub(crate) fn quote<T: ToString>(value: T) -> String {
    // the replace is for the case, that quote is called on an already quoted string, which could
    // cause problems!
    format!("\"{}\"", value.to_string().replace("\"", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yes_or_no() {
        assert!(parse_yes_or_no("YES").unwrap());
        assert!(!parse_yes_or_no("NO").unwrap());
        // TODO: test for error
    }

    #[test]
    fn test_parse_u64() {
        assert_eq!(parse_u64("1").unwrap(), 1);
        assert_eq!(parse_u64("25").unwrap(), 25);
        // TODO: test for error
    }

    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"TestValue\""), "TestValue".to_string());
        assert_eq!(unquote("\"TestValue\n\""), "TestValue".to_string());
        assert_eq!(unquote("\"TestValue\n\r\""), "TestValue".to_string());
    }

    #[test]
    fn test_quote() {
        assert_eq!(quote("value"), "\"value\"".to_string());
        assert_eq!(quote("\"value\""), "\"value\"".to_string());
    }
}
