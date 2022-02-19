use instant::Duration;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, sync::mpsc};
use thiserror::Error;

use crate::resources::resources_catalog::ResourcesCatalogItem;
use crate::resources::{Mesh, Resources, ResourcesCatalog, Texture};

#[derive(Debug, Error)]
pub enum ResourceLoaderError {
    #[error("Load resources not completed")]
    LoadNotCompletedError,
    #[error("Already take resources")]
    AlreadyTakeResourcesError,
}

pub struct ResourcesLoader {
    take_flag: bool,
    loaded_flag: Arc<AtomicBool>,
    loaded_count: Arc<AtomicU32>,
    all_count: usize,
    loaded_meshes: Arc<Mutex<Option<HashMap<u64, Mesh>>>>,
    loaded_textures: Arc<Mutex<Option<HashMap<u64, Texture>>>>,
}
impl ResourcesLoader {
    pub fn start_load(catalog: ResourcesCatalog) -> Self {
        let all_count = catalog.items.len();
        let loaded_flag = Arc::new(AtomicBool::new(false));
        let loaded_count = Arc::new(AtomicU32::new(0));
        let loaded_meshes = Arc::new(Mutex::new(Some(HashMap::new())));
        let loaded_textures = Arc::new(Mutex::new(Some(HashMap::new())));

        #[cfg(not(target_arch = "wasm32"))]
        {
            tokio::spawn(Self::load(
                catalog.clone(),
                Arc::clone(&loaded_flag),
                Arc::clone(&loaded_count),
                all_count,
                Arc::clone(&loaded_meshes),
                Arc::clone(&loaded_textures),
            ));
        }

        Self {
            take_flag: false,
            loaded_flag,
            loaded_count,
            all_count,
            loaded_meshes,
            loaded_textures,
        }
    }

    async fn load(
        catalog: ResourcesCatalog,
        loaded_flag: Arc<AtomicBool>,
        loaded_count: Arc<AtomicU32>,
        all_count: usize,
        loaded_meshes: Arc<Mutex<Option<HashMap<u64, Mesh>>>>,
        loaded_textures: Arc<Mutex<Option<HashMap<u64, Texture>>>>,
    ) {
        let (mesh_sender, mesh_receiver) = mpsc::channel();
        let (texture_sender, texture_receiver) = mpsc::channel();
        let tasks = catalog
            .items
            .iter()
            .map(|item| match item.clone() {
                ResourcesCatalogItem::Mesh { hash, path } => {
                    Box::pin(Self::load_mesh(mesh_sender.clone(), hash, path))
                        as Pin<Box<dyn Future<Output = ()> + Send>>
                }
                ResourcesCatalogItem::Texture { hash, path } => {
                    Box::pin(Self::load_texture(texture_sender.clone(), hash, path))
                        as Pin<Box<dyn Future<Output = ()> + Send>>
                }
            })
            .collect::<Vec<Pin<Box<dyn Future<Output = ()> + Send>>>>();

        #[cfg(not(target_arch = "wasm32"))]
        {
            tokio::spawn(async {
                futures::future::join_all(tasks).await;
            });
            tokio::task::yield_now().await;
        }

        while loaded_count.load(Ordering::SeqCst) < all_count as u32 {
            for mesh in mesh_receiver.try_iter() {
                loaded_meshes
                    .lock()
                    .unwrap()
                    .as_mut()
                    .and_then(|hm| hm.insert(mesh.hash, mesh));
                loaded_count.fetch_add(1, Ordering::SeqCst);
            }
            for texture in texture_receiver.try_iter() {
                loaded_textures
                    .lock()
                    .unwrap()
                    .as_mut()
                    .and_then(|hm| hm.insert(texture.hash, texture));
                loaded_count.fetch_add(1, Ordering::SeqCst);
            }
        }

        tokio::time::sleep(Duration::from_secs_f32(0.3)).await;

        loaded_flag.store(true, Ordering::SeqCst);
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_mesh(mesh_sender: Sender<Mesh>, hash: u64, path: impl AsRef<Path>) {
        // tokio::time::sleep(Duration::from_secs_f32(1.2)).await;

        mesh_sender
            .send(Mesh { hash })
            .unwrap_or_else(|_| panic!("Failed to send loaded mesh."))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_texture(texture_sender: Sender<Texture>, hash: u64, path: impl AsRef<Path>) {
        use image::GenericImageView;

        tokio::time::sleep(Duration::from_secs_f32(1.2)).await;

        let img = image::open(path).unwrap_or_else(|_| panic!("Failed to load image."));
        let dimensions = img.dimensions();
        let rgba: &[u8] = &img.to_rgba8();
        texture_sender
            .send(Texture {
                hash,
                rgba: Vec::from(rgba),
                height: dimensions.0,
                width: dimensions.1,
            })
            .unwrap_or_else(|_| panic!("Failed to send loaded mesh."))
    }

    pub fn all_count(&self) -> usize {
        self.all_count
    }

    pub fn loaded_count(&self) -> usize {
        self.loaded_count.load(Ordering::SeqCst) as usize
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded_flag.load(Ordering::SeqCst)
    }

    pub fn progress(&self) -> f32 {
        self.loaded_count() as f32 / self.all_count() as f32
    }

    pub fn take_flag(&self) -> bool {
        self.take_flag
    }

    pub fn take_resources(&mut self) -> Result<Resources, ResourceLoaderError> {
        if self.loaded_flag.load(Ordering::SeqCst) {
            if !self.take_flag() {
                self.take_flag = true;
                Ok(Resources::new(
                    self.loaded_meshes.lock().unwrap().take().unwrap(),
                    self.loaded_textures.lock().unwrap().take().unwrap(),
                ))
            } else {
                Err(ResourceLoaderError::AlreadyTakeResourcesError)
            }
        } else {
            Err(ResourceLoaderError::LoadNotCompletedError)
        }
    }
}
