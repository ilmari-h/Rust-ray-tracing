use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;
mod vec;
use vec::{Vec3, Color, Point3};
mod ray;
use ray::Ray;
use rand::Rng;
mod hittables;
use hittables::{Sphere, Hittable, HittableVec, HitRecord};

// NOTE: add concurrency

const ASPECT_R: f32 = 16.0 / 9.0;
const IWIDTH: usize = 400; // Making this big causes stack overflow because of array size IWIDTH*IHEIGHT.
const IHEIGHT: usize = ( IWIDTH as f32 / ASPECT_R as f32 ) as usize ;
type Pixel = [i32; 3];
type PArr = [Pixel; IWIDTH*IHEIGHT];

fn get_random(range: Option<Range<f64>>) -> f64 {
    let mut rng = rand::thread_rng();
    match range {
        Some(r) => return rng.gen_range(r),
        None => return rng.gen_range(0.0..1.0)
    }
}

fn pixel_color(ray: &Ray, objects: &HittableVec) -> Color {
    let collision: Option<HitRecord> = objects.hit(ray,0.0,f64::INFINITY);
    match collision {
        Some(rec) => {
            return 0.5 * ( rec.normal + Color::new(1.0,1.0,1.0) )*255.0;
        },
        None => ()
    }
    let unit_dir =  vec::unit_vec(ray.dir);
    let t = 0.5*(unit_dir.y + 1.0);
    return 255.0 * ((1.0-t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5,0.7,1.0));
}

fn main() -> () {
    let mut objects = HittableVec::new();
    objects.push(Box::new(Sphere{center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}));
    objects.push(Box::new(Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));

    let mut im: PArr = [[0,0,0]; IWIDTH*IHEIGHT];

    im = render(&im, &objects);
    save_image(&im);
}

fn render(img: &PArr, objects: &HittableVec) -> PArr {

    // Camera
    let vp_height = 2.0;
    let vp_width = ASPECT_R * vp_height;
    let focal_length = 1.0;
    let origin = Point3{x: 0.0, y: 0.0, z: 0.0};

    let horizontal = Vec3{x: vp_width as f64, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: vp_height as f64, z: 0.0};
    let low_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x:0.0,y:0.0,z: focal_length};

    let mut x = 0;
    let mut y = IHEIGHT;
    let mut n_img: PArr = img.to_owned();
    for i in 0..IWIDTH*IHEIGHT {

        // Shoot ray from viewport.
        let u: f64 = (x as f32 / ( IWIDTH as f32 - 1.0 )).into(); // horizontal offset, percentage
        let v: f64 = (y as f32 / ( IHEIGHT as f32 - 1.0 )).into(); // vertical offset percentage
        let ray: Ray = Ray::new( // see Figure 3: Camera geometry
            origin,
            low_left_corner // In relation to bottom left corner
                + u * horizontal // Add horizontal offset vector u
                + v * vertical // Add vertical offset vector v
                - origin // Offset to put origin in center (no effect when {0,0,0}).
        );

        // Add pixel
        let color = pixel_color(&ray, objects);
        n_img[i] = [color[0] as i32,color[1] as i32,color[2] as i32];

        // screen coordinates
        x = x+1;
        if x % IWIDTH == 0 {
            y = y-1;
            x = 0;
        }

    }
    return n_img;
}

fn save_image(img: &PArr) -> std::io::Result<()> {
    let mut f = File::create("img.ppm")?;
    let header: String = format!("P3\n{} {}\n{}\n",IWIDTH,IHEIGHT,255);
    f.write_all(header.as_bytes())?;
    for i in 0..IWIDTH*IHEIGHT {
        let px: Pixel = img[i];
        let px_s: String = format!("{} {} {}\n", px[0], px[1], px[2]);
        f.write_all(px_s.as_bytes())?;
    }
    Ok(())
}

