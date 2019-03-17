use rustic_raytracer;
use rustic_raytracer::sphere;

fn main(){
    let world = rustic_raytracer::World::default();

    let s1 = sphere::Sphere::new(0.0, 0.25, -1.0, 0.5);
    world.add_hitable(&s1);
    let s2 = sphere::Sphere::new(0.0, -100.5, -1.0, 100.0);
    world.add_hitable(&s2);

    rustic_raytracer::render(&world);
}