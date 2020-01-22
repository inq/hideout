mod builder;

pub use builder::Builder;

pub struct Router {
    paths: Vec<(String, &'static dyn Fn(&str) -> ())>,
}

impl Router {
    pub(self) fn from_builder(paths: Vec<(String, &'static dyn Fn(&str) -> ())>) -> Self {
        Self { paths }
    }

    pub fn test_handle(&self, uri: &str) {
        self.paths[0].1(uri);
    }
}
