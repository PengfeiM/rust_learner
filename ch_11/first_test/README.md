# Tests for Rust⚙

## Three steps to write tests
- Mock data
- Run code
- Assert the result

## try tests
```bash
❯ cargo test
   Compiling first_test v0.1.0 (/home/revan_m/codes/rust_begginer/rust_learner/ch_11/first_test)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests src/lib.rs (target/debug/deps/first_test-ffbd421014c93a53)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests first_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Rust built-in Assert
```rust
// three macros as assertions.

// bool type
assert!()
// assert equality
assert_qa!()
// assert unequable
assert_ne!()
```
