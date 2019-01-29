mod sphere;

fn main() {
    let s = sphere::Sphere::new(10.0, 10.0, 10.0, 2.0);
    println!("Sphere: {:?}",s);
}