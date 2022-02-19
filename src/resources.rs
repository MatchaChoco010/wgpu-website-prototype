mod mesh;
mod resources;
mod resources_catalog;
mod resources_loader;
mod texture;

pub use mesh::Mesh;
pub use resources::Resources;
pub use resources_catalog::ResourcesCatalog;
use resources_catalog::ResourcesCatalogItem;
pub use resources_loader::ResourcesLoader;
pub use texture::Texture;

pub fn get_catalog() -> ResourcesCatalog {
    ResourcesCatalog::new(&[
        ResourcesCatalogItem::mesh(hash("room"), "assets/room.glb"),
        ResourcesCatalogItem::texture(
            hash("room_tex_base_color"),
            "assets/room_Material_BaseColor.png",
        ),
        ResourcesCatalogItem::texture(
            hash("room_tex_metallic"),
            "assets/room_Material_Metallic.png",
        ),
        ResourcesCatalogItem::texture(hash("room_tex_normal"), "assets/room_Material_Normal.png"),
        ResourcesCatalogItem::texture(
            hash("room_tex_roughness"),
            "assets/room_Material_Roughness.png",
        ),
    ])
}

pub const fn hash(name: &str) -> u64 {
    const_fnv1a_hash::fnv1a_hash_str_64(name)
}
