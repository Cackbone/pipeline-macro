//! # How to use `pipeline_macro` ?
//!
//! * Define a pipeline with type in input and type in ouput
//! * Use `run` method to run this pipeline
//!
//! ## Basic example:
//! ```
//! use pipeline_macro::*;
//! # fn add2(i: i32) -> i32 { i + 2 }
//! # fn div_by_3(i: i32) -> f64 { (i as f64) / 3.0 }
//! # fn mul_by_83(i: f64) -> f32 { (i as f32) * 83.0 }
//!
//! # fn run_pipeline() -> Result<(), &'static str> {
//! let pipeline = pipeline! {
//!     i32
//!     => add2
//!     => div_by_3
//!     => mul_by_83
//!     ;-> f64
//! };
//!
//! let result = pipeline.run(3)?; // ~= 110.6666..
//! # Ok(())
//! # }
//! ```
//!
//! ## Closure example:
//! ```
//! use pipeline_macro::*;
//! # fn div_by_3(i: i32) -> f64 { (i as f64) / 3.0 }
//! # fn mul_by_83(i: f64) -> f32 { (i as f32) * 83.0 }
//!
//! # fn run_pipeline() -> Result<(), &'static str> {
//! let pipeline = pipeline! {
//!     i32
//!     => |i: i32| i + 2
//!     => div_by_3
//!     => mul_by_83
//!     ;-> f64
//! };
//!
//! let result = pipeline.run(3)?; // ~= 110.6666..
//! # Ok(())
//! # }
//! ```
mod pipeline;
mod macros;

pub use pipeline::Pipeline;


#[cfg(test)]
mod tests {

    use crate::*;

    fn add2(i: i32) -> i32 {
        i + 2
    }

    fn div_by_3(i: i32) -> f64 {
        (i as f64) / 3.0
    }

    fn mul_by_83(i: f64) -> f32 {
        (i as f32) * 83.0
    }

    #[test]
    fn basic_pipeline() -> Result<(), &'static str> {
        let pipeline = pipeline! {
            i32
            => add2
            => div_by_3
            => mul_by_83
            ;-> f32
        };
        let result = pipeline.run(2)?;

        assert_eq!(result, ((2 + 2) as f64 / 3.0) as f32 * 83.0);
        Ok(())
    }


    #[test]
    fn pipeline_with_closures() -> Result<(), &'static str> {
        let pipeline = pipeline! {
            i32
            => |i: i32| i + 2
            => div_by_3
            => mul_by_83
            ;-> f32
        };
        let result = pipeline.run(2)?;

        assert_eq!(result, ((2 + 2) as f64 / 3.0) as f32 * 83.0);
        return Ok(())
    }
}
