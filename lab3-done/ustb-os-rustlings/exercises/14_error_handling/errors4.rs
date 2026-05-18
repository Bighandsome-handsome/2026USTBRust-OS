// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        // Add code to return an Err value if the value is negative or zero. And an Ok.
        if value < 0 {
            return Err(CreationError::Negative);
        }
        if value == 0 {
            return Err(CreationError::Zero);
        }
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
