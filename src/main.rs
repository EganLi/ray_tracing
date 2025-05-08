use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use interval::Interval;
use ray::Ray;
use rtweekend::infinity;
use sphere::Sphere;
use vec3::{Color, Point};

mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();

    world.add(Sphere::new(&Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(&Point::new(0.0, -100.5, -1.0), 100.0));

    let mut cam = Camera::defalut();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world).unwrap();
}
