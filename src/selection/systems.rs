use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};
use hexx::{HexLayout, InsetOptions, PlaneMeshBuilder};

use crate::{
    input::MousePosition,
    selection::{HoverIndicator, Indicator, SelectionIndicator},
    world::{OnHex, WorldLayout, WorldOrigin},
};

fn hexagonal_inset_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_inset_options(InsetOptions {
            keep_inner_face: false,
            scale: 0.05,
            ..Default::default()
        })
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

pub fn cleanup_indicators(mut commands: Commands, indicators: Query<Entity, With<Indicator>>) {
    for indicator in indicators {
        commands.entity(indicator).despawn();
    }
}

pub fn setup_indicators(
    mut commands: Commands,
    world: Res<WorldLayout>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add({
        let base_mesh = hexagonal_inset_plane(&world.layout);
        let vertices = base_mesh.count_vertices();
        base_mesh.with_inserted_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(vec![
                Color::srgb(0.8, 0.8, 0.8)
                    .to_linear()
                    .to_f32_array();
                vertices
            ]),
        )
    });

    let material = materials.add(ColorMaterial::default());

    commands.spawn((
        HoverIndicator,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        SelectionIndicator,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));
}

pub fn mouse_hover(
    position: Res<MousePosition>,
    world: Res<WorldLayout>,
    origin: Single<&Transform, With<WorldOrigin>>,
    indicator: Single<&mut OnHex, With<HoverIndicator>>,
) {
    let origin = origin.into_inner();
    indicator.into_inner().0 = Some(world.pick_tile(position.0, origin.translation.xy()));
}

pub fn mouse_press(
    mouse: Res<ButtonInput<MouseButton>>,
    hover: Single<&OnHex, With<HoverIndicator>>,
    select: Single<&mut OnHex, (With<SelectionIndicator>, Without<HoverIndicator>)>,
) {
    if mouse.just_released(MouseButton::Left) {
        let hovered = hover.into_inner();
        let mut current = select.into_inner();

        if current.0 != hovered.0 {
            current.0 = hovered.0;
        } else {
            current.0 = None;
        }
    }
}
