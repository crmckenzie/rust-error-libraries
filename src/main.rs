#[macro_use]
extern crate err_derive;

mod snafu_example;
mod err_derive_example;

fn main() {
    let result = snafu_example::raise_public_error();
    let cloned = result.clone().unwrap_err();
    println!("snafu Example: {:?}", cloned);

    let result = err_derive_example::raise_public_error();
    let cloned = result.clone().unwrap_err();
    println!("err-derive example: {:?}", cloned);
    match cloned {
        err_derive_example::PublicError::FredBob {source: _, backtrace} => {
            println!("backtrace: {:?}", backtrace);
        }
    }

}
