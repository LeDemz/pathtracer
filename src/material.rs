use crate::{vec3::{random_unit_vector, reflect}, Color, HitRecord, Ray};



pub trait Material {
    fn scatter (&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool;
}

pub struct DefaultMaterial {
    albedo : Color,
}

impl DefaultMaterial {
    pub fn new() -> Self {
        DefaultMaterial {
            albedo : Color::new(0.5, 0.5, 0.5),
        }
    }
}

impl Material for DefaultMaterial {
    fn scatter (&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        return false;
    }
}

pub struct Lambertian {
    albedo : Color,
}

impl Lambertian {
    pub fn new(albedo : Color) -> Self {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter (&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo : Color,
}

impl Metal {
    pub fn new(albedo : Color) -> Self {
        Metal {
            albedo,
        }
    }
}

impl Material for Metal {
    fn scatter (&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        let reflected = reflect(r_in.direction(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}