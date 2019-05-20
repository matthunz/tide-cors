#![feature(async_await)]

use futures::future::BoxFuture;
use std::collections::HashSet;
use tide::http::header::{self, HeaderMap, HeaderValue};
use tide::http::StatusCode;
use tide::middleware::{Middleware, Next};
use tide::response::IntoResponse;
use tide::{Context, Response};


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
                res.headers_mut().append(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
                res
            }),
            Err(e) => Box::pin(async { e.into_response() }),
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
