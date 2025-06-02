use std::f64::consts::PI;

use hexx::{HexLayout, HexOrientation, OffsetHexMode, shapes::flat_rectangle};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Seedable, utils::ColorGradient};
use rand::{Rng, rng};

pub use world::GeneratedWorld;

mod world;

#[derive(Debug, Clone, Copy)]
pub struct WorldParams {
    pub width: i32,
    pub height: i32,
}

fn get_noise_fn() -> impl NoiseFn<f64, 3> {
    Fbm::<Perlin>::default()
        .set_seed(rng().random())
        .set_lacunarity(1.91010101)
        .set_persistence(0.40)
        .set_octaves(18)
}

pub fn generate_world(params: &WorldParams) -> GeneratedWorld<[u8; 4]> {
    let layout = HexLayout::flat().with_hex_size(2.);
    let hex_rect = layout.rect_size();

    let angle_extent = 360.0;
    let height_extent = (2. * PI)
        * (params.height as f64 / params.width as f64)
        * (hex_rect.x as f64 / hex_rect.y as f64);

    let x_step = angle_extent / params.width as f64;
    let y_step = height_extent / params.height as f64;

    let noise = get_noise_fn();
    let colours = ColorGradient::default().build_terrain_gradient();

    let vec = flat_rectangle([1, params.width, 1, params.height])
        .map(|hex| {
            let [x, y] = hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat);

            let mut current_height = y_step * y as f64;
            let current_angle = x_step * x as f64;

            if x % 2 == 0 {
                current_height += y_step * 0.5;
            }

            let point_x = current_angle.to_radians().cos();
            let point_z = current_angle.to_radians().sin();

            let value = noise.get([point_x, current_height, point_z]);
            colours.get_color(value)
        })
        .collect();

    GeneratedWorld::new(params.width, params.height, layout, vec)
}
