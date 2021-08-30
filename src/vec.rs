use std::ops;
use rand::Rng;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn unit_vec(vec: Vec3) -> Vec3 {
    return vec / vec.len();
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return
        u[0] * v[0]
      + u[1] * v[1]
      + u[2] * v[2]
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let vec = Vec3::new_random(Some(-1.0..1.0)); // FIXME uniform distribution?
        if vec.len_sq() < 1.0 { return vec }
    }
}

pub fn random_unit() -> Vec3 {
    return unit_vec(random_in_unit_sphere());
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3{x:x,y:y,z:z};
    }

    pub fn new_zero() -> Vec3 {
        return Vec3{x:0.0,y:0.0,z:0.0};
    }

    pub fn new_random(range: Option<ops::Range<f64>>) -> Vec3 { // FIXME uniform distribution?
        let mut rng = rand::thread_rng();
        match range {
            Some(r) => return Vec3::new( rng.gen_range(r.clone()), rng.gen_range(r.clone()), rng.gen_range(r.clone())),
            None =>  return Vec3::new(rng.gen_range(0.0..1.0),rng.gen_range(0.0..1.0),rng.gen_range(0.0..1.0))
         }

    }
    pub fn len(self) -> f64 {
        let sqlen = self.len_sq();

        return sqlen.sqrt();
    }

    pub fn len_sq(self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn cross(self,v: Vec3) -> Vec3 {
        return Vec3{
            x: self[1] * v[2] - self[2] * v[1],
            y: self[2] * v[0] - self[0] * v[2],
            z: self[0] * v[1] - self[1] * v[0]}
    }

    pub fn dot(self, v: Vec3) -> f64 {
        return
              self[0] * v[0]
            + self[1] * v[1]
            + self[2] * v[2]
    }

}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z
        };
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        return Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        return Vec3{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
       return other*self;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        return Vec3{
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        return Vec3{
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Attempting to index Vec3 with {}",i)
        }
    }
}

