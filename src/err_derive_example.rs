use std::sync::Arc;
use backtrace::Backtrace;
use err_derive::Error;

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


