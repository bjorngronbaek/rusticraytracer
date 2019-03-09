pub mod ray {

    #[cfg(test)]
    mod tests {
        use super::Ray;
        use euclid::Vector3D;
        use euclid::Point3D;
        use float_cmp::ApproxEqUlps;

        #[test]
        pub fn test_simple_point_at(){
            let ray = Ray::new(Point3D::new(0.0,0.0,0.0), Vector3D::new(1.0,0.0,0.0));
            let p = ray.point_at(3.0);
            assert_eq!(p,Point3D::new(3.0,0.0,0.0));
        }

        #[test]
        pub fn test_simple_point_at_on_two_dimensions(){
            let ray = Ray::new(Point3D::new(0.0,0.0,0.0), Vector3D::new(3.0,4.0,0.0));
            let p = ray.point_at(5.0);
            assert_eq!(p,Point3D::new(3.0,4.0,0.0));
        }

        #[test]
        pub fn test_simple_point_at_on_two_dimensions_x_z(){
            let ray = Ray::new(Point3D::new(0.0,0.0,0.0), Vector3D::new(3.0,0.0,4.0));
            let p = ray.point_at(5.0);
            assert_eq!(p,Point3D::new(3.0,0.0,4.0));
        }

        #[test]
        pub fn test_point_at_on_two_dimensions_normalized(){
            let ray = Ray::new(Point3D::new(0.0,0.0,0.0), Vector3D::new(3.0,4.0,0.0));
            let p = ray.point_at(50.0);
            //assert_eq!(p,Point3D::new(30.0,40.0,0.0)); floating precission comparison!!!
            assert!(p.x.approx_eq_ulps(&30.0,2));
            assert!(p.y.approx_eq_ulps(&40.0,2));
            assert!(p.z.approx_eq_ulps(&0.0,2));
        }        
    }

    pub use euclid::Vector3D;
    pub use euclid::Point3D;

    #[derive(Debug)]    
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
            (self.a + self.b.normalize() * t)
        }

        pub fn direction(&self) -> Vector3D<f32> {
            self.b
        }

        pub fn origin(&self) -> Point3D<f32> {
            self.a
        }
    }

    #[derive(Debug)]    
    pub struct HitRecord {
        pub t: f32,
        pub p: Point3D<f32>,
        pub normal: Vector3D<f32>
    }

    impl HitRecord {
        pub fn new(t1: f32, p1: Point3D<f32>, normal1: Vector3D<f32>) -> HitRecord {
            HitRecord {
                t: t1,
                p: p1,
                normal: normal1
            }
        }
    }

    pub trait Hitable {
        fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    }

    pub fn hitable(ray: &Ray, t_min: f32, t_max: f32, hitables: &Vec<&Hitable>) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_found: Option<HitRecord> = Option::None;

        for object in hitables {
            let hit = object.hit(ray, t_min, closest_so_far);
            match hit {
                Some(h) => {
                    closest_so_far = h.t;
                    hit_found = Option::Some(h);
                },
                None => {}
            }
        }
        return hit_found;
    }
}

pub mod light{
    use super::ray::Point3D;

    #[derive(Debug)]
    pub struct Light{
        pub position : Point3D<f32>,
        pub intensity : f32,
        //color : image::color<u8>,
    }

    impl Light{
        pub fn new(x:f32,y:f32,z:f32,i:f32) -> Light {
            Light{
                position: Point3D::new(x, y, z),
                intensity: i,
            }
        }
    }
}

pub mod camera {
    use super::ray::Point3D;
    use super::ray::Vector3D;
    use super::ray::Ray;

    pub struct Camera {
        lower_left_corner : Point3D<f32>,
        horizontal : Vector3D<f32>,
        vertical : Vector3D<f32>,
        origin : Point3D<f32>,
    }

    impl Default for Camera {
        fn default() -> Self {
            Camera {
                lower_left_corner : Point3D::new(-2.0, -1.0, -1.0),
                horizontal : Vector3D::new(4.0, 0.0, 0.0),
                vertical : Vector3D::new(0.0, 4.0, 0.0),
                origin : Point3D::new(0.0,0.0,0.0),
            }
        }
    }
    
    impl Camera {
        pub fn get_ray(&self,u: f32, v: f32) -> Ray {
            Ray::new(self.origin, self.lower_left_corner.to_vector() + self.horizontal*u + self.vertical*v - self.origin.to_vector())
        }   
    }
}

pub mod sphere {
    #[cfg(test)]
    mod tests {

        use euclid::Vector3D;
        use euclid::Point3D;
        use super::Sphere;
        use super::Hitable;
        use super::Ray;

        use crate::ray::hitable;

        #[test]
        pub fn test_simple_hit() {
            let s = Sphere::new(5.0, 0.0, 0.0, 2.0);
            
            let p = Point3D::new(0.0,0.0,0.0);
            let direction = Vector3D::new(1.0, 0.0, 0.0);
            let ray = Ray::new(p,direction);

            let hit = s.hit(&ray, -10.0, 10.0);
            assert!(hit.is_some());
            assert_eq!(hit.unwrap().p, Point3D::new(3.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_simple_hitable_list(){
            let mut objects: Vec<&Hitable> = Vec::new();
            
            let s1 = Sphere::new(5.0, 0.0, 0.0, 2.0);
            let s2 = Sphere::new(10.0, 0.0, 0.0, 2.0);
            objects.push(&s1);
            objects.push(&s2);

            let p = Point3D::new(0.0,0.0,0.0);
            let direction = Vector3D::new(1.0, 0.0, 0.0);
            let ray = Ray::new(p,direction);
             
            let hit = hitable(&ray,-10.0,100.0,&objects);
            assert!(hit.is_some());
            assert_eq!(hit.unwrap().p, Point3D::new(3.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_hit_from_inside(){
            let s = Sphere::new(1.0, 0.0, 0.0, 2.0);
            
            let p = Point3D::new(0.0,0.0,0.0);
            let direction = Vector3D::new(1.0, 0.0, 0.0);
            let ray = Ray::new(p,direction);

            let hit = s.hit(&ray, 0.0, 10.0);
            assert!(hit.is_some());
            assert_eq!(hit.unwrap().p, Point3D::new(3.0, 0.0, 0.0));
        }
    }

    use crate::ray::Ray;
    use crate::ray::Point3D;
    use crate::ray::Hitable;
    use crate::ray::HitRecord;

    #[derive(Debug)]
    pub struct Sphere{
        center: Point3D<f32>,
        radius: f32,
    }

    impl Sphere{
        pub fn new(x:f32,y:f32,z:f32,r:f32) -> Sphere {
            Sphere{
                center: Point3D::new(x, y, z),
                radius: r,
            }
        }
    }

    impl Hitable for Sphere {
        fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
            let oc = ray.origin() - self.center;
            let a = ray.direction().dot(ray.direction());
            let b = oc.dot(ray.direction());
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminat = b*b - a*c;

            if discriminat > 0.0 {
                let temp1 = (-b - f32::sqrt(b*b-a*c)) / a;
                if temp1 < t_max && temp1 > t_min {
                    let p = ray.point_at(temp1);
                    let normal = (p - self.center).normalize();
                    let hit = HitRecord::new(temp1,p,normal);

                    return Option::Some(hit);
                }
                
                let temp2 = (-b + f32::sqrt(b*b-a*c)) / a;
                if temp2 < t_max && temp2 > t_min {
                    let p = ray.point_at(temp2);
                    let normal = (p - self.center).normalize();
                    let hit = HitRecord::new(temp2,p,normal);

                    return Option::Some(hit);
                }               
            }

            return Option::None;
        }
    }
}