use crate::vec::{Vec3, Point3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray{orig: origin, dir: direction} 
    }

    pub fn at(self,t: f64) -> Point3 {
       return self.orig + t * self.dir;
    }
}
    
