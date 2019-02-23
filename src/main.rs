use rustic_raytracer::sphere;
use rustic_raytracer::World;

use euclid::*;

extern crate image;

fn main(){
    //test_render();
    test_euclid();


}

fn test_render(){
    let s = sphere::Sphere::new(0.0,0.0,-2.0,1.0);    
    let mut w = World::new(3, 3);
    w.add_object(s);
    w.render();
}

fn test_euclid(){
    pub struct ScreenSpace;
    pub struct WorldSpace;

    pub type WorldPoint = TypedPoint3D<f32,WorldSpace>;
    pub type ScreenPoint  = TypedPoint2D<f32,ScreenSpace>;

    pub type ProjectionMatrix = TypedTransform3D<f32,ScreenSpace,WorldSpace>;

    let p3 = WorldPoint::new(10.0, 10.0, 10.0);
    let p2 = ScreenPoint::new(10.0, 10.0);

    //0.945519, 0, -0.325569, 0, -0.179534, 0.834209, -0.521403, 0, 0.271593, 0.551447, 0.78876, 0, 4.208271, 8.374532, 17.932925, 1)
    let camera_transform = ProjectionMatrix::row_major_2d(1.0, 0.0, 0.0, -1.0, 0.0, 0.0);
    //println!("3D transform: {:?}",camera_transform.transform_point3d(&p3));
    println!("2D transform: {:?}",camera_transform.transform_point3d(&p3));

}