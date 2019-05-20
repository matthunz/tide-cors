use std::collections::HashSet;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _cors = Cors::default().allow_origin("example.com");
    }
}
