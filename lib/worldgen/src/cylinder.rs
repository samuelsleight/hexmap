use std::f64::consts::PI;

use hexx::{HexLayout, HexOrientation, OffsetHexMode, shapes::flat_rectangle};
use noise::NoiseFn;

pub struct CylindricalHexMapSampler {
    width: i32,
    height: i32,
    scale_factor: f64,
    layout: HexLayout,
}

impl CylindricalHexMapSampler {
    pub fn new(width: i32, height: i32, scale_factor: f64, layout: HexLayout) -> Self {
        Self {
            width,
            height,
            scale_factor,
            layout,
        }
    }

    pub fn height_extent(&self) -> f64 {
        let hex_rect = self.layout.rect_size();
        let scale = 1. / self.scale_factor;

        2. * PI
            * scale
            * (self.height as f64 / self.width as f64)
            * (hex_rect.x as f64 / hex_rect.y as f64)
    }

    pub fn x_step(&self) -> f64 {
        360.0 / self.width as f64
    }

    pub fn y_step(&self) -> f64 {
        self.height_extent() / self.height as f64
    }

    fn sample_xy_impl<Noise: NoiseFn<f64, 3>>(
        x: i32,
        y: i32,
        x_step: f64,
        y_step: f64,
        scale: f64,
        noise: &Noise,
    ) -> f64 {
        let mut current_height = y_step * y as f64;
        let current_angle = x_step * x as f64;

        if x % 2 == 0 {
            current_height += y_step * 0.5;
        }

        let point_x = current_angle.to_radians().cos() * scale;
        let point_z = current_angle.to_radians().sin() * scale;

        noise.get([point_x, current_height, point_z])
    }

    pub fn sample_xy<Noise: NoiseFn<f64, 3>>(&self, x: i32, y: i32, noise: &Noise) -> f64 {
        Self::sample_xy_impl(
            x,
            y,
            self.x_step(),
            self.y_step(),
            1. / self.scale_factor,
            noise,
        )
    }

    pub fn generate<Noise: NoiseFn<f64, 3>>(
        &self,
        noise: Noise,
    ) -> impl ExactSizeIterator<Item = f64> {
        let x_step = self.x_step();
        let y_step = self.y_step();
        let scale = 1. / self.scale_factor;

        flat_rectangle([1, self.width, 1, self.height]).map(move |hex| {
            let [x, y] = hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat);
            Self::sample_xy_impl(x, y, x_step, y_step, scale, &noise)
        })
    }
}
