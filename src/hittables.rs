use crate::vec::{Vec3, Point3, Color, NEAR_ZERO};
use crate::ray::Ray;
use crate::texture::Texture;

pub type HittableVec = Vec<Box<dyn Hittable>>;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box< dyn Material>
}

pub struct Plane {
    pub y: f64,
    pub material: Box<dyn Material >
}

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub hit_material: &'a dyn Material,
    pub t: f64,
}

impl HitRecord<'_> {
    pub fn on_hit(&self, ray_in: &Ray) -> (Ray,Color) {
        return self.hit_material.scatter(&self, ray_in);
    }
}

#[derive(Copy, Clone)]
pub struct Diffuse {
    pub attenuation: Color
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub attenuation: Color
}

pub struct TexMat {
    // pub specular_map: Tex,
    pub texture: Texture
    // pub
}

fn is_front_facing(ray: &Ray, outward_normal: Vec3) -> bool {
    return Vec3::dot(ray.dir, outward_normal).is_sign_negative();
}

pub trait Material {
    fn scatter(&self, record: &HitRecord, ray_in: &Ray) -> (Ray, Color);
}

impl Material for TexMat {
    fn scatter(&self, record: &HitRecord, _: &Ray) -> (Ray, Color) {
        //let target = record.p + record.normal + Vec3::random_unit();
        //let x = record.p.x;
        //let y = record.p.y;
        let img = &self.texture.image;
        let (x_b, y_b) = img.dimensions();
        let target = record.p + record.normal + Vec3::random_unit();
        let x = (record.p.x * 500.0 ) as i32 % x_b as i32;
        let y = (record.p.z * 500.0) as i32 % y_b as i32; // Assuming world z coordinate maps to image y
        let px = img.get_pixel(x.abs() as u32, y.abs() as u32);
        let r = px.0[0] as f64 / 255.0;
        let g = px.0[1] as f64 / 255.0;
        let b = px.0[2] as f64 / 255.0;
        return (Ray::new(record.p, target - record.p), Vec3::new(r*0.8,g*0.8,b*0.8));
    }
}

impl Material for Diffuse {
    fn scatter(&self, record: &HitRecord, _: &Ray) -> (Ray, Color) {
        let target = record.p + record.normal + Vec3::random_unit();
        return (Ray::new(record.p, target - record.p), self.attenuation);
    }
}

impl Material for Metal {
    fn scatter(&self, record: &HitRecord, ray_in: &Ray) -> (Ray, Color) {
        let reflected = Vec3::reflect(ray_in.dir,record.normal);
        return (Ray::new(record.p, reflected), self.attenuation);
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for HittableVec {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec = None;
        for boxed in self.iter() {

            let hittable = boxed.as_ref();
            let new_hit_record = hittable.hit(ray,t_min,closest_so_far);
            match new_hit_record {
                Some(hit) => {
                    hit_rec = new_hit_record; // Only this issue left
                    closest_so_far = hit.t;
                },
                None => ()
            }
        }
        return hit_rec;
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // Plane
        let plane_normal = Vec3::new(0.0,-1.0,0.0);
        let point_on_ray = ray.orig;
        let denom = Vec3::dot(ray.dir, plane_normal);
        if denom < NEAR_ZERO {
            return None;
        }
        let point_on_plane = Vec3::new(0.0, self.y, 0.0);
        let t = (Vec3::dot((point_on_plane - point_on_ray),plane_normal) / denom).abs();

        if t < t_min || t > t_max {
            return None;
        }
        let p = ray.at(t);
        let front_face = is_front_facing(&ray, plane_normal);
        return Some( HitRecord {
            t: t,
            p: p,
            front_face: front_face,
            normal: if front_face { plane_normal } else { -plane_normal },
            hit_material: self.material.as_ref()
        } );
    }
}

impl Hittable for Sphere {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // see Figure 5: Ray-sphere intersection results
        let oc = ray.orig - self.center;
        let a = ray.dir.len_sq();
        let half_b = Vec3::dot(oc, ray.dir);
        let c = oc.len_sq() - self.radius.powi(2);
        let discriminant = half_b*half_b - a*c;
        if discriminant.is_sign_negative() { return None };
        let sqrt_disc = discriminant.sqrt();
        let mut root = (-half_b - sqrt_disc) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_disc) / a;
            if root < t_min || root > t_max { return None };
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = is_front_facing(&ray, outward_normal);
        return Some( HitRecord{
            t: root,
            p: p,
            front_face: front_face,
            normal: if front_face { outward_normal } else {-outward_normal},
            hit_material: self.material.as_ref()
        } );
    }
}
