// tag::prelude[]
pub const IDENTIFIER: &str = "2023/00";

pub type InputType = &'static str;
pub type Sol1Type = usize;
// end::prelude[]

// tag::parse[]
pub const FERRIS: &str = r"    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /";

/// Parse the puzzle input.
/// 
/// # Examples
/// ```
/// # use mr_kaffee_2023_00::*;
/// let input = parse_input();
/// assert_eq!(FERRIS, input, "Expect input to be ferris");
/// ```
pub fn parse_input() -> InputType {
    FERRIS
}
// end::parse[]

// tag::star_1[]
pub fn star_1(input: &str) -> Sol1Type {
    println!("{}", input);
    0
}
// end::star_1[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// This is a repetition of the doctest for the [`parse_input`] function.
    pub fn test_parse_input() {
        assert_eq!(FERRIS, parse_input());
    }

    #[should_panic]
    #[test]
    /// This is an arbitrary test which is expected to panic.
    pub fn test_panic() {
        panic!("This will panic!");
    }
}
// end::tests[]
