//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Invalid(::alloc::boxed::Box<str>),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
