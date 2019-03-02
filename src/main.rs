extern crate euclid;
use euclid::*;

extern crate image;
use image::{ImageBuffer};

use rustic_raytracer::ray;
use rustic_raytracer::sphere;

fn main(){
    let nx = 200;
    let ny = 100;

    let mut img = ImageBuffer::new(nx,ny);

    let lower_left_corner = Point3D::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3D::new(4.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 2.0, 0.0);
    let origin = Point3D::new(0.0,0.0,0.0);

    let s1 = sphere::Sphere::new(0.0, 0.0, -1.0, 0.5);
    let s2 = sphere::Sphere::new(0.0, 1.0, -1.0, 0.5);
    let s3 = sphere::Sphere::new(0.0, -100.5, 1.0, 100.0);
    let objects: Vec<&ray::Hitable> = vec![&s1,&s2,&s3];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = ray::Ray::new(origin,lower_left_corner.to_vector() + horizontal*u + vertical*v);
            
            let col = color(&r,&objects);            
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            img.put_pixel(i , j, image::Rgb([ir,ig,ib]));
        } 
    }

    img.save("img.png").unwrap();
}

fn color(ray: &ray::Ray, objects: &Vec<&ray::Hitable>) -> Vector3D<f32> {
    let hit = ray::hitable(ray,0.0,100.0,objects);
    
    match hit {
        Some(h) => {
            return vec3(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0) * 0.5;     
        }
        None => {
            let unit_direction = ray.direction().normalize();
            let t = (unit_direction.y + 1.0) * 0.5;
            vec3(1.0, 1.0, 1.0) * (1.0-t) + vec3(0.5, 0.7, 1.0) * t
        }
    }    
}