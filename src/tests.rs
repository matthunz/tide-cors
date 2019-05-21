use crate::Cors;
use futures::executor::block_on;
use http_service::{Body, HttpService};
use tide::http::{header, Request};
use tide::{App, Server};

fn request(service: &Server<()>, req: http_service::Request) -> http_service::Response {
    block_on(service.respond(&mut (), req)).unwrap()
}

#[test]
fn validates_origin() {
    let origin = "foo";
    let mut app = App::new(());
    app.middleware(Cors::default().allow_origin(&origin));
    let service = app.into_http_service();

    let invalid = Request::new(Body::empty());
    assert_eq!(403, request(&service, invalid).status());

    let valid = Request::builder()
        .header(header::ORIGIN, origin)
        .body(Body::empty())
        .unwrap();
    assert_eq!(404, request(&service, valid).status());
}
