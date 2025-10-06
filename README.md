# Try_Rust
This is a repository inspired by [aylei/leetcode-rust](https://github.com/aylei/leetcode-rust).

It is a repository of my trials at solving leetcode problems in Rust. Each module `p#_sss.rs` is a solution of the problem no. # on leetcode.

More additions can be made in the same format where the module contains the Solution struct and the tests for the said struct.

The tests are written in the inbuilt test infra of rust.

To run a test in the individual module use the following command:
```bash
cargo test p#_sss
```
Take note that the name of the module is without the .rs extention.

For running all the tests just run:
```bash
cargo test
```