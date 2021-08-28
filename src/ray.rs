use crate::vec::{Vec3, Point3};

/**
 * Create struct P(t) = A + t * b,
 * where A = origin, b = direction and t = distance travelled on ray.
 */
#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray{orig: origin, dir: direction} 
    }

    /**
     * Compute result of P(t) = A + t * b.
     */
    pub fn at(self,t: f64) -> Point3 {
       return self.orig + t * self.dir;
    }
}
    
