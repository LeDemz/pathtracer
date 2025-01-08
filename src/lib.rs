pub mod color;
mod ray;
mod vec3;

pub use color::write_color;
pub use color::Color;
pub use ray::Ray;
pub use vec3::{dot, unit_vector, Point3, Vec3};
