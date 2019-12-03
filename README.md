# Error Handling Exploration

## Best Practices

1. If a function can fail for only one reason, **do not** use an error enum. Rather, use an error struct.
1. If a function can fail for multiple reasons, create an error enum with a
struct-like variant for each reason.

## Goals

1. Internal implementation details should not leak into upper layers. (e.g., external-facing error types should not expose transitive dependencies on internal
error types.)
    1. This should be accomplished by adding the `std::error::Error` trait
    as the `source` field for our struct-like error variants.
1. Error enum variants should curry the backtrace for logging purposes.
1. Error enums should be cloneable to support mocking for automated testing.
1. Error enum variants should include some sort of display macro.

## Libraries Evaluated

* [snafu](https://docs.rs/snafu/0.6.0/snafu/index.html)
* err-derive
* quick-errors

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

### err-derive

`err-derive` is a port of the `failure` crate except
it's designed to work with `std::error::Error`. This is
good in that in using it we'd depend only on the standard
library. This is bad in that the standard library does not
support `Backtrace` except on nightly.

`err-derive` also lacks macros that let us auto-implement 
`From` implementations for our source fields.