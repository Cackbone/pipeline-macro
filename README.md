pipeline-macro
==============

![crates.io](https://img.shields.io/crates/v/pipeline-macro.svg)

A crate to create a pipeline in Rust.


# How to use it ?

* Define a pipeline with type in input and type in output
* Use `run` method to run this pipeline

## Basic example:
```rust
    let pipeline = pipeline! {
        i32
        => add2
        => div_by_3
        => mul_by_83
        ;-> f64
    };

    let result = pipeline.run(3); // ~= 110.6666..
```


## Closure example:
```rust
    let pipeline = pipeline! {
        i32
        => |i: i32| i + 2
        => div_by_3
        => mul_by_83
        ;-> f64
    };

    let result = pipeline.run(3); // ~= 110.6666..
```



# How to build ?
```
    cargo build
```



# How to run tests ?
```
    cargo test
```

