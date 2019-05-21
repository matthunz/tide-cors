use display_derive::Display;

/// Set of errors that can occur during processing CORS
#[derive(Display, Debug, PartialEq)]
pub enum Error {
    /// The HTTP request header `Origin` is required but was not provided
    #[display(fmt = "The HTTP request header `Origin` is required but was not provided")]
    MissingOrigin,
    /// `Origin` is not allowed to make this request
    #[display(fmt = "`Origin` is not allowed to make this request")]
    OriginNotAllowed,
}

impl std::error::Error for Error {}