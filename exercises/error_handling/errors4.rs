// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

//

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        let res =  value as u64;
        if res == 0 {
            Err(CreationError::Zero)
        }else if res.to_string() == value.to_string() {
            Ok(PositiveNonzeroInteger(value as u64))
        
        }else{
            Err(CreationError::Negative)
        }
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
