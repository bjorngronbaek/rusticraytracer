use rand::Rng;

extern crate euclid;
use euclid::{Vector3D};

extern crate image;
use image::{ImageBuffer};

use rustic_raytracer::{ray,sphere,camera,light};

use noise::{NoiseFn, Perlin};
use std::f32;

pub use euclid::Point3D;

fn main(){
    let nx = 600;
    let ny = 600;
    let ns = 10;
    let light_radius = 500.0;
    let light_samples = 1;

    let mut img = ImageBuffer::new(nx,ny);
    let mut rng = rand::thread_rng();

    let camera = camera::Camera::default();

    let mut objects: Vec<&ray::Hitable> = Vec::new();
    let mut lights: Vec<light::Light> = Vec::new();

    let s1 = sphere::Sphere::new(0.0, 0.0, -1.0, 0.5);        
    objects.push(&s1);

    //let s2 = sphere::Sphere::new(-0.5, -0.5, -1.0, 0.3);
    //objects.push(&s2);
    
    let s3 = sphere::Sphere::new(0.0, -100.5, -1.0, 100.0);
    objects.push(&s3);

    let pi2 = 2.0*f32::consts::PI;

    let mut t:f32 = 0.0;
    let mut spiral:f32 = 0.0;
    
    for _i in 0..light_samples{
        let light = light::Light::new((light_radius-spiral)*t.cos(), 100.0, (light_radius-spiral)*t.sin(), 100.0);
        lights.push(light);
        t = t + 3.0*pi2/light_samples as f32;
        spiral = spiral + 0.5*light_radius/light_samples as f32;
    }

    //let light1 = light::Light::new(0.0, -200.0, -1.0, 100.0);
    //lights.push(&light1);

    //let light2 = light::Light::new(0.0, 200.0, -1.0, 100.0);
    //lights.push(&light2);

    //let light3 = light::Light::new(100.0, 200.0, 1.0, 100.0);
    //lights.push(&light3);

    let perlin = Perlin::new();
    

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vector3D<f32> = Vector3D::new(0.0,0.0,0.0);
            for _s in 0..ns{
                let ri: f32 = rng.gen();
                let rj: f32 = rng.gen();
                let u = (i as f32 + ri) / nx as f32;
                let v = (j as f32 + rj) / ny as f32;

                let r = camera.get_ray(u, v);
                col += color(&r,&objects,&lights, perlin);       
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

fn color(ray: &ray::Ray, objects: &Vec<&ray::Hitable>, lights: &Vec<light::Light>, texture: Perlin) -> Vector3D<f32> {

    let hit = ray::hitable(ray, 0.0, 1000.0, objects);
    
    match hit {
        Some(h) => {

            let hit : Point3D<f32> = h.p;
            let tex = texture.get([hit.x as f64, hit.y as f64, hit.z as f64]) as f32;
            let mut color: Vector3D<f32> = Vector3D::new(0.0, 0.0, 0.0);
            let light_fraction: f32 = 1.0/(lights.len() as f32);
            let mut total_light = 0.0;
            for light in lights {
                let search_direction : Vector3D<f32> = light.position - hit;
                let shadow = ray::Ray::new(hit, search_direction);
                let block = ray::hitable(&shadow, 0.0, 1000.0, objects);
                match block {
                    Some(_b) => {
                        //color = color + Vector3D::new(tex, tex, (1.0 + tex)/2.0)*light_fraction*light_fraction;
                        //color = color + Vector3D::new(1.0, 0.0, 0.0)*light_fraction; 
                    }
                    None =>{ 
                        total_light += light_fraction;
                        //color = color + Vector3D::new(1.0, 0.0, 0.0)*light_fraction;
                        //color = color + Vector3D::new(tex, tex, (1.0 + tex)/2.0)*light_fraction;  
                        //return Vector3D::new(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0) * 0.5; 
                    }
                }
            }

            color = Vector3D::new(tex, tex, tex) * total_light;

            return color;     
        }
        None => {
            let unit_direction = ray.direction().normalize();
            let t = (unit_direction.y + 1.0) * 0.5;
            Vector3D::new(1.0, 1.0, 1.0) * (1.0-t) + Vector3D::new(0.5, 0.7, 1.0) * t
        }
    }    
}