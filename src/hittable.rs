use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{self, Point, Vec3, dot},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal.assign(outward_normal);
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    // fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
