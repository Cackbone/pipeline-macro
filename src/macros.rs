/// Helper to pass return of functions as parameter of other functions
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __pipeline_fn {
    (($fn:ident($($args:expr),*)), $ret:expr) => {
        $fn($ret $(,$arg)*);
    };
    ($fn:ident, $ret:expr) => {
        $fn($ret)
    };
}


/// Return new instance of pipeline struct
///
/// # Syntax:
/// ```
/// # use pipeline_macro::*;
/// # struct InputType {}
/// # struct OutputType {}
/// # fn function1(i: InputType) -> InputType { i }
/// # fn function2(i: InputType) -> OutputType { OutputType {} }
/// pipeline! {
///      InputType => function1 => function2 -> OutputType
/// };
/// ```
#[macro_export]
macro_rules! pipeline {
    ($in:ty => $($fns:ident) => * -> $out:ty) => {
        {{
            Pipeline { fun : (| input: $in | {
                let result = input;

                $(
                    let result = $crate::__pipeline_fn!($fns, result);
                )*
                    return result;
            })}
        }}
    };
}


