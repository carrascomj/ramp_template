# Ramp template
__Rust Adapted to My Projects__ template.

It includes: 
* Benchmarks boilerplate with [criterion.rs](https://github.com/bheisler/criterion.rs).
  ```
  cargo bench some_bench
  ```
* Github action CI: rustfmt, clippy and cargo test.
* Github action to synchronize API docs and README.md (from `src/lib.rs` and `./README.tpl`).
  ```
  cargo readme
  ```

