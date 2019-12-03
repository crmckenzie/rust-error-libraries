mod snafu_example;

fn main() {
    println!("Hello, world!");
    let result = snafu_example::raise_snafu_error();
    let cloned = result.clone().unwrap_err();
    println!("{:?}", cloned);

    

}
