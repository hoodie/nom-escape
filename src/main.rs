//! Hi there, I would like to use the `escaped` parser combinator to
//! parse arbitrary quoted strings that may contain escaped quotes.
//! It seems to work fine in in the documentation, and when used with in built-ins.
//!
//! But when I try to use it in a more real-worldish example I see inconsistent behavior.
use nom::{
    bytes::complete::{escaped, take_while},
    character::complete::alphanumeric1,
    character::complete::one_of,
    IResult,
};
fn main() {
    fn inner2(input: &str) -> IResult<&str, &str> {
        alphanumeric1(input)
    }

    fn inner1(input: &str) -> IResult<&str, &str> {
        take_while(|c| c != '\\' && c != '"')(input)
    }

    // using escaped as in the docs
    fn escaped1(input: &str) -> IResult<&str, &str> {
        escaped(
            alphanumeric1,
            // take_while(|c| c != '\\' && c != '"'),
            '\\',
            one_of(r#""\n"#),
        )(input)
    }

    // how I would really like to use it
    fn escaped2(input: &str) -> IResult<&str, &str> {
        escaped(
            // alphanumeric1,
            take_while(|c| c != '\\' && c != '"'),
            '\\',
            one_of(r#""\n"#),
        )(input)
    }

    // test1 and test2 produce different results...
    println!("even though both inner parsers produce the same output...");
    let a = dbg!(inner1(r#"hello\"world"#).unwrap()); // = ("\\\"world", "hello",)
    let b = dbg!(inner2(r#"hello\"world"#).unwrap()); // = ("\\\"world", "hello",)
    assert_eq!(a, b);

    println!("...why don't these two escaped parser behave differntly?");
    let a = dbg!(escaped1(r#"hello\"world"#).unwrap()); //  = ("", "hello\\\"world",) <- I want this
    let b = dbg!(escaped2(r#"hello\"world"#).unwrap()); //  = ("\\\"world", "hello",)
    assert_eq!(a, b);
}
