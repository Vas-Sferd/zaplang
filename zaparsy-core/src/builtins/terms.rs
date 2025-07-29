pub trait ContextlessRule {
    fn extract(s: &str) -> Result<(&str, &str), String>;
}

#[macro_export]
macro_rules! impl_onechar_rule {
    ($struct:ty, $predicate:expr) => {
        impl ContextlessRule for $struct {
            fn extract(s: &str) -> Result<(&str, &str), String> {
                let mut chars = s.chars();
                if let Some(c) = chars.next() {
                    if ($predicate)(c) {
                        let len = c.len_utf8();
                        return Ok((&s[len..], &s[..len]));
                    }
                }
                Err("Character match failed".to_string())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_multichar_rule {
    ($struct:ty, $predicate:expr) => {
        impl ContextlessRule for $struct {
            fn extract(s: &str) -> Result<(&str, &str), String> {
                let mut count = 0;
                for c in s.chars() {
                    if !($predicate)(c) {
                        break;
                    }
                    count += c.len_utf8();
                }
                if count > 0 {
                    Ok((&s[count..], &s[..count]))
                } else {
                    Err("Sequence match failed".to_string())
                }
            }
        }
    };
}

/// SingleChar Builtins Rules
pub struct Digit;
pub struct HexDigit;
pub struct Lower;
pub struct Upper;
pub struct Alpha;
pub struct Alnum;
pub struct Common;
pub struct Space;
pub struct Tab;
pub struct WhiteSpace;
pub struct Dot;
pub struct Comma;
pub struct Colon;
pub struct Semicolon;
pub struct Quote;
pub struct Squote;
pub struct Backtick;
pub struct Slash;
pub struct BackSlash;
pub struct Underscore;
pub struct Plus;
pub struct Minus;
pub struct Star;
pub struct Ampersand;

// Realization
impl_onechar_rule!(Digit, |c: char| c.is_ascii_digit());
impl_onechar_rule!(HexDigit, |c: char| c.is_ascii_hexdigit());
impl_onechar_rule!(Lower, |c: char| c.is_ascii_lowercase());
impl_onechar_rule!(Upper, |c: char| c.is_ascii_uppercase());
impl_onechar_rule!(Alpha, |c: char| c.is_ascii_alphabetic());
impl_onechar_rule!(Alnum, |c: char| c.is_ascii_alphanumeric());
impl_onechar_rule!(Common, |c: char| !c.is_control());
impl_onechar_rule!(Space, |c: char| c == ' ');
impl_onechar_rule!(Tab, |c: char| c == '\t');
impl_onechar_rule!(EoL, |c: char| c == '\n' || c == '\r');
impl_onechar_rule!(WhiteSpace, |c: char| c.is_ascii_whitespace());
impl_onechar_rule!(Dot, |c: char| c == '.');
impl_onechar_rule!(Comma, |c: char| c == ',');
impl_onechar_rule!(Colon, |c: char| c == ':');
impl_onechar_rule!(Semicolon, |c: char| c == ';');
impl_onechar_rule!(Quote, |c: char| c == '"');
impl_onechar_rule!(Squote, |c: char| c == '\'');
impl_onechar_rule!(Backtick, |c: char| c == '`');
impl_onechar_rule!(Slash, |c: char| c == '/');
impl_onechar_rule!(BackSlash, |c: char| c == '\\');
impl_onechar_rule!(Underscore, |c: char| c == '_');
impl_onechar_rule!(Plus, |c: char| c == '+');
impl_onechar_rule!(Minus, |c: char| c == '-');
impl_onechar_rule!(Star, |c: char| c == '*');
impl_onechar_rule!(Ampersand, |c: char| c == '&');

/// Multichar Builtins Rules
pub struct Digits;
pub struct Letters;
pub struct Alnums;
pub struct WhiteSpaces;
pub struct EoL;
pub struct EoI;
pub struct Commons;

impl_multichar_rule!(Digits, |c: char| c.is_ascii_digit());
impl_multichar_rule!(Letters, |c: char| c.is_ascii_alphabetic());
impl_multichar_rule!(Alnums, |c: char| c.is_ascii_alphanumeric());
impl_multichar_rule!(WhiteSpaces, |c: char| c.is_ascii_whitespace());
impl_multichar_rule!(Commons, |c: char| !c.is_control());

impl ContextlessRule for EoI {
    fn extract(s: &str) -> Result<(&str, &str), String> {
        if s.is_empty() {
            Ok((s, s))
        } else {
            Err("Expected end of input".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(Digits::extract("123"), Ok(("", "123")))
    }

    #[test]
    fn test_alnum() {
        assert_eq!(Alnum::extract("1a"), Ok(("a", "1")));
        assert_eq!(Alnum::extract("a1"), Ok(("1", "a")));
    }
}
