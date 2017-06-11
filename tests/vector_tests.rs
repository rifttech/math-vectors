
extern crate math_vectors;
#[cfg(test)]
mod vector_tests {
    use super::math_vectors::vectors::vector::*;
    use super::math_vectors::vectors::*;

    #[test]
    fn it_works() {
        /*
                let vec = Vector2::default();
                println!("vector is {},{}",vec.x, vec.y);
                println!("sqr magnitude is : {}", vec.sqr_magnitude());
                println!("magnitude is : {}", vec.magnitude());

                println!();

                let vec = Vector2::new(3.0, 2.0);
                println!("vector is {},{}",vec.x, vec.y);
                println!("sqr magnitude is : {}", vec.sqr_magnitude());
                println!("magnitude is : {}", vec.magnitude());
                println!();

                let a = Vector2::new(4.0, 5.0);
                let b = Vector2::new(5.0, 6.0);
                println!("vector a is ({},{})",a.x, a.y);
                println!("vector b is ({},{})",b.x, b.y);
                let sum = a + b;
                println!("sum of a and b is : {},{}", sum.x, sum.y);
                println!();

                let a = Vector2::new(4.0, 5.0);
                let b = Vector2::new(5.0, 6.0);
                println!("vector a is ({},{})",a.x, a.y);
                println!("vector b is ({},{})",b.x, b.y);
                let sub = a - b;
                println!("sum of a and b is : ({},{})", sub.x, sub.y);
                println!();

                let a = Vector2::new(4.0, 5.0);
                let b = Vector2::new(5.0, 6.0);
                println!("vector a is ({},{})",a.x, a.y);
                println!("vector b is ({},{})",b.x, b.y);
                let mul = a * b;
                println!("sum of a and b is : ({},{})", mul.x, mul.y);
                println!();

                let a = Vector2::new(4.0, 5.0);
                let b = Vector2::new(5.0, 6.0);
                println!("vector a is ({},{})",a.x, a.y);
                println!("vector b is ({},{})",b.x, b.y);
                let div = a / b;
                println!("sum of a and b is : ({},{})", div.x, div.y);

                println!();


                let a = Vector2::new(1.0, 5.0);
                let b = Vector2::new(1.0, 5.0);
                println!("vector a is ({},{})",a.x, a.y);
                println!("vector b is ({},{})",b.x, b.y);

                let is_equal = a == b;
                println!("a == b is : {}", is_equal);
                let is_equal = b == a;
                println!("a == b is : {}", is_equal);


                println!();
                let a = Vector2::new(1.0, 6.0);
                let b = Vector2::new(1.0, 5.0);
                println!("vector a is ({},{})",a.x, a.y);
                println!("vector b is ({},{})",b.x, b.y);
                let is_not_equal = a == b;
                println!("a != b is : {}", is_not_equal);
                let is_not_equal = b == a;
                println!("a != b is : {}", is_not_equal);

                println!();

                let vector = Vector2::new(4.0, 6.0);
                let normalized_vector = vector.normalize();
                println!("vector a is ({},{})",vector.x, vector.y);
                println!("vector a is ({},{})",normalized_vector.x, normalized_vector.y);

                let norm_vector_magnitude = normalized_vector.magnitude();
                println!("magnitude a is ({})",norm_vector_magnitude);
        */


    }

    #[test]
    fn it_should_be_add_vectors(){
        let a = Vector2::new(1.0, 1.0);
        let b = Vector2::new(2.0, 3.0);
        let c = a + b;
        assert_eq!(c,Vector2::new(3.0, 4.0), "vectors should be equals" );
    }

    #[test]
    fn it_should_be_subtract_vectors() {
        let a = Vector2::new(1.0, 1.0);
        let b = Vector2::new(2.0, 3.0);
        let c = a - b;
        assert_eq!(c,Vector2::new(-1.0, -2.0), "vectors should be equals");
    }

    #[test]
    fn it_should_be_multiply_vector_by_value() {
        let a = Vector2::new(1.0, 1.0);
        let b = 4.0;
        let c = a * b;
        println!("{:?}",c);
        assert_eq!(c,Vector2::new(4.0, 4.0), "vectors should be equals");

        let a = Vector2::new(1.0, 1.0);
        let b = 4.0;
        let c = b * a;
        println!("{:?}",c);
        assert_eq!(c,Vector2::new(4.0, 4.0), "vectors should be equals");


    }

    #[test]
    fn it_should_be_divide_vector_by_value() {
        let a = Vector2::new(2.0, 2.0);
        let b = 4.0;
        let c = a / b;
        println!("{:?}",c);
        assert_eq!(c,Vector2::new(0.5, 0.5), "vectors should be equals");

        let a = Vector2::new(2.0, 2.0);
        let b = 4.0;
        let c = b / a;
        println!("{:?}",c);
        assert_eq!(c,Vector2::new(2.0, 2.0), "vectors should be equals");
    }

    #[test]
    fn constant_vectors_in_use() {
        let t = vector::ONE + vector::LEFT;
        println!("{:?}",t);
        assert_eq!(t,Vector2::new(0.0,1.0),"vectors should be equals");
    }
}
