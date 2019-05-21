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

#![deny(missing_docs)]

use futures::future::{self, BoxFuture, FutureExt, TryFutureExt};
use std::collections::HashSet;
use tide::error::ResultExt;
use tide::http::header::{self, HeaderMap, HeaderValue};
use tide::middleware::{Middleware, Next};
use tide::response::IntoResponse;
use tide::{Context, Response};

mod error;
pub use error::Error;

#[cfg(test)]
mod tests;


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
        Box::pin(
            future::ready(self.validate_origin(cx.request().headers()))
                .and_then(|origin| {
                    next.run(cx).map(|mut res| {
                        res.headers_mut()
                            .append(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
                        Ok(res)
                    })
                })
                .map(|res| res.with_err_status(403).into_response()),
        )
    }
}
