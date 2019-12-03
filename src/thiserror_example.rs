use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum InternalError {
    #[error("This is my internal error.")]
    FooBar
}

#[derive(Clone, Debug, Error)]
pub enum PublicError {
    #[error("Public Error: {:?}", source)]
    FredBob { 
        source: Arc<dyn std::error::Error>
    },
}

impl From<InternalError> for PublicError {
    fn from(input: InternalError) -> PublicError {
        match input.clone() {
            InternalError::FooBar  => {
                PublicError::FredBob {
                    source: Arc::new(input),
                }
            }
        }
    }
}

fn internal() -> Result<i32, InternalError> {
    Err(InternalError::FooBar)
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


