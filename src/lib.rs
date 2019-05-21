//! Cross-origin resource sharing (CORS) middleware for Tide applications
//!
//! # Example
//! ```
//! use std::io;
//! use tide::App;
//! use tide_cors::Cors;
//!
//! let mut app = App::new(());
//! app.middleware(
//!     Cors::default().allow_origin("https://www.rust-lang.org/")
//! );
//! ```

#![feature(async_await)]
#![deny(missing_docs)]

use futures::future::BoxFuture;
use std::collections::HashSet;
use tide::http::header::{self, HeaderMap, HeaderValue};
use tide::http::StatusCode;
use tide::middleware::{Middleware, Next};
use tide::response::IntoResponse;
use tide::{Context, Response};

#[cfg(test)]
mod tests;


/// Set of errors that can occur during processing CORS
#[derive(Debug)]
pub enum Error {
    /// The HTTP request header `Origin` is required but was not provided
    MissingOrigin,
    /// `Origin` is not allowed to make this request
    OriginNotAllowed,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        ().with_status(StatusCode::FORBIDDEN).into_response()
    }
}

/// CORS middleware struct
///
/// The wildcard origin (`"*"`) is used by default.
/// Calling `allow_origin` will enable a whitelist of origins instead.
#[derive(Default)]
pub struct Cors {
    origins: Option<HashSet<&'static str>>,
}

impl Cors {
    /// Add an origin that is allowed to make requests.
    /// Will be verified against the `Origin` request header.
    ///
    /// ```
    /// # use tide_cors::Cors;
    /// Cors::default()
    ///     .allow_origin("https://www.rust-lang.org/");
    /// ```
    pub fn allow_origin(mut self, origin: &'static str) -> Self {
        if self.origins.is_none() {
            self.origins = Some(HashSet::new());
        }
        self.origins.as_mut().unwrap().insert(origin);
        self
    }
    fn validate_origin(&self, headers: &HeaderMap) -> Result<HeaderValue, Error> {
        if let Some(origins) = &self.origins {
            let value = headers.get(header::ORIGIN);
            let origin = value
                .and_then(|hdr| hdr.to_str().ok())
                .ok_or(Error::MissingOrigin)?;

            if origins.contains(origin) {
                Ok(HeaderValue::from(value.unwrap()))
            } else {
                Err(Error::OriginNotAllowed)
            }
        } else {
            Ok(HeaderValue::from_str("*").unwrap())
        }
    }
}

impl<Data: 'static + Send + Sync> Middleware<Data> for Cors {
    fn handle<'a>(&'a self, cx: Context<Data>, next: Next<'a, Data>) -> BoxFuture<'a, Response> {
        match self.validate_origin(cx.request().headers()) {
            Ok(origin) => Box::pin(async {
                let mut res = next.run(cx).await;
                res.headers_mut()
                    .append(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
                res
            }),
            Err(e) => Box::pin(async { e.into_response() }),
        }
    }
}
