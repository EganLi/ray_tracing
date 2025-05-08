use rand::Rng;

pub const infinity: f64 = f64::INFINITY;
pub const pi: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}

pub fn random_double() -> f64 {
    random_double_range(0.0, 1.0)
}
