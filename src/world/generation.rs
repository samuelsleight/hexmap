use bevy::{
    asset::RenderAssetUsages,
    platform::collections::{HashMap, hash_map::Entry},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use hexx::{HexLayout, PlaneMeshBuilder};

use super::{WorldColumn, WorldLayout, WorldOrigin, WorldParams, WorldTiles};

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn generate_world(
    mut commands: Commands,
    params: Res<WorldParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let generated_world = hexmap_worldgen::generate_world(&hexmap_worldgen::WorldParams {
        width: params.width,
        height: params.height,
        scale_factor: params.scale_factor,
    });

    let world = WorldLayout {
        layout: generated_world.layout().clone().with_hex_size(6.),
        width: generated_world.width(),
        height: generated_world.height(),
    };

    let mut tiles = WorldTiles::default();

    let mut material_cache = HashMap::<[u8; 4], Handle<ColorMaterial>>::new();
    let mesh = meshes.add(hexagonal_plane(&world.layout));

    commands.spawn(WorldOrigin).with_children(|origin| {
        let columns = (1..=world.width)
            .map(|column| {
                let hex = world.hex(column, 0);

                origin
                    .spawn((
                        WorldColumn { column },
                        Transform::from_xyz(world.layout.hex_to_world_pos(hex).x, 0., 0.),
                    ))
                    .id()
            })
            .collect::<Vec<_>>();

        tiles.tiles = generated_world
            .tiles()
            .map(|(hex, &[r, g, b, a])| {
                let material = match material_cache.entry([r, g, b, a]) {
                    Entry::Occupied(material) => material.get().clone(),
                    Entry::Vacant(vacant) => vacant
                        .insert(materials.add(Color::srgb_u8(r, g, b)))
                        .clone(),
                };

                let [x, _] = world.hex_to_xy(hex);
                let pos = world.layout.hex_to_world_pos(hex);

                origin
                    .commands_mut()
                    .spawn((
                        Mesh2d(mesh.clone()),
                        MeshMaterial2d(material),
                        Transform::from_xyz(0., pos.y, 0.),
                        ChildOf(columns[x as usize - 1]),
                    ))
                    .id()
            })
            .collect()
    });

    commands.remove_resource::<WorldParams>();
    commands.insert_resource(tiles);
    commands.insert_resource(world);
}
