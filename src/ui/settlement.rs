use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
    render::mesh::VertexAttributeValues,
    text::TextLayoutInfo,
};

use crate::{camera::RenderOrder, world::WorldLayout};

#[derive(Clone, Component)]
#[component(on_insert = on_insert)]
pub struct SettlementUi(pub String);

#[derive(Resource)]
struct BannerMesh(Handle<Mesh>);

#[derive(Resource)]
struct BannerMaterial(Handle<ColorMaterial>);

fn on_insert(mut world: DeferredWorld, context: HookContext) {
    let component = world
        .entity(context.entity)
        .get::<SettlementUi>()
        .unwrap()
        .clone();

    let scale = world.resource::<WorldLayout>().layout.scale;

    world.commands().spawn((
        RenderOrder::WorldUi,
        TextFont::default().with_font_size(64.),
        TextColor::WHITE,
        Text2d(component.0),
        ChildOf(context.entity),
        Transform::from_scale(Vec3::splat(0.04)).with_translation(Vec3::new(
            0.,
            scale.x - 1.01,
            0.,
        )),
    ));
}

fn on_text_updated(
    mut commands: Commands,
    mesh: Res<BannerMesh>,
    material: Res<BannerMaterial>,
    query: Query<(Entity, &TextLayoutInfo), Changed<TextLayoutInfo>>,
) {
    for (entity, layout) in query {
        commands.spawn((
            ChildOf(entity),
            Transform::from_scale(layout.size.extend(1.) * 1.05)
                .with_translation(Vec3::new(0., 0., -1.)),
            Mesh2d(mesh.0.clone()),
            MeshMaterial2d(material.0.clone()),
        ));
    }
}

fn init_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Rectangle::from_length(1.).mesh().build();
    let vertices = mesh.count_vertices();

    commands.insert_resource(BannerMesh(meshes.add(mesh.with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(vec![
            Color::srgba(0.2, 0.2, 0.2, 0.05).to_linear().to_f32_array();
            vertices
        ]),
    ))));

    commands.insert_resource(BannerMaterial(materials.add(ColorMaterial::default())));
}

pub fn register(app: &mut App) {
    app.add_systems(Startup, init_resources)
        .add_systems(Update, on_text_updated);
}
