extern crate euclid;
use euclid::*;

extern crate image;
use image::{ImageBuffer,GenericImage};

use rustic_raytracer::ray::*;

fn main(){
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255\n",nx,ny);

    let lower_left_corner = Point3D::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3D::new(100.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 100.0, 0.0);
    let origin = Point3D::new(0.0,0.0,0.0);

    let mut img = ImageBuffer::new(nx,ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            //let col = (i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin,lower_left_corner.to_vector() + horizontal*u + vertical*v);
            let col = color(r);
            
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            img.put_pixel(i , j, image::Rgb([ir,ig,ib]));
            //println!("{},{}:({} {} {})",i,j,ir,ig,ib);
        } 
    }

    img.save("img.png").unwrap();
}

fn color(ray: Ray) -> Vector3D<f32> {
    let unit_direction = ray.direction().normalize();
    let t = (unit_direction.y * 1.0) * 0.5;
    vec3(1.0, 1.0, 1.0) * (1.0-t) + vec3(0.5, 0.7, 1.0) * t
}