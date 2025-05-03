use crate::vec3;
use crate::vec3::Point;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(ori: &Point, dir: &Vec3) -> Ray {
        Ray {
            orig: Point::new(ori.x(), ori.y(), ori.z()),
            dir: Vec3::new(dir.x(), dir.y(), dir.z()),
        }
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn origin_mut(&mut self) -> &mut Point {
        &mut self.orig
    }

    pub fn direction_mut(&mut self) -> &mut Vec3 {
        &mut self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * &self.dir
    }
}
