use super::signature::{Output, Parameter};

/**
 * The immutable action (instructions) intended for a machine
 */
type Action<P, O> = Box<dyn FnOnce(P) -> O>;

/**
 * A machine with restrictions to the amount of instruction to be given
 */
#[non_exhaustive]
pub struct Machine<P, O>(Action<P, O>);

impl<P, O> Machine<P, O> {
    pub fn perform(self, p: P) -> O {
        self.0(p)
    }
}

/**
 * Limits `Machine` to the allowed variants for the coorespondong `Parameter` and `Output` of the provided closure
 * This is the only form of initiating a `Machine` because `Machine` is "non_exhaustive"
 */
impl<P, O, T> From<T> for Machine<P, O>
where
    P: Into<Parameter>,          // Restrict parameters
    O: Into<Output>,             // Restrict output
    T: FnOnce(P) -> O + 'static, // The closure itself - Given a static lifetime so that it will meet its required lifetime bounds
{
    fn from(c: T) -> Self {
        Self(Box::new(c))
    }
}
