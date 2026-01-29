# Error Handling

> [!ABSTRACT] to panic, or not to panic: that is the question.

like other language, Rust also has some little trick to deal with runtime errors.

Rust groups errors into 2 categories:
* recoverable errors
* unrecoverable errors

Accordingly, there are 2 ways to deal with 2 kind of errors.
* type: `Result<T, E>`
* macro: `panic!`


