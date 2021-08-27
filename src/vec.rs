use std::ops;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn unit_vec(vec: &Vec3) -> Vec3 {
    let v = vec.clone();
    return v / v.len();
}

impl Vec3 {
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

