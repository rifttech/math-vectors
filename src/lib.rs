#![doc(html_root_url = "https://rifttech.github.io/math-vectors/")]
pub mod vectors;
#[cfg(test)]
mod vector_tests {
    use super::vectors::vector::*;
    use super::vectors::*;

    #[test]
    fn it_works() {


        let a = Vector2::new(1.0, 5.0);
        let b = Vector2::new(1.0, 5.0);
        println!("vector a is ({},{})",a.x, a.y);
        println!("vector b is ({},{})",b.x, b.y);

        let is_equal = a == b;
        println!("a == b is : {}", is_equal);
        assert!(is_equal);
        let is_equal = b == a;
        println!("a == b is : {}", is_equal);
        assert!(is_equal);


        println!();
        let a = Vector2::new(1.0, 6.0);
        let b = Vector2::new(1.0, 5.0);
        println!("vector a is ({},{})",a.x, a.y);
        println!("vector b is ({},{})",b.x, b.y);
        let is_not_equal = a != b;
        println!("a != b is : {}", is_not_equal);
        assert!(is_not_equal);
        let is_not_equal = b != a;
        println!("a != b is : {}", is_not_equal);
        assert!(is_not_equal);

    }

    #[test]
    fn it_should_be_add_vectors() {
        let a = Vector2::new(1.0, 1.0);
        let b = Vector2::new(2.0, 3.0);
        let c = a + b;
        assert_eq!(c, Vector2::new(3.0, 4.0), "vectors should be equals" );
    }

    #[test]
    fn it_should_be_subtract_vectors() {
        let a = Vector2::new(1.0, 1.0);
        let b = Vector2::new(2.0, 3.0);
        let c = a - b;
        assert_eq!(c, Vector2::new(-1.0, -2.0), "vectors should be equals");
    }

    #[test]
    fn it_should_be_multiply_vector_by_value() {
        let a = Vector2::new(1.0, 1.0);
        let b = 4.0;
        let c = a * b;
        println!("{:?}", c);
        assert_eq!(c, Vector2::new(4.0, 4.0), "vectors should be equals");

        let a = Vector2::new(1.0, 1.0);
        let b = 4.0;
        let c = b * a;
        println!("{:?}", c);
        assert_eq!(c, Vector2::new(4.0, 4.0), "vectors should be equals");
    }

    #[test]
    fn it_should_be_divide_vector_by_value() {
        let a = Vector2::new(2.0, 2.0);
        let b = 4.0;
        let c = a / b;
        println!("{:?}", c);
        assert_eq!(c, Vector2::new(0.5, 0.5), "vectors should be equals");

        let a = Vector2::new(2.0, 2.0);
        let b = 4.0;
        let c = b / a;
        println!("{:?}", c);
        assert_eq!(c, Vector2::new(2.0, 2.0), "vectors should be equals");
    }

    #[test]
    fn constant_vectors_in_use() {
        let t = vector::ONE + vector::LEFT;
        println!("{:?}", t);
        assert_eq!(t, Vector2::new(0.0, 1.0), "vectors should be equals");
    }

    #[test]
    fn test_magnitude() {
        let v = Vector2::new(0.0, 5.0);
        let vector_magnitude = v.magnitude();
        assert_eq!(vector_magnitude, 5.0);

        let v = Vector2::new(0.0, 5.0);
        let squared_magnitude = v.sqr_magnitude();
        assert_eq!(squared_magnitude, 25.0);
    }

    #[test]
    fn it_initialize_default_vector() {
        let v = Vector2::default();
        assert_eq!(v, vector::ZERO);
    }

    #[test]
    fn it_should_return_normalized_vector() {
        let v = Vector2::new(5.0, 0.0);
        let v_normalized = v.normalize();
        assert_eq!(v_normalized, vector::RIGHT);

        let v = Vector2::new(0.0, 0.0);
        let v_normalized = v.normalize();
        assert_eq!(v_normalized, vector::ZERO);
    }

