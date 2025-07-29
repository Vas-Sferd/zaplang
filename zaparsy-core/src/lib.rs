pub mod builtins;

use builtins::terms::ContextlessRule;
use zaparsy_macro::ContextlessRule;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(ContextlessRule)]
    #[choice_literal("let", "zap", "do")]
    struct Keyword;

    #[test]
    fn test_digits() {
        assert_eq!(Keyword::extract("let a = 5;"), Ok((" a = 5;", "let")))
    }
}
