
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Vector2 {

      pub x:f64
    , pub y:f64

}

impl Vector2 {

    /* constructors*/

    ///
    /// Returns zero vector
    ///
    pub fn default() -> Vector2 {
        Vector2{
            x: 0.0,
            y: 0.0,
        }
    }

    ///
    /// Returns vector{x,y}
    ///
    pub fn new(x:f64, y:f64) -> Vector2 {
        Vector2{
            x: x,
            y: y,
        }
    }

    ///
    /// Returns the squared length of this vector
    ///
    pub fn sqr_magnitude(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }

    ///
    /// Returns the length of this vector
    ///
    pub fn magnitude(&self) -> f64 {
       self.sqr_magnitude().sqrt()
    }

    ///
    /// Returns vector with a magnitude of 1
    ///
    pub fn normalize(&self) -> Vector2 {
        Vector2{ x: (self.x / self.magnitude())
               , y: (self.y / self.magnitude())
        }
    }

}

///
/// Overrides '+' operator
/// Example
/// let a = Vector{1.0 ,0.2}
/// let b = Vector{1.5,-0.5}
/// let c = a + b
/// or
/// let c = b + a
///
impl Add for Vector2{
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2{ x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
}

///
/// Overrides '-' operator
/// subtracts vector
/// if a and b are vector then you
/// can:
/// a - b or b - a
///
impl Sub for Vector2{
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2{ x: (self.x - rhs.x), y: (self.y - rhs.y) }
    }
}

///
/// Overrides '*' operator
/// multiplies vector by value(component-wise)
/// for Vector{x,y} * Scalar = Vector{Scalar * x ,Scalar * y}
///
impl Mul<f64> for Vector2{
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2{ x: (self.x * rhs), y: (self.y * rhs) }
    }
}

///
/// Overrides '*' operator
/// multiplies vector by value(component-wise)
/// for Scalar * Vector{x,y} = Vector{Scalar * x ,Scalar * y}
///
impl Mul<Vector2> for f64{
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2{ x: (self * rhs.x), y: (self * rhs.y) }
    }
}

///
///  Overrides '/' operator
///  Divide vector by value(component-wise)
///  Vector{x,y} / scalar = Vector{x/scalar, y/scalar}
///
impl Div<f64> for Vector2{
    type Output = Vector2;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2{ x: (self.x / rhs), y: (self.y / rhs) }
    }
}

///
/// Overrides '/' operator
/// Divide scalar by vector
/// scalar / Vector{x, y} => Vector{scalar/x, scalar/y}
///
impl Div<Vector2> for f64{
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2{ x: (self / rhs.x), y: (self / rhs.y) }
    }
}

///
/// Overrides '==' operator
/// Compare vectors like A == B or B == A
///
impl PartialEq for Vector2{
    fn eq(&self, other: &Vector2) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
    fn ne(&self, other: &Vector2) -> bool {
        (self.x != other.x) || (self.y != other.y)
    }
}


///
/// Shorthand for writing Vector2{ x: -1.0, y: 0.0 }
///
pub const   LEFT: Vector2 = Vector2{ x: -1.0, y: 0.0 };

///
/// Shorthand for writing Vector2{ x: 0.0, y: 1.0 }
///
pub const   UP: Vector2 = Vector2{ x: 0.0, y: 1.0 };

///
/// Shorthand for writing Vector2{ x: 1.0, y: 0.0 }
///
pub const   RIGHT: Vector2 = Vector2{ x: 1.0, y: 0.0 };

///
/// Shorthand for writing Vector2{ x: 0.0, y: -1.0 }
///
pub const   DOWN: Vector2 = Vector2{ x: 0.0, y: -1.0 };

///
/// Shorthand for writing Vector2{ x: 0.0, y: 0.0 }
///
pub const   ZERO: Vector2 = Vector2{ x: 0.0, y: 0.0 };

///
/// Shorthand for writing Vector2{ x: 1.0, y: 1.0 }
///
pub const   ONE: Vector2 = Vector2{ x: 1.0, y: 1.0 };


///
/// Multiplies two vectors component-wise
///
pub fn scale(a : Vector2, b:Vector2) -> Vector2 {
    Vector2{ x: (a.x * b.x), y: (a.y * b.y) }
}

///
/// Returns a vector that is made from the largest components of two vectors
///
pub fn max(lhs:Vector2, rhs:Vector2) -> Vector2 {
    Vector2{
        x: (if lhs.x > rhs.x {lhs.x} else {rhs.x})
        , y: (if lhs.y > rhs.y {lhs.y} else {rhs.y})
    }
}

///
/// Returns a vector that is made from the smallest components of two vectors
///
pub fn min(lhs: Vector2, rhs:Vector2) -> Vector2 {
    Vector2{
        x: (if lhs.x < rhs.x {lhs.x} else {rhs.x})
        , y: (if lhs.y < rhs.y {lhs.y} else {rhs.y})
    }
}

///
/// Dot Product of two vectors.
///
pub fn dot(lhs:Vector2, rhs:Vector2) -> f64 {
    lhs.x * rhs.x + lhs.x * rhs.x
}

///
/// Reflects a vector off the vector defined by a normal.
///
pub fn reflect(vector:Vector2, normal:Vector2) -> Vector2 {
    let val = 2.0 * ((vector.x * normal.x) +  (vector.y * normal.y));
    Vector2{ x: (vector.x - (normal.x * val))
        , y: (vector.y - (normal.y * val))
    }
}

///
/// Returns the distance between a and b.
///
pub fn distance(a:Vector2, b:Vector2) -> f64 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x*x + y*y).sqrt()
}

///
/// Returns the squared distance between a and b.
///
pub fn distance_squared(a:Vector2, b:Vector2) -> f64{
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x*x + y*y)
}
