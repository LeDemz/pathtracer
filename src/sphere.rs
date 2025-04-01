use crate::{dot, HitRecord, Hittable, Interval, Material, Point3, Ray};
use std::rc::Rc;
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit_tmin_tmax(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if (root <= ray_tmin) || (ray_tmax <= root) {
            root = (h + sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }

    fn hit_interval(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