    #[test]
    fn calc_distance_and_sqr_distance() {
        let v0 = Vector2::new(5.0,3.0);
        let v1 = Vector2::new(1.0,0.0);
        let dist = vector::distance(v0, v1);
        assert_eq!(dist, 5.0);
        let v0 = Vector2::new(5.0,3.0);
        let v1 = Vector2::new(1.0,0.0);
        let sqr_dist = vector::distance_squared(v0, v1);
        assert_eq!(sqr_dist, 25.0);
    }

    #[test]
    fn test_dot_product() {
        let v0 = Vector2::new(5.0, 3.0);
        let v1 = Vector2::new(3.0, 3.0);
        let result = vector::dot(v0, v1);
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_vector_max() {
        let v0 = vector::RIGHT * 5.0;
        let v1 = vector::LEFT * 10.0 + vector::ONE * 5.0;

        let max_vector = vector::max(v0, v1);
        assert_eq!(max_vector, Vector2 { x: 5.0, y: 5.0 });
        println!("{:?}", max_vector);
    }
    #[test]
    fn test_vector_min() {
        let v0 = vector::RIGHT * 5.0;
        let v1 = vector::LEFT * 10.0 + vector::ONE * 5.0;

        let max_vector = vector::min(v0, v1);
        assert_eq!(max_vector, Vector2 { x: -5.0, y: 0.0 });
        println!("{:?}", max_vector);
    }

    #[test]
    fn scale_vector() {
        let vector = Vector2::new(5.0, 9.0);
        let scale = Vector2::new(2.0, 3.0);

        let scaled_vector = vector::scale(vector, scale);
        assert_eq!(scaled_vector, Vector2 { x: 10.0, y: 27.0 });
        println!("{:?}", scaled_vector);
    }

    #[test]
    fn find_reflected_vector() {
        let vector = Vector2::new(5.0, 9.0);
        let normal = Vector2::new(1.0, 0.0);

        let reflected_vector = vector::reflect(vector, normal);
        assert_eq!(reflected_vector, Vector2 { x: -5.0, y: 9.0 });
        println!("{:?}", reflected_vector);

    }

    #[test]
    fn it_should_be_parallel() {
        let u = Vector2::new(2.0, 2.0);
        let v = Vector2::new(5.0, 5.0);

        let are_parallel = vector::are_parallel(u, v);
        assert!(are_parallel);
        //println!("u and v are parallel : {}", is_parallel);

        let u = Vector2::new(-2.0, 2.0);
        let v = Vector2::new(5.0, 5.0);

        let are_parallel = vector::are_parallel(u, v);
        assert!(are_parallel);
        //println!("u and v are parallel : {}", is_parallel);


        let u = Vector2::new(0.0, 0.0);
        let v = Vector2::new(5.0, 5.0);

        let are_parallel = vector::are_parallel(u, v);
        assert!(are_parallel);
        //println!("u and v are parallel : {}", is_parallel);
    }

    #[test]
    fn it_should_be_codirectional() {
        let u = Vector2::new(2.0, 2.0);
        let v = Vector2::new(5.0, 5.0);

        let are_codirectional = vector::are_codirectional(u, v);
        assert!(are_codirectional);

        let u = Vector2::new(10.0, 2.0);
        let v = Vector2::new(5.0, 5.0);

        let are_codirectional = vector::are_codirectional(u, v);
        assert!(!are_codirectional);
    }

    #[test]
    fn it_should_be_orthogonal() {
        let u = Vector2::new(0.0, 5.0);
        let v = Vector2::new(10.0, 0.0);

        let are_orthogonal = vector::are_orthogonal(u, v);
        assert!(are_orthogonal);

        let u = Vector2::new(1.0, 5.0);
        let v = Vector2::new(10.0, 0.0);

        let are_orthogonal = vector::are_orthogonal(u, v);
        assert!(!are_orthogonal);
    }
}