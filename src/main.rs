extern crate indicatif;

use std::sync::Arc;
use std::fs::File;
use std::io::prelude::*;
mod camera;
use camera::Camera;
mod vec;
use vec::{Vec3, Color, Point3};
mod ray;
use ray::Ray;
mod hittables;
use hittables::{Sphere, Plane, Hittable, HittableVec, HitRecord};
mod materials;
use materials::{Diffuse, Metal, Dielectric, TexMat};
use indicatif::{ProgressBar, ProgressStyle};
mod texture;
use texture::Texture;
mod util;
use util::randomf;

// NOTE: add concurrency

const ASPECT_R: f64 = 16.0 / 9.0;
const IWIDTH: usize = 1280; // Making this big causes stack overflow because of array size IWIDTH*IHEIGHT.
const IHEIGHT: usize = ( IWIDTH as f32 / ASPECT_R as f32 ) as usize ;
const BLACK: Vec3 = Vec3{x:0.0,y:0.0,z:0.0};
const RAY_RECURSION_DEPTH: u32 = 50;
const ALMOST_ZERO: f64 = 0.001;
type Pixel = [i32; 3];
type PArr = Vec<Pixel>;

fn ray_color(ray: &Ray, objects: &HittableVec, depth: u32) -> Color {
    let collision: Option<HitRecord> = objects.hit(ray,ALMOST_ZERO,f64::INFINITY);
    if depth == 0 { return BLACK }

    match collision {
        Some(record) => {
            let (bounced_ray, attenuation) = record.on_hit(ray);
            return attenuation * ray_color(&bounced_ray, &objects, depth -1);
        },
        None => {
            let unit_dir =  Vec3::unit_vec(ray.dir);
            let target = 0.5*(unit_dir.y + 1.0);
            return (1.0-target) * Color::new(1.0,1.0,1.0) + target * Color::new(0.5,0.7,1.0);
        }
    }
}

fn main() -> () {
    let test_tex = Texture::new("img/tile2_diff.jpg".to_string(), 1.0);

    let brown_matte = Arc::new( Diffuse{ attenuation: Color{x: 0.51, y: 0.31, z: 0.21} });
    let clear_metal = Arc::new(Metal{ attenuation: Color{x: 1.0, y: 1.0, z: 1.0}, fuzz: 0.0 } );
    let blue_metal = Arc::new( Metal{ attenuation: Color{x: 0.20, y: 0.20, z: 0.70}, fuzz: 0.0 });
    let green_metal = Arc::new (Metal{ attenuation: Color{x: 0.0, y: 0.70, z: 0.0}, fuzz: 0.1 });
    let black_metal = Arc::new (Metal{ attenuation: Color{x: 0.1, y: 0.1, z: 0.1} , fuzz: 0.0 } );
    let glass_m = Arc::new(  Dielectric{ index_of_refraction: 1.5 } );

    let texture_m = Arc::new( TexMat{ texture: test_tex } );


    let mut objects = HittableVec::new();
    objects.push(Box::new(Sphere{center: Vec3::new(-0.51, 0.0, -1.0), radius: 0.5, material: brown_matte.clone()}));
    objects.push(Box::new(Sphere{center: Vec3::new(0.51, 0.0, -1.0), radius: 0.5, material: blue_metal.clone()}));
    objects.push(Box::new(Sphere{center: Vec3::new(-0.1, -0.35, 0.2), radius: 0.15, material: clear_metal.clone()}));
    objects.push(Box::new(Sphere{center: Vec3::new(-1.2, 0.0, 0.0), radius: 0.5, material: green_metal.clone()}));
    objects.push(Box::new(Sphere{center: Vec3::new(1.2, 0.0, 0.0), radius: 0.5, material: glass_m.clone()}));
    objects.push(Box::new(Sphere{center: Vec3::new(0.5, -0.35, -0.3), radius: 0.15, material: black_metal.clone()}));
    objects.push(Box::new(Sphere{center: Vec3::new(-0.7, -0.35, -0.48), radius: 0.15, material: green_metal.clone()}));
    objects.push(Box::new(Plane{y:-0.5, material: texture_m.clone() } ) );

    let mut im: PArr = vec![];

    println!("Rendering image...");
    im = render(&im, &objects); // TODO combine these two
    save_image(&im);
}

fn render(img: &PArr, objects: &HittableVec) -> PArr {

    // Camera
    let camera = Camera::new( ASPECT_R, 60.0, Vec3::new(0.0, 0.75, 3.0) );
    let pixel_samples = 100;
    let bar = ProgressBar::new(IHEIGHT as u64);
    bar.set_style(ProgressStyle::default_bar()
                   .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                   .progress_chars("#>-"));

    let mut x = 0;
    let mut y = IHEIGHT;
    let mut n_img: PArr = img.to_owned();
    for i in 0..IWIDTH*IHEIGHT {

        // Create rays.
        let mut color = Color::new(0.0,0.0,0.0);
        for _ in 0..pixel_samples {

            let u: f64 = ( // horizontal offset percentage
                ( x as f32 + randomf(None) as f32 )
                / ( IWIDTH as f32 - 1.0 )
            ).into();
            let v: f64 = ( // vertical offset percentage
                ( y as f32 + randomf(None) as f32 )
                / ( IHEIGHT as f32 - 1.0 )
            ).into();

            let ray: Ray = camera.get_ray(u,v);
            color = color + ray_color(&ray, objects, RAY_RECURSION_DEPTH);
        }

        // Scale by samples and gamma correction
        let scale = 1.0 / pixel_samples as f64;
        color = Vec3::new(
            (color.x * scale).sqrt(),
            (color.y * scale).sqrt(),
            (color.z * scale).sqrt()
        ) * 255.0;

        // Add pixel
        n_img.push( [color[0] as i32,color[1] as i32,color[2] as i32] );

        // screen coordinates
        x = x+1;
        if x % IWIDTH == 0 {
            bar.inc(1);
            y = y-1;
            x = 0;
        }
    }
    bar.finish();
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

