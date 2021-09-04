use crate::vec::{Vec3, Point3};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    low_left_corner: Point3,
    horizontal_max: Vec3,
    vertical_max: Vec3
}

impl Camera {
    pub fn new( aspect_r: f64, fov: f64, origin: Vec3 ) -> Camera {

        let theta = fov.to_radians();
        let v = (theta/2.0).tan();
        let vp_w = 2.0 * v;
        let vp_h = vp_w / aspect_r;

        let focal_l = 1.0;

        let horizontal_max = Vec3::new(vp_w, 0.0, 0.0);
        let vertical_max = Vec3::new(0.0, vp_h, 0.0);

        return Camera{
            origin: origin,
            horizontal_max: horizontal_max,
            vertical_max: vertical_max,
            low_left_corner: origin - horizontal_max / 2.0 - vertical_max / 2.0 - Vec3::new(0.0,0.0, focal_l)
        }

    }

    // Get ray towards a certain screen point defined by offset percentages in
    // relation to the bottom left corner of the screen.
    pub fn get_ray(self, h_offset: f64, v_offset: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.low_left_corner
                + h_offset * self.horizontal_max
                + v_offset * self.vertical_max
                - self.origin + Vec3::new(0.0,-0.2,0.0)
        );
    }
}
