#![feature(async_await)]

use futures::future::{BoxFuture, FutureExt};
use std::collections::HashSet;
use tide::http::StatusCode;
use tide::http::header::{self, HeaderMap};
use tide::{Context, Response};
use tide::response::IntoResponse;
use tide::middleware::{Middleware, Next};


#[derive(Debug)]
pub enum Error {
    MissingOrigin,
    OriginNotAllowed,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        ().with_status(StatusCode::FORBIDDEN).into_response()
    }
}

#[derive(Default)]
pub struct Cors {
    origins: Option<HashSet<&'static str>>,
}

impl Cors {
    pub fn allow_origin(&mut self, origin: &'static str) -> &mut Self {
        if let None = self.origins {
            self.origins = Some(HashSet::new());
        }
        self.origins.as_mut().unwrap().insert(origin);
        self
    }
    fn validate_origin(&self, headers: &HeaderMap) -> Result<(), Error> {
        if let Some(origins) = &self.origins {
            let origin = headers
                .get(header::ORIGIN)
                .and_then(|hdr| hdr.to_str().ok())
                .ok_or(Error::MissingOrigin)?;

            if origins.contains(origin) {
                Ok(())
            } else {
                Err(Error::OriginNotAllowed)
            }
        } else {
            Ok(())
        }
    }
}

impl<Data: 'static + Send + Sync> Middleware<Data> for Cors {
    fn handle<'a>(&'a self, cx: Context<Data>, next: Next<'a, Data>) -> BoxFuture<'a, Response> {
        match self.validate_origin(cx.request().headers()) {
            Ok(()) => next.run(cx),
            Err(e) => FutureExt::boxed(async { e.into_response() }),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _cors = Cors::default().allow_origin("example.com");
    }
}
