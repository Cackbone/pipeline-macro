/// A pipeline
///
/// # Generic parameters
/// * `I` - Input type
/// * `O` - Ouput type
pub struct Pipeline<I, O> {
    pub fun: fn(I) -> Result<O, &'static str>
}

impl<I, O> Pipeline<I, O> {
    /// Run this pipeline
    ///
    /// # Arguments
    ///
    /// * `input` - Input for this pipeline
    pub fn run(&self, input: I) -> Result<O, &'static str> {
        (self.fun)(input)
    }
}


// impl Parse for Pipeline {

// }
