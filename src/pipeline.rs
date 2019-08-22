/// A pipeline
///
/// # Generic parameters
/// * `I` - Input type
/// * `O` - Ouput type
pub struct Pipeline<I, O> {
    pub fun: fn(I) -> O
}

impl<I, O> Pipeline<I, O> {
    /// Run this pipeline
    ///
    /// # Arguments
    ///
    /// * `input` - Input for this pipeline
    pub fn run(&self, input: I) -> O {
        (self.fun)(input)
    }
}
