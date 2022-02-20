use image::GenericImageView;
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

#[derive(Debug)]
pub struct ResourcesLoader {
    take_flag: bool,
    loaded_flag: Arc<AtomicBool>,
    loaded_count: Arc<AtomicU32>,
    all_count: usize,
    loaded_meshes: Arc<Mutex<Option<HashMap<u64, Mesh>>>>,
    loaded_textures: Arc<Mutex<Option<HashMap<u64, Texture>>>>,
}
impl ResourcesLoader {
    pub fn new() -> Self {
        let all_count = 0;
        let loaded_flag = Arc::new(AtomicBool::new(false));
        let loaded_count = Arc::new(AtomicU32::new(0));
        let loaded_meshes = Arc::new(Mutex::new(Some(HashMap::new())));
        let loaded_textures = Arc::new(Mutex::new(Some(HashMap::new())));

        Self {
            take_flag: false,
            loaded_flag,
            loaded_count,
            all_count,
            loaded_meshes,
            loaded_textures,
        }
    }

    pub fn start_load(&mut self, catalog: ResourcesCatalog) {
        self.all_count = catalog.items.len();

        #[cfg(not(target_arch = "wasm32"))]
        {
            tokio::spawn(Self::load(
                catalog.clone(),
                Arc::clone(&self.loaded_flag),
                Arc::clone(&self.loaded_count),
                self.all_count,
                Arc::clone(&self.loaded_meshes),
                Arc::clone(&self.loaded_textures),
            ));
        }

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(Self::load(
                catalog.clone(),
                Arc::clone(&self.loaded_flag),
                Arc::clone(&self.loaded_count),
                self.all_count,
                Arc::clone(&self.loaded_meshes),
                Arc::clone(&self.loaded_textures),
            ));
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
        let (mesh_sender, mesh_receiver) = mpsc::channel::<Mesh>();
        let (texture_sender, texture_receiver) = mpsc::channel::<Texture>();

        #[cfg(not(target_arch = "wasm32"))]
        {
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
            tokio::spawn(async {
                futures::future::join_all(tasks).await;
            });
            tokio::task::yield_now().await;
        }

        #[cfg(target_arch = "wasm32")]
        {
            let tasks = catalog
                .items
                .iter()
                .map(|item| match item.clone() {
                    ResourcesCatalogItem::Mesh { hash, path } => {
                        Box::pin(Self::load_mesh(mesh_sender.clone(), hash, path))
                            as Pin<Box<dyn Future<Output = ()>>>
                    }
                    ResourcesCatalogItem::Texture { hash, path } => {
                        Box::pin(Self::load_texture(texture_sender.clone(), hash, path))
                            as Pin<Box<dyn Future<Output = ()>>>
                    }
                })
                .collect::<Vec<Pin<Box<dyn Future<Output = ()>>>>>();
            wasm_bindgen_futures::spawn_local(async {
                futures::future::join_all(tasks).await;
            });
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

            #[cfg(target_arch = "wasm32")]
            {
                let _ = wasm_timer::Delay::new(Duration::ZERO).await;
                log::debug!("loading loop.")
            }
        }

        // tokio::time::sleep(Duration::from_secs_f32(0.3)).await;

        loaded_flag.store(true, Ordering::SeqCst);
    }

    // #[cfg(not(target_arch = "wasm32"))]
    async fn load_mesh(mesh_sender: Sender<Mesh>, hash: u64, path: impl AsRef<Path>) {
        mesh_sender
            .send(Mesh { hash })
            .unwrap_or_else(|_| panic!("Failed to send loaded mesh."))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_texture(texture_sender: Sender<Texture>, hash: u64, path: impl AsRef<Path>) {
        use std::env;
        use std::path::PathBuf;

        let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut cargo_dir = PathBuf::from(manifest_dir);
            cargo_dir.push(path);
            cargo_dir.as_path().to_owned()
        } else {
            path.as_ref().to_owned()
        };

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

    #[cfg(target_arch = "wasm32")]
    async fn load_texture(texture_sender: Sender<Texture>, hash: u64, path: impl AsRef<Path>) {
        use futures::StreamExt;
        use js_sys::Uint8Array;
        use wasm_bindgen::{prelude::*, JsCast};
        use wasm_bindgen_futures::JsFuture;
        use wasm_streams::readable::ReadableStream;
        use web_sys::{Blob, Request, RequestInit, RequestMode, Response};

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(path.as_ref().to_str().unwrap(), &opts)
            .unwrap_or_else(|_| panic!("Failed to create request."));

        let window = web_sys::window().unwrap_or_else(|| panic!("Failed to load html window."));
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap_or_else(|_| panic!("Failed to fetch request."));

        let resp: Response = resp_value
            .dyn_into()
            .unwrap_or_else(|_| panic!("Failed to cast Response"));
        let blob: Blob =
            JsFuture::from(resp.blob().unwrap_or_else(|_| panic!("Failed to get Blob")))
                .await
                .unwrap_or_else(|_| panic!("Failed to unwrap JsSuture"))
                .dyn_into()
                .unwrap_or_else(|_| panic!("Failed to cast Blob"));
        let readable_stream = blob.stream().dyn_into().unwrap_throw();
        let mut stream = ReadableStream::from_raw(readable_stream).into_stream();
        let mut bytes = vec![];
        while let Some(Ok(chunk)) = stream.next().await {
            let uint8_array = Uint8Array::new(&chunk);
            bytes.append(&mut uint8_array.to_vec());
        }
        log::error!("length: {}", bytes.len());

        let img = image::load_from_memory(&bytes).unwrap();
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
