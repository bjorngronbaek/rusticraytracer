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
    let horizontal = Vector3D::new(4.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 2.0, 0.0);
    let origin = Point3D::new(0.0,0.0,0.0);

    let mut img = ImageBuffer::new(nx,ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            //let col = (i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin,lower_left_corner.to_vector() + horizontal*u + vertical*v);
            //println!("({},{}:){:?}",i,j,r);
            let col = color(&r);
            
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            img.put_pixel(i , j, image::Rgb([ir,ig,ib]));
            //println!("{},{}:({} {} {})",i,j,ir,ig,ib);
        } 
    }

    img.save("img.png").unwrap();
}

fn color(ray: &Ray) -> Vector3D<f32> {
    let t = hit_sphere(point3(0.0, 0.0, -1.0), 0.5, ray);
    
    if t > 0.0 {
        let normal = (ray.point_at(t) - point3(0.0, 0.0,-1.0)).normalize();
        return vec3(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5; 
    }
    
    let unit_direction = ray.direction().normalize();
    let t = (unit_direction.y + 1.0) * 0.5;
    vec3(1.0, 1.0, 1.0) * (1.0-t) + vec3(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Point3D<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminat = b*b - 4.0*a*c;

    if discriminat < 0.0 {
        return -1.0;
    } 
    else {
        return (-b - f32::sqrt(discriminat) ) / (2.0 * a);
    }
    
}