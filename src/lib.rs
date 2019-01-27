#[cfg(test)]
mod tests {
    use euclid::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    pub fn test_new_simple_sphere(){
        let sphere = super::Sphere::new(10.0, 10.0, 10.0, 2.0);
        assert_eq!(sphere.radius,2.0);
    }

    #[test]
    pub fn test_no_intersection(){
        let sphere = super::Sphere::new(10.0, 10.0, 10.0, 2.0);
        let origin = Vector3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(-1.0,-1.0,-1.0);
        assert_eq!(sphere.intersects(origin,direction).is_ok(),false);
    }    

    #[test]
    pub fn test_simple_intersection(){
        let sphere = super::Sphere::new(5.0, 1.0, 0.0, 2.0);
        let origin = Vector3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(1.0,0.0,0.0);
        assert_eq!(sphere.intersects(origin,direction).is_ok(),true);
    }

    #[test]
    pub fn test_simple_intersection_on_x_axis(){
        let sphere = super::Sphere::new(5.0, 0.0, 0.0, 2.0);
        let origin = Vector3D::new(0.0, 0.0, 0.0);
        let direction = Vector3D::new(1.0,0.0,0.0);
        let intersection = sphere.intersects(origin,direction);
        assert_eq!(intersection.is_ok(),true);
        assert_eq!(intersection.unwrap(),Vector3D::new(3.0, 0.0, 0.0));
    }

    #[test]
    pub fn test_euclid_mul(){
        let v3: Vector3D<i32,> = vec3(5, 10, 15);
        let v3scaled = v3 * 4;
        assert_eq!(v3scaled.x,5*4);
    }
}

use euclid::*;

pub struct Sphere{
    center: Vector3D<f32>,
    radius: f32,
}

impl Sphere{
    pub fn new(x:f32,y:f32,z:f32,r:f32) -> Sphere {
        Sphere{
            center: Vector3D::new(x, y, z),
            radius: r,
        }
    }

    pub fn intersects(&self, origin: Vector3D<f32>, direction: Vector3D<f32>) -> Result<Vector3D<f32>,&str> {
        let vpc = dbg!(self.center - origin); //vector v from the orgin p to the center c

        let t = dbg!(vpc.dot(direction));
        if t <= 0.0 {
            return Err("No intersection");
        }        
        else {
            let intersection;
            let pc = dbg!(direction * (vpc.dot(direction) / direction.square_length()));
            let pc_length = dbg!( (self.center - pc).length() );
            if dbg!(pc_length > self.radius) {
                return Err("No intersection");
            }
            else{
                let dist = (self.radius.powi(2) - (pc - self.center).length().powi(2)).sqrt();
                if vpc.length() > self.radius {
                    let di1 = (pc - origin).length() - dist;
                    intersection = origin + direction * di1;
                }
                else {
                    let di1 = (pc - origin).length() + dist;
                    intersection = origin + direction * di1;
                }
            }

            dbg!(Ok(intersection))
        }
    }
}