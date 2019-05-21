use std::io;
use tide::App;
use tide_cors::Cors;

fn main() -> io::Result<()> {
    let mut app = App::new(());
    app.middleware(Cors::default().allow_origin("https://www.rust-lang.org/"));
    app.serve("localhost:8000")
}
