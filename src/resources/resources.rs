use std::{collections::HashMap, slice::SliceIndex};

use crate::resources::{Mesh, Texture};

pub struct Resources {
    meshes: HashMap<u64, Mesh>,
    textures: HashMap<u64, Texture>,
}
impl Resources {
    pub(super) fn new(meshes: HashMap<u64, Mesh>, textures: HashMap<u64, Texture>) -> Self {
        Self { meshes, textures }
    }

    pub fn mesh(&self, hash: u64) -> &Mesh {
        self.meshes
            .get(&hash)
            .unwrap_or_else(|| panic!("no such hash resource."))
    }

    pub fn texture(&self, hash: u64) -> &Texture {
        self.textures
            .get(&hash)
            .unwrap_or_else(|| panic!("no such hash resource."))
    }
}
