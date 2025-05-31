use std::{
    f64::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    asset::RenderAssetUsages,
    platform::collections::HashMap,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use hexx::{Hex, HexLayout, HexOrientation, PlaneMeshBuilder, shapes::flat_rectangle};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Seedable, utils::ColorGradient};

#[derive(Debug, Resource)]
struct HexGrid {
    entities: HashMap<Hex, Entity>,
    layout: HexLayout,
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

fn get_noise() -> impl NoiseFn<f64, 3> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Fbm::<Perlin>::default()
        .set_seed(seed as u32)
        .set_lacunarity(2.01010101)
        .set_persistence(0.25)
        .set_octaves(8)
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout::flat()
        .with_hex_size(4.)
        .with_origin(Vec2::splat(-400.));

    let hex_rect = layout.rect_size();

    let mesh = meshes.add(hexagonal_plane(&layout));

    let noise = get_noise();

    let colours = ColorGradient::default().build_terrain_gradient();

    let w = 130.;
    let h = 80.;

    let angle_extent = 360.0;
    let height_extent = (2. * PI) * (h / w) * (hex_rect.x as f64 / hex_rect.y as f64);

    let x_step = angle_extent / w;
    let y_step = height_extent / h;

    let entities = flat_rectangle([1, w as i32, 1, h as i32])
        .map(|hex| {
            let [x, y] = hex.to_offset_coordinates(hexx::OffsetHexMode::Even, HexOrientation::Flat);

            let mut current_height = y_step * y as f64;
            let current_angle = x_step * x as f64;

            if y % 2 == 0 {
                current_height += y_step * 0.5;
            }

            let point_x = current_angle.to_radians().cos();
            let point_z = current_angle.to_radians().sin();

            let value = noise.get([point_x, current_height, point_z]);
            let [r, g, b, _] = colours.get_color(value);
            let material = materials.add(Color::srgb_u8(r, g, b));

            let pos = layout.hex_to_world_pos(hex);
            let entity = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .id();
            (hex, entity)
        })
        .collect();

    commands.insert_resource(HexGrid { entities, layout })
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .run();
}
