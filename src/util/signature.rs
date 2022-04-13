/// Restricted parameter(s) of the closure for a machine
///
/// # Example
///
/// ```
/// pub enum Parameter {
///     Number(u8)
/// },
/// ```
///
/// ```
/// impl From<u8> for Parameter {
///     fn from(number: u8) -> Self {
///         Self::Number(number)
///     }
/// }
/// ```
///
/// This allows for machines to be specified with the parameter of the implemented type
/// ```
/// let machine: Machine<u8, ()> = |x: u8| -> () {}.into();
/// ```
pub enum Parameter {
    None, //  In this case, the only allowed parameters for the closure is equivalent to nothing (A single unit)
          //  With more variants, the less restricted the parameter(s) is/are
}

impl From<()> for Parameter {
    fn from(_: ()) -> Self {
        Self::None
    }
}

/// Restricted output(s) of the closure for a machine
///
/// # Example
///
/// ```
/// pub enum Output {
///     Number(u8)
/// },
/// ```
///
/// ```
/// impl From<u8> for Output {
///     fn from(number: u8) -> Self {
///         Self::Number(number)
///     }
/// }
/// ```
///
/// This allows for machines to be specified with the output of the implemented type
/// ```
/// let machine: Machine<(), u8> = |_| -> u8 { u8::MIN }.into();
/// ```
pub enum Output {
    None, //  In this case, the only allowed outputs for the closure is equivalent to nothing (A single unit)
          //  With more variants, the less restricted the output is
}

impl From<()> for Output {
    fn from(_: ()) -> Self {
        Self::None
    }
}
