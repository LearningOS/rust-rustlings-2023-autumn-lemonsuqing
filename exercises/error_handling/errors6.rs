// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
     fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError{
        ParsePosNonzeroError::ParseInt(err)
     }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
    /*

        这段 Rust 代码中的 `s.parse().map_err(ParsePosNonzeroError::from_parseint)?` 
    表示将字符串 `s` 解析为一个 `i64` 类型的值，并将解析错误转换为一个自定义的 `ParsePosNonzeroError` 类型。
    如果解析成功，则返回一个包含解析后的值的 `Ok` 变体；否则，返回一个包含错误信息的 `Err` 变体。
    在这个例子中，我们使用了 `?` 运算符来处理解析错误。如果解析失败，则会立即返回一个包含错误信息的 `Err` 变体。

        另外，`PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)` 表示
    创建一个新的 `PositiveNonzeroInteger` 实例，并将输入值转换为该类型。
    如果输入值为负数或零，则返回一个包含错误信息的 `Err` 变体；
    否则，返回一个包含新实例的 `Ok` 变体。

    */
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
