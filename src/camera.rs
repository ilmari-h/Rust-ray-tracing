use crate::vec::*;
use crate::ray::*;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    low_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new( aspect_r: f64, fov: f64, origin: Vec3, pitch_yaw: Vec3  ) -> Camera {

        let vup = v3!(0.0,1.0,0.0);
        let theta = fov.to_radians();
        let v = (theta/2.0).tan();
        let vp_w = 2.0 * v;
        let vp_h = vp_w / aspect_r;

        let horizontal_max = v3!(vp_w, 0.0, 0.0);
        let vertical_max = v3!(0.0, vp_h, 0.0);

        let lookp = v3!(0.0,0.0,1.0)
            .rotate_x(pitch_yaw[0])
            .rotate_y(pitch_yaw[1]);

        let w = Vec3::unit_vec(lookp);
        let u = Vec3::unit_vec(Vec3::cross(vup,w)); // rotate this with torque
        let v = Vec3::cross(w,u);

        return Camera{
            origin: origin,
            horizontal: vp_w * u,
            vertical: vp_h * v,
            low_left_corner: origin - horizontal_max / 2.0 - vertical_max / 2.0 - w
        }

    }

    // Get ray towards a certain screen point defined by offset percentages in
    // relation to the bottom left corner of the screen.
    pub fn get_ray(self, h_offset: f64, v_offset: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.low_left_corner
                + h_offset * self.horizontal
                + v_offset * self.vertical
                - self.origin
        );
    }
}
