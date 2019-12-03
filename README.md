# Error Handling Exploration

## Values

1. If a function can fail for only one reason, **do not** use an error enum. Rather, use an error struct.
1. If a function can fail for multiple reasons, create an error enum with a
struct-like variant for each reason.
1. Error types should implement `std::error::Error`.
1. Error types should derive `Clone` to support mocking in a test context.
1. Errors that originate from other errors should curry the originating error
as the `source`.
    1. Caveat: They should not _expose_ the inner error type. `source` should be
    defined as `std::error::Error` so that internal dependencies do not leak.

## How a good library can help

1. Offer a `#[derive(std::error::Error)]` macro.
1. Support `Arc<std::error::Error>` as a source via something like the `context` function.
1. Support `Display` and `Debug` formatting macros.

## Libraries Evaluated

* [snafu](https://docs.rs/snafu/0.6.0/snafu/index.html)
* [err-derive](https://gitlab.com/torkleyy/err-derive)
* [quick-error](https://docs.rs/quick-error/1.2.2/quick_error/)
* [thiserror](https://github.com/dtolnay/thiserror)
* [anyhow](https://github.com/dtolnay/anyhow)

### Snafu

* Snafu offers backtrace support, but it conflicts with
the `Clone` requirement.

In order to support `Clone`, we must wrap
`source` and `backtrace` in `Arc`'s.
 
Snafu misses the mark in the following ways:
while the source attribute does allow you to 
specify a constructor function, the constructor
function does not work with traits. You can get
around this by manually adding `map_err` to
your calling function, but this is less than
ideal.
 
It's also impossible to add the backtrace using
this model as Snafu errors out.

### quick-error

Rejected because

1. It adds a new macro language.
1. It does not solve the leaking of internal error types problem.

### err-derive

`err-derive` is a port of the `failure` crate except
it's designed to work with `std::error::Error`. This is
good in that in using it we'd depend only on the standard
library. This is bad in that the standard library does not
support `Backtrace` except on nightly.

`err-derive` also lacks macros that let us auto-implement 
`From` implementations for our source fields.

I was able to get it to work with the `backtrace` crate.

Conclusion: It's an _okay_ solution, but still requires
a lot of boilerplate.


### thiserror

The `thiserror` crate is nice in its simplicity but has all of the same
capabilities and limitations of `err-derive`. Caveat: I was not able to
get it to work with the `backtrace` crate.


### anyhow

`anyhow` appears to be really about stripping away the boilerplate. It does
a good job of this but leaves you in the position of having stringly typed
errors which does not fit my needs.