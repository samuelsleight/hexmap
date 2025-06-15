use std::{collections::VecDeque, num::NonZero};

use bevy::{
    asset::RenderAssetUsages,
    platform::collections::{HashMap, hash_map::Entry},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};

use hexx::{HexLayout, PlaneMeshBuilder};
use rand::{Rng, rng};

use hexmap_worldgen::{
    settlements::{self, SettlementParams},
    terrain::{self, TerrainParams, TerrainType},
};

use crate::world::{OnHex, ZoneHighlight};

use super::{WorldColumn, WorldLayout, WorldOrigin, WorldParams, WorldTiles};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ClosestZone {
    zone: usize,
    cost: NonZero<usize>,
}

impl ClosestZone {
    fn new(zone: usize, cost: NonZero<usize>) -> Self {
        Self { zone, cost }
    }
}

fn terrain_colour(colour: TerrainType) -> [u8; 4] {
    match colour {
        TerrainType::DeepOcean => [6, 58, 127, 255],
        TerrainType::ShallowOcean => [14, 112, 192, 255],
        TerrainType::Coast => [25, 150, 230, 255],
        TerrainType::Beach => [210, 170, 110, 255],
        TerrainType::Plains => [70, 120, 60, 255],
        TerrainType::Hills => [110, 140, 100, 255],
        TerrainType::LowMountains => [150, 150, 150, 255],
        TerrainType::HighMountains => [220, 220, 200, 255],
        TerrainType::Peaks => [250, 250, 250, 255],
    }
}

fn terrain_zone_cost(terrain: TerrainType) -> usize {
    match terrain {
        TerrainType::DeepOcean => 500,
        TerrainType::ShallowOcean => 100,
        TerrainType::Coast => 50,
        TerrainType::Beach => 1,
        TerrainType::Plains => 1,
        TerrainType::Hills => 5,
        TerrainType::LowMountains => 100,
        TerrainType::HighMountains => 500,
        TerrainType::Peaks => 1000,
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
    let generated_terrain = terrain::generate(TerrainParams::new(
        params.width,
        params.height,
        params.scale_factor,
    ));

    let world = WorldLayout {
        layout: generated_terrain.layout().clone().with_hex_size(6.),
        width: generated_terrain.width(),
        height: generated_terrain.height(),
    };

    let material = materials.add(ColorMaterial::default());

    let base_mesh = hexagonal_plane(&world.layout);

    let mut mesh_cache = HashMap::<_, Handle<Mesh>>::new();

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

    let tiles = generated_terrain
        .tiles()
        .map(|(hex, terrain)| {
            let colour = terrain_colour(terrain);
            let mesh = match mesh_cache.entry(colour) {
                Entry::Occupied(mesh) => mesh.get().clone(),
                Entry::Vacant(vacant) => vacant
                    .insert({
                        let colour = Color::srgb_u8(colour[0], colour[1], colour[2]);
                        let mut mesh = base_mesh.clone();
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_COLOR,
                            VertexAttributeValues::Float32x4(vec![
                                colour.to_linear().to_f32_array();
                                6
                            ]),
                        );
                        meshes.add(mesh)
                    })
                    .clone(),
            };

            let [x, _] = world.hex_to_xy(hex);
            let pos = world.layout.hex_to_world_pos(hex);

            commands
                .spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(0., pos.y, 0.),
                    ChildOf(columns[x as usize - 1]),
                ))
                .id()
        })
        .collect();

    let terrain = generated_terrain.tiles().collect::<HashMap<_, _>>();

    let settlement_material =
        materials.add(ColorMaterial::from_color(Color::srgb_u8(100, 50, 150)));

    let settlement_mesh = meshes.add(Rectangle::new(6., 6.));

    let settlements =
        settlements::generate(&generated_terrain, SettlementParams::new(rng().random()))
            .collect::<VecDeque<_>>();

    let zone_colours = settlements
        .iter()
        .map(|_| [rng().random(), rng().random(), rng().random(), 150])
        .collect::<Vec<[u8; 4]>>();

    let mut closest_zones = settlements
        .iter()
        .enumerate()
        .map(|(index, hex)| (*hex, ClosestZone::new(index, NonZero::new(1).unwrap())))
        .collect::<HashMap<_, _>>();

    for hex in settlements.iter() {
        let mut hex = *hex;
        hex.x -= 1;
        hex.y -= 1;

        commands.spawn((
            Transform::from_xyz(0., 0., 3.),
            Mesh2d(settlement_mesh.clone()),
            MeshMaterial2d(settlement_material.clone()),
            OnHex(Some(hex)),
        ));
    }

    let mut frontier = settlements;

    let cost_fn = |from, to| {
        let next_cost =
            if let Some(cost) = terrain.get(&to).map(|terrain| terrain_zone_cost(*terrain)) {
                cost
            } else {
                return None;
            };

        let this_cost = terrain_zone_cost(*terrain.get(&from).unwrap());

        Some(if next_cost > this_cost {
            next_cost * 2
        } else {
            next_cost / 2
        })
    };

    while let Some(hex) = frontier.pop_front() {
        let current = *closest_zones.get(&hex).unwrap();

        for neighbour in hex.all_neighbors() {
            let neighbour = world.wrap(neighbour);

            let cost = if let Some(cost) = cost_fn(hex, neighbour) {
                cost
            } else {
                continue;
            };

            let neighbour_cost = NonZero::new(current.cost.get() + cost).unwrap();

            match closest_zones.entry(neighbour) {
                Entry::Occupied(entry) => {
                    let existing_cost = entry.get().cost;

                    if neighbour_cost < existing_cost {
                        entry.replace_entry_with(|_, _| {
                            Some(ClosestZone::new(current.zone, neighbour_cost))
                        });
                        frontier.push_back(neighbour);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(ClosestZone::new(current.zone, neighbour_cost));
                    frontier.push_back(neighbour);
                }
            }
        }
    }

    for (mut hex, zone) in closest_zones {
        hex.x -= 1;
        hex.y -= 1;

        let colour = zone_colours[zone.zone];
        let mesh = match mesh_cache.entry(colour) {
            Entry::Occupied(mesh) => mesh.get().clone(),
            Entry::Vacant(vacant) => vacant
                .insert({
                    let colour = Color::srgba_u8(colour[0], colour[1], colour[2], colour[3]);
                    let mut mesh = base_mesh.clone();
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_COLOR,
                        VertexAttributeValues::Float32x4(vec![colour.to_srgba().to_f32_array(); 6]),
                    );
                    meshes.add(mesh)
                })
                .clone(),
        };

        commands.spawn((
            Transform::from_xyz(0., 0., 5.),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            OnHex(Some(hex)),
            ZoneHighlight,
        ));
    }

    commands.remove_resource::<WorldParams>();
    commands.insert_resource(WorldTiles { tiles });
    commands.insert_resource(world);
}
