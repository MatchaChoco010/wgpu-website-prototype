use std::path::{Path, PathBuf};

#[derive(Clone)]
pub(super) enum ResourcesCatalogItem {
    Mesh { hash: u64, path: PathBuf },
    Texture { hash: u64, path: PathBuf },
}
impl ResourcesCatalogItem {
    pub fn mesh(hash: u64, path: impl AsRef<Path>) -> Self {
        Self::Mesh {
            hash,
            path: PathBuf::from(path.as_ref()),
        }
    }

    pub fn texture(hash: u64, path: impl AsRef<Path>) -> Self {
        Self::Texture {
            hash,
            path: PathBuf::from(path.as_ref()),
        }
    }
}

#[derive(Clone)]
pub struct ResourcesCatalog {
    pub(super) items: Vec<ResourcesCatalogItem>,
}
impl ResourcesCatalog {
    pub(super) fn new(items: &[ResourcesCatalogItem]) -> Self {
        Self {
            items: Vec::from(items),
        }
    }
}
