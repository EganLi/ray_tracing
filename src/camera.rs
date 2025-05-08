use crate::Color;
use crate::HitRecord;
use crate::Hittable;
use crate::Interval;
use crate::Ray;
use crate::infinity;
use crate::rtweekend::random_double;
use crate::vec3::{Point, Vec3, unit_vector, write_color};
use std::{fs::File, io::Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn defalut() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 0,
            center: Point::default(),
            pixel00_loc: Point::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    pub fn render<T: Hittable>(&mut self, world: &T) -> std::io::Result<()> {
        self.initialize();

        let mut file = File::create("image.ppm")?;
        // Render
        writeln!(
            file,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;
        // println!("P3\n{} {}\n255\n", image_width, image_height);

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", (self.image_height - j));
            for i in 0..self.image_width {
                /*
                let pixel_center =
                    self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(&self.center, &ray_direction);

                let pixel_color = self.ray_color(&r, world);
                write_color(&mut file, &pixel_color)?;
                */
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                write_color(&mut file, &pixel_color, self.samples_per_pixel)?;
            }
        }
        print!("\rDone");

        Ok(())
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, world: &T) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, infinity), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(*r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
