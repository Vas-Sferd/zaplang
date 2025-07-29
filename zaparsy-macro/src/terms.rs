/// Any digit char
pub struct Digit;

/// Any lower ASCI letter
pub struct Lower;

/// Any upper ASCI letter
pub struct Upper;

/// Any ASCI letter
pub struct Aplha;

/// Any ASCI letter or any digit
pub struct Alnum;

/// The Spacebar ' '
pub struct Space;

/// The tabulation "\t" symbol
pub struct Tab;

/// End of line - EOL symbol
pub struct EndOfLine;

/// Any white space symbol (Space, Tab or EOL)
pub struct WhiteSpace;

/// The dot Symbol
pub struct Dot;

/// The comma symbol
pub struct Comma;

/// The comma symbol
pub struct Colon;

/// The semicolon symbol
pub struct Semicolon;

/// The double quote symbol
pub struct Quote;

/// The single quote symbol
pub struct Squote;

pub struct Backtick;

pub struct Slash;

pub struct BackSlash;

pub struct Underscore;

pub struct Plus;

pub struct Minus;

pub struct Star;

pub struct Ampersand;

// Sequence of one or more digits
pub struct Digits;

/// Sequence of one or more letters
pub struct Letters;

/// Sequence of one or more alphanumeric characters
pub struct Alnums;

/// Sequence of one or more whitespace characters
pub struct WhiteSpaces;

/// End of input
pub struct Eoi;

/// Match any single character, that not special utf char or emoji
pub struct Common;

/// Match any single character except the given one
pub struct NotChar(char);

/// Match any of the provided characters
pub struct OneOfChars(&'static [char]);

/// Match none of the provided characters
pub struct NoneOfChars(&'static [char]);

/// Match a string literal
pub struct Lit;

/// Optional pattern
pub struct Opt<P>(P);

/// Zero or more repetitions
pub struct Many0<P>(P);

/// One or more repetitions
pub struct Many1<P>(P);

/// Separated list
pub struct SepBy<P, S>(P, S);

/// Map parsed value with a function
pub struct Map<P, F>(P, F);

/// Verify parsed value with a predicate
pub struct Verify<P, F>(P, F);
