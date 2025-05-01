use std::{fs::File, io::Write};

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use rtweekend::infinity;
use sphere::Sphere;
use vec3::{Color, Point, Vec3, dot, unit_vector, write_color};

mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();

    let a = r.direction().length_squared();
    let h = dot(&r.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, infinity, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    // image_height=225
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let mut world = HittableList::default();
    world.add(Sphere::new(&Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(&Point::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    // viewport_width=1.125
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    // viewport_u=(2,0,0),viewport_v(0,1.125,0)

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    writeln!(file, "P3\n{} {}\n255\n", image_width, image_height)?;
    // println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&mut file, &pixel_color)?;
        }
    }
    print!("\rDone");

    Ok(())
}
