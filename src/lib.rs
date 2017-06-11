
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
    }
}