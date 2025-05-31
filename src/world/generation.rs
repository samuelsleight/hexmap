use std::{
    f64::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use hexx::{HexLayout, PlaneMeshBuilder, shapes::flat_rectangle};

use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Seedable, utils::ColorGradient};

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

fn get_noise() -> impl NoiseFn<f64, 3> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Fbm::<Perlin>::default()
        .set_seed(seed as u32)
        .set_lacunarity(2.01010101)
        .set_persistence(0.20)
        .set_octaves(8)
}

pub fn generate_world(
    mut commands: Commands,
    params: Res<WorldParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout::flat().with_hex_size(6.);

    let hex_rect = layout.rect_size();

    let mesh = meshes.add(hexagonal_plane(&layout));

    let noise = get_noise();

    let colours = ColorGradient::default().build_terrain_gradient();

    let w = params.width;
    let h = params.height;

    let angle_extent = 360.0;
    let height_extent = (2. * PI) * (h as f64 / w as f64) * (hex_rect.x as f64 / hex_rect.y as f64);

    let x_step = angle_extent / w as f64;
    let y_step = height_extent / h as f64;

    let world = WorldLayout {
        layout,
        width: w,
        height: h,
    };

    let mut tiles = WorldTiles::default();

    commands.spawn(WorldOrigin).with_children(|origin| {
        let columns = (1..=w)
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

        for hex in flat_rectangle([1, w, 1, h]) {
            let [x, y] = world.hex_to_xy(hex);

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

            let pos = world.layout.hex_to_world_pos(hex);

            let entity = origin
                .commands_mut()
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material),
                    Transform::from_xyz(0., pos.y, 0.),
                ))
                .id();

            origin
                .commands_mut()
                .entity(columns[x as usize - 1])
                .add_child(entity);

            tiles.tiles.push(entity);
        }
    });

    commands.remove_resource::<WorldParams>();
    commands.insert_resource(tiles);
    commands.insert_resource(world);
}
