# Error Handling Exploration

## Best Practices

1. If a function can fail for only one reason, **do not** use an error enum. Rather, use an error struct.
1. If a function can fail for multiple reasons, create an error enum with a
struct-like variant for each reason.

## Goals

1. Internal implementation details should not leak into upper layers. (e.g., external-facing error types should not expose transitive dependencies on internal
errot types.)
    1. This should be accomplished by adding the `std::error::Error` trait
    as the `source` field for our struct-like error variants.
1. Error enum variants should curry the backtrace for logging purposes.
1. Error enums should be cloneable to support mocking for automated testing.
1. Error enum variants should include some sort of display macro. 