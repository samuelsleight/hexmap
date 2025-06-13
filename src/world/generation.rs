use bevy::{
    asset::RenderAssetUsages,
    platform::collections::{HashMap, hash_map::Entry},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use hexx::{HexLayout, PlaneMeshBuilder};

use hexmap_worldgen::TerrainType;

use crate::world::OnHex;

use super::{WorldColumn, WorldLayout, WorldOrigin, WorldParams, WorldTiles};

fn terrain_colour(colour: TerrainType) -> [u8; 3] {
    match colour {
        TerrainType::DeepOcean => [6, 58, 127],
        TerrainType::ShallowOcean => [14, 112, 192],
        TerrainType::Coast => [25, 150, 230],
        TerrainType::Beach => [210, 170, 110],
        TerrainType::Plains => [70, 120, 60],
        TerrainType::Hills => [110, 140, 100],
        TerrainType::LowMountains => [150, 150, 150],
        TerrainType::HighMountains => [220, 220, 200],
        TerrainType::Peaks => [250, 250, 250],
    }
}

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

    let mut material_cache = HashMap::<_, Handle<ColorMaterial>>::new();
    let mesh = meshes.add(hexagonal_plane(&world.layout));

    let origin = commands.spawn(WorldOrigin).id();

    let columns = (1..=world.width)
        .map(|column| {
            let hex = world.hex(column, 0);

            commands
                .spawn((
                    WorldColumn { column },
                    Transform::from_xyz(world.layout.hex_to_world_pos(hex).x, 0., 0.),
                    ChildOf(origin),
                ))
                .id()
        })
        .collect::<Vec<_>>();

    let tiles = generated_world
        .tiles()
        .map(|(hex, terrain)| {
            let colour = terrain_colour(terrain);
            let material = match material_cache.entry(colour) {
                Entry::Occupied(material) => material.get().clone(),
                Entry::Vacant(vacant) => vacant
                    .insert(materials.add(Color::srgb_u8(colour[0], colour[1], colour[2])))
                    .clone(),
            };

            let [x, _] = world.hex_to_xy(hex);
            let pos = world.layout.hex_to_world_pos(hex);

            commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material),
                    Transform::from_xyz(0., pos.y, 0.),
                    ChildOf(columns[x as usize - 1]),
                ))
                .id()
        })
        .collect();

    let settlement_material =
        materials.add(ColorMaterial::from_color(Color::srgb_u8(100, 50, 150)));

    let settlement_mesh = meshes.add(Rectangle::new(6., 6.));

    for hex in generated_world.generate_settlements() {
        commands.spawn((
            Mesh2d(settlement_mesh.clone()),
            MeshMaterial2d(settlement_material.clone()),
            OnHex(Some(hex)),
        ));
    }

    commands.remove_resource::<WorldParams>();
    commands.insert_resource(WorldTiles { tiles });
    commands.insert_resource(world);
}
