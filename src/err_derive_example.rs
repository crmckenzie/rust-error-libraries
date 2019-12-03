///
/// In order to support `Clone`, we must wrap
/// `source` and `backtrace` in `Arc`'s.
/// 
/// Snafu misses the mark in the following ways:
/// while the source attribute does allow you to 
/// specify a constructor function, the constructor
/// function does not work with traits. You can get
/// around this by manually adding `map_err` to
/// your calling function, but this is less than
/// ideal.
/// 
/// It's also impossible to add the backtrace using
/// this model as Snafu errors out.
///
/// 
use std::sync::Arc;
use backtrace::Backtrace;

#[derive(Clone, Debug, Error)]
pub enum InternalError {
    #[error(display="This is my internal error.")]
    FooBar{
        backtrace: Backtrace
    },
}

#[derive(Clone, Debug, Error)]
pub enum PublicError {
    #[error(display = "Public Error: {:?}", source)]
    FredBob { 
        source: Arc<dyn std::error::Error>,
        backtrace: Backtrace
    },
}

impl From<InternalError> for PublicError {
    fn from(input: InternalError) -> PublicError {
        match input.clone() {
            InternalError::FooBar { backtrace } => {
                PublicError::FredBob {
                    source: Arc::new(input),
                    backtrace,
                }
            }
        }
    }
}

fn internal() -> Result<i32, InternalError> {
    Err(InternalError::FooBar {
        backtrace: backtrace::Backtrace::new()
    })
}

pub fn raise_public_error() -> Result<i32, PublicError> {
    internal().map_err(|input| input.into())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn clone() {
        let result = raise_public_error();

        let cloned = result.clone();

        println!("{:?}", cloned);
    }
}


