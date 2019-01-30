mod intersection {
    use euclid::Vector3D;

    pub trait Intersection {
        fn intersects(&self, origin: Vector3D<f32>, direction: Vector3D<f32>) -> Result<Vector3D<f32>,&str>;
    }
}