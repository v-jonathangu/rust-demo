# Rust Demo

example of azp with rust, and unit testing

![Rust](https://github.com/ninjaoflight/rust-demo/workflows/Rust/badge.svg)

## Usage

``` sh
cargo run -- [numbers]
```

> example

``` sh
$ cargo run -- 2 4 6 8 -1
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target\debug\rust.exe 2 4 6 8 -1`
["target\\debug\\rust.exe", "2", "4", "6", "8", "-1"]
fib 2 = 1
fib 4 = 3
fib 6 = 8
fib 8 = 21
Error: -1 is a negative number
```

## Testing

``` sh
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.07s
     Running target\debug\deps\rust-9deb8861ae56aeed.exe

running 2 tests
test fib::tests::test_fibonacci ... ok
test fib::tests::test_negative_fail_fibonnacci ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
