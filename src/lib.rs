#[cfg(test)]
mod tests {
    use crate::sphere::Vector3D;
    
    #[test]
    pub fn test_simple_straight_line_direction(){
        let w = super::World::new(2, 2);
        let d = w.get_direction(Vector3D::new(0.0, 0.0, 0.0),0.0,0.0);
        assert_eq!(d, Vector3D::new(0.0, 0.0, -1.0));
    }
    
    #[test]
    pub fn test_simple_straight_line_direction_large(){
        let w = super::World::new(300, 300);
        let d = w.get_direction(Vector3D::new(0.0, 0.0, 0.0),0.0,0.0);
        assert_eq!(d, Vector3D::new(0.0, 0.0, -1.0));
    }

    #[test]
    pub fn test_simple_straight_line_direction_large_and_center(){
        let w = super::World::new(300, 300);
        let d = w.get_direction(Vector3D::new(150.0, 150.0, 0.0),150.0,150.0);
        assert_eq!(d, Vector3D::new(0.0, 0.0, -1.0));
    }

    #[test]
    pub fn test_center_and_center(){
        let w = super::World::new(3, 3);
        let d = w.get_direction(Vector3D::new(1.0, 1.0, 0.0),1.0,1.0);
        assert_eq!(d.x,0.0);
        assert_eq!(d.y,0.0);
        assert_eq!(d.z,-1.0);
    }

    #[test]
    pub fn test_center_and_below(){
        let w = super::World::new(3, 3);
        let d = w.get_direction(Vector3D::new(1.0, 1.0, 0.0),1.0,0.0);
        assert_eq!(d.x,0.0);    //direction in x stays the same
        assert!(d.y < 0.0);     //direction in y < 0 since orgin is above point
        assert!(d.z < 0.0);
    }

    #[test]
    pub fn test_left_and_aligned(){
        let w = super::World::new(3, 3);
        let d = w.get_direction(Vector3D::new(1.0, 1.0, 0.0),0.0,1.0);
        assert!(d.x < 0.0);     //direction in x < 0 since orgin is to the left of the point
        assert_eq!(d.y,0.0);    //direction in y stays the same since the origin is at the same height
        assert!(d.z < 0.0);
    }

    #[test]
    pub fn test_right_and_abowe(){
        let w = super::World::new(3, 3);
        let d = w.get_direction(Vector3D::new(1.0, 1.0, 0.0),2.0,2.0);
        assert!(d.x > 0.0);
        assert!(d.y > 0.0);
        assert!(d.z < 0.0);
    }

    #[test]
    pub fn test_origin_of_plane() {
        let w = super::World::new(3, 3);
        let origin = w.get_plane_origin();
        assert_eq!(origin.x,1.0);
        assert_eq!(origin.y,1.0);
        assert_eq!(origin.z,1.0);
    }

}

pub mod intersection {
    use euclid::Vector3D;

    pub trait Intersection {
        fn intersects(&self, origin: &Vector3D<f32>, direction: &Vector3D<f32>) -> Result<Vector3D<f32>,&str>;
    }
}

pub mod sphere;
use sphere::Vector3D;
use sphere::Intersection;

extern crate image;

pub struct World {
    height: i32,
    width: i32,
    objects: Vec<sphere::Sphere>,
}

impl World {
    pub fn new(height: i32, width: i32) -> World {
        World {
            height: height,
            width: width,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: sphere::Sphere) {
        self.objects.push(object);
    }

    fn get_direction(&self,origin: Vector3D<f32>, x: f32, y: f32) -> Vector3D<f32> {
        let d = Vector3D::new(x, y, -(self.width / 2) as f32) - origin;
        d.normalize()
    }

    fn get_plane_origin(&self) -> Vector3D<f32> {
        let x = self.height as f32 / 2.0;
        let y = self.width as f32 / 2.0;
        
        Vector3D::new(x,y, 0.0)
    }

    pub fn render(&self) {
        let mut imgbuf = image::ImageBuffer::from_fn(self.height as u32, self.width as u32, |x, y| {
            if x % 2 == 0 {
                image::Rgb([100,100,100])
            } else {
                image::Rgb([255,255,255])
            }
        });

        let origin: Vector3D<f32> = self.get_plane_origin();

        let objects = &self.objects[..];
        for x in 0..self.height {
            for y in 0..self.width {
                let dir = self.get_direction(origin, x as f32, y as f32);

                for object in objects {
                    let intersection = object.intersects(&origin,&dir);
                    match intersection {
                        Ok(_) => 
                        {
                            //println!("({},{}):\t1",x,y);
                            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                            *pixel = image::Rgb([255,100,50]);
                        },
                        _ => 
                        {
                            //println!("({},{}):\t0",x,y)
                        }, 
                    }
                }
            }
        }

        imgbuf.save("ray.png").unwrap();
    }
}

pub mod ray {
    mod tests {
        use super::Ray;
        use euclid::Vector3D;
        use euclid::Point3D;

        #[test]
        pub fn test_simple_point_at(){
            let ray = Ray::new(Point3D::new(0.0,0.0,0.0), Vector3D::new(1.0,0.0,0.0));
            let p = ray.point_at(3.0);
            assert_eq!(p,Point3D::new(3.0,0.0,0.0));
        }
    }

    use euclid::Vector3D;
    use euclid::Point3D;

    pub struct Ray{
        a: Point3D<f32>,
        b: Vector3D<f32>,
    }

    impl Ray {
        pub fn new(a: Point3D<f32>, b: Vector3D<f32>) -> Self {
            Self {
                a: a,
                b: b,
            } 
        }

        pub fn point_at(&self,t: f32) -> Point3D<f32> {
            (self.a.to_vector() + self.b * t).to_point()
        }

        pub fn direction(&self) -> Vector3D<f32> {
            self.b
        }
    }
}