use crate::hittable::Hittable;
use crate::camera::Camera;
use xenon::color::Color;
use crate::ray::Ray;
use crate::random::with_rng;
use antsy::LoadingBar;
use std::fs::File;
use xenon::write::fn_to_png;

pub struct Renderer<W: Hittable, C: Camera> {
    world: W,
    camera: C,
    image_width: u32,
    aspect_ratio: f32,
    max_depth: u16,
    num_samples: u16,
}

impl<W: Hittable, C: Camera> Renderer<W, C> {
    pub fn new(world: W, camera: C) -> Self {
        Renderer {
            world,
            camera,
            image_width: 800,
            aspect_ratio: 16.0 / 9.0,
            max_depth: 50,
            num_samples: 100,
        }
    }

    pub fn width(self, image_width: u32) -> Self {
        Renderer {image_width, ..self}
    }

    pub fn aspect_ratio(self, aspect_ratio: f32) -> Self {
        Renderer {aspect_ratio, ..self}
    }

    pub fn render_to_file(self, filename: &str) {
        let file = File::create(filename).unwrap();
        let image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        let mut loadingbar = LoadingBar::new(image_height as u16).unwrap();

        fn_to_png(self.image_width, image_height, file, |i, j| {
            let mut color = Color::new(0.0, 0.0, 0.0);
            loadingbar.update((image_height - j) as u16).unwrap();
            for _ in 0..=self.num_samples {
                let u = (i as f32 + with_rng(rand::Rng::gen::<f32>)) / (self.image_width - 1) as f32;
                let v = (j as f32 + with_rng(rand::Rng::gen::<f32>)) / (image_height - 1) as f32;
                let r = self.camera.make_ray(u, v);
                color += ray_color(r, &self.world, self.max_depth);
            }
            color / self.num_samples as f32
        });
        loadingbar.finish().unwrap();
    }
}

fn ray_color(r: Ray, to_hit: &impl Hittable, depth: u16) -> Color {
    if depth == 0 {
        Color::new(0.0, 0.0, 0.0)
    } else if let Some(hit) = to_hit.intersect(&r, 0.00001, f32::INFINITY) {
        if let Some((scattered_ray, atten)) = hit.mat.scatter(&hit, r) {
            atten * ray_color(scattered_ray, to_hit, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_dir = r.d.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}


