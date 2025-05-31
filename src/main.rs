use std::{
    f64::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use hexx::{
    Hex, HexLayout, HexOrientation, OffsetHexMode, PlaneMeshBuilder, shapes::flat_rectangle,
};

use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Seedable, utils::ColorGradient};

#[derive(Component)]
#[require(InheritedVisibility)]
struct ViewportOffset;

#[derive(Component)]
#[require(InheritedVisibility)]
struct GridParent;

#[derive(Component)]
#[require(InheritedVisibility)]
struct ColumnParent {
    column: i32,
}

#[derive(Resource)]
struct GridParams {
    layout: HexLayout,
    width: i32,
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
        .set_persistence(0.20)
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
    let layout = HexLayout::flat().with_hex_size(6.);

    let hex_rect = layout.rect_size();

    let mesh = meshes.add(hexagonal_plane(&layout));

    let noise = get_noise();

    let colours = ColorGradient::default().build_terrain_gradient();

    let w = 130i32;
    let h = 80i32;

    let angle_extent = 360.0;
    let height_extent = (2. * PI) * (h as f64 / w as f64) * (hex_rect.x as f64 / hex_rect.y as f64);

    let x_step = angle_extent / w as f64;
    let y_step = height_extent / h as f64;

    commands
        .spawn((ViewportOffset, Transform::from_xyz(-550., -400., 0.)))
        .with_children(|viewport_transform| {
            viewport_transform
                .spawn((GridParent, Transform::from_xyz(0., 0., 0.)))
                .with_children(|grid_parent| {
                    let mut columns = Vec::new();

                    for column in 1..=w {
                        let hex = Hex::from_offset_coordinates(
                            [column, 0],
                            OffsetHexMode::Even,
                            HexOrientation::Flat,
                        );

                        columns.push(
                            grid_parent
                                .spawn((
                                    ColumnParent { column },
                                    Transform::from_xyz(layout.hex_to_world_pos(hex).x, 0., 0.),
                                ))
                                .id(),
                        );
                    }

                    for hex in flat_rectangle([1, w, 1, h]) {
                        let [x, y] =
                            hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat);

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
                        grid_parent
                            .commands_mut()
                            .entity(columns[x as usize - 1])
                            .with_child((
                                Mesh2d(mesh.clone()),
                                MeshMaterial2d(material),
                                Transform::from_xyz(0., pos.y, 0.),
                            ));
                    }
                });
        });

    commands.insert_resource(GridParams { layout, width: w });
}

fn scroll_grid(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid: Single<&mut Transform, With<GridParent>>,
) {
    let mut transform = grid.into_inner();

    let speed = 200. * time.delta_secs();

    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        transform.translation.y += speed;
    }

    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        transform.translation.y -= speed;
    }

    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        transform.translation.x += speed;
    }

    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        transform.translation.x -= speed;
    }
}

fn wrap_grid(
    grid_params: Res<GridParams>,
    grid: Single<&mut Transform, (With<GridParent>, Changed<Transform>)>,
    columns: Query<(&ColumnParent, &mut Transform), Without<GridParent>>,
) {
    let mut grid_transform = grid.into_inner();
    let layout = &grid_params.layout;

    // Compute the width of the world in pixel coordinates
    let world_width = layout
        .hex_to_world_pos(Hex::from_offset_coordinates(
            [grid_params.width, 0],
            OffsetHexMode::Even,
            HexOrientation::Flat,
        ))
        .x;

    // Ensure the grid offset is between 0 and the width of the world
    if grid_transform.translation.x < 0. {
        grid_transform.translation.x += world_width
    }

    if grid_transform.translation.x > world_width {
        grid_transform.translation.x -= world_width
    }

    // Compute the distance in hexes between the grid offset and the right edge of the world
    let hex_offset = grid_params.width
        - layout
            .world_pos_to_hex(grid_transform.translation.xy())
            .to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat)[0];

    // Wrap any column past the right edge of the world over to the left
    for (column, mut col_transform) in columns {
        let wrapped_column = if column.column > hex_offset {
            column.column - grid_params.width
        } else {
            column.column
        };

        let hex = Hex::from_offset_coordinates(
            [wrapped_column, 0],
            OffsetHexMode::Even,
            HexOrientation::Flat,
        );

        col_transform.translation.x = layout.hex_to_world_pos(hex).x;
    }
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
        .add_systems(Update, (scroll_grid, wrap_grid).chain())
        .run();
}
