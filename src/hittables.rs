use crate::vec::{Vec3, Point3, dot};
use crate::ray::Ray;

pub type HittableVec = Vec<Box<dyn Hittable>>;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
}

fn is_front_facing(ray: &Ray, outward_normal: Vec3) -> bool {
    return dot(ray.dir, outward_normal).is_sign_negative();
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for HittableVec {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec = None;
        for boxed in self.iter() {
            // Find closest hit and return that so that objects closer to camera cover the ones
            // behind them.
            let hittable = boxed.as_ref();
            let new_hit_record = hittable.hit(ray,t_min,closest_so_far);
            match new_hit_record {
                Some(hit) => {
                    hit_rec = Some(hit);
                    closest_so_far = hit.t;
                },
                None => ()
            }
        }
        return hit_rec;
    }
}

impl Hittable for Sphere {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // see Figure 5: Ray-sphere intersection results
        let oc = ray.orig - self.center;
        let a = ray.dir.len_sq();
        let half_b = dot(oc, ray.dir);
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
        let rec = HitRecord{
            t: root,
            p: p,
            front_face: front_face,
            normal: if front_face { outward_normal } else {-outward_normal}
        };
        return Some(rec);
    }
}
