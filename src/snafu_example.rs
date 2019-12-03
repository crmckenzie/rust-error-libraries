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
use snafu::*;



#[derive(Clone, Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("This is my internal error"))]
    FooBar 
    //{
        // Snafu won't let us handle the construction of the backtrace.
        // backtrace: Arc<Backtrace>
    //},
}

#[derive(Clone, Debug, Snafu)]
pub enum PublicError {
    #[snafu(display("Public Error: {:?}", source))]
    FredBob { 
        source: Arc<dyn std::error::Error>,
        // Snafu won't let us handle the construction of the backtrace.
        // backtrace: Arc::new(Backtrace) 
    },
}


impl From<InternalError> for PublicError {
    fn from(input: InternalError) -> PublicError {
        match input.clone() {
            InternalError::FooBar => {
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


