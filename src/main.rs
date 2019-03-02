use rand::Rng;

extern crate euclid;
use euclid::{Vector3D};

extern crate image;
use image::{ImageBuffer};

use rustic_raytracer::{ray,sphere,camera};

fn main(){
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut img = ImageBuffer::new(nx,ny);
    let mut rng = rand::thread_rng();

    let camera = camera::Camera::default();

    let mut objects: Vec<&ray::Hitable> = Vec::new();
    let s1 = sphere::Sphere::new(0.0, 0.0, -1.0, 0.5);        
    objects.push(&s1);

    let s2 = sphere::Sphere::new(-0.5, -0.5, -1.0, 0.3);
    objects.push(&s2);
    
    let s3 = sphere::Sphere::new(0.0, -100.5, -1.0, 100.0);
    objects.push(&s3);

    for j in (0..ny).rev() {
        for i in 0..nx {

            let mut col: Vector3D<f32> = Vector3D::new(0.0,0.0,0.0);
            for _s in 0..ns{
                let ri: f32 = rng.gen();
                let rj: f32 = rng.gen();
                let u = (i as f32 + ri) / nx as f32;
                let v = (j as f32 + rj) / ny as f32;

                let r = camera.get_ray(u, v);
                col += color(&r,&objects);       
            }
            col = col / ns as f32;
                 
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            img.put_pixel(i , j, image::Rgb([ir,ig,ib]));
        } 
    }

    /* flip vertically to make the y-axis go up */
    let flipped_img = image::imageops::flip_vertical(&img);
    flipped_img.save("img_aa_fliped.png").unwrap();
}

fn color(ray: &ray::Ray, objects: &Vec<&ray::Hitable>) -> Vector3D<f32> {
    let hit = ray::hitable(ray,0.0,1000.0,objects);
    
    match hit {
        Some(h) => {
            return Vector3D::new(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0) * 0.5;     
        }
        None => {
            let unit_direction = ray.direction().normalize();
            let t = (unit_direction.y + 1.0) * 0.5;
            Vector3D::new(1.0, 1.0, 1.0) * (1.0-t) + Vector3D::new(0.5, 0.7, 1.0) * t
        }
    }    
}