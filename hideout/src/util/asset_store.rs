use std::collections::HashMap;
use std::path::Path;

pub struct Asset {
    data: Vec<u8>,
    content_type: String,
}

#[derive(Default)]
pub struct AssetStore {
    inner: HashMap<String, Asset>,
}

#[derive(Debug)]
pub enum Error {
    Filename,
    Unicode,
    FileRead(std::io::Error),
}

impl AssetStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P: AsRef<Path>>(&mut self, path: P, content_type: &str) -> Result<(), Error> {
        let key = path
            .as_ref()
            .file_name()
            .ok_or(Error::Filename)?
            .to_str()
            .ok_or(Error::Unicode)?
            .to_string();
        let buf = std::fs::read(path).map_err(Error::FileRead)?;
        self.inner.insert(
            key,
            Asset {
                data: buf,
                content_type: content_type.to_string(),
            },
        );
        Ok(())
    }

    pub fn serve(&self, key: &str) -> Option<crate::http::Response> {
        self.inner.get(key).map(|asset| {
            use crate::http::Response;

            Response::binary(200, vec![], &asset.data, &asset.content_type)
        })
    }
}
