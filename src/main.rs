use std::fs::File;
use std::io::prelude::*;
mod vec;
use vec::{Vec3, Color, Point3, unit_vec};
mod ray;
use ray::Ray;

const ASPECT_R: f32 = 16.0 / 9.0;
const IWIDTH: usize = 400;
const IHEIGHT: usize = ( IWIDTH as f32 / ASPECT_R as f32 ) as usize ;
type Pixel = [i32; 3];
type PArr = [Pixel; IWIDTH*IHEIGHT];

fn pixel_color(ray: &Ray) -> Color {
    let unit_dir =  unit_vec(&ray.dir);
    let t = 0.5*(unit_dir.y + 1.0);
    return 255.0 * ((1.0-t) * Color{x:1.0,y:1.0,z:1.0} + t * Color{x: 0.5, y: 0.7, z: 1.0});
}

fn main() -> () {
    let mut im: PArr = [[0,0,0]; IWIDTH*IHEIGHT];

    im = render(&im);
    let tv = Vec3{x: 4.0, y: 2.0, z: 0.0};
    println!("len: {}", tv.len());
    print!("size {}", IHEIGHT);
    save_image(&im);
}

fn render(img: &PArr) -> PArr {

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

        let u: f64 = (x as f32 / ( IWIDTH as f32 - 1.0 )).into();
        let v: f64 = (y as f32 / ( IHEIGHT as f32 - 1.0 )).into();
        let ray: Ray = Ray::new(origin, low_left_corner + u * horizontal + v * vertical - origin);
        let color = pixel_color(&ray);

        n_img[i] = [color[0] as i32,color[1] as i32,color[2] as i32];
        // Set x and y
        x = x+1;
        if x % IWIDTH == 0 {
            y = y-1;
            x = 0;
        }

    }
    return n_img;
}

// TODO: use concurrency to stream the render operation into file?
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
