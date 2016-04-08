use std::ops::{Add, Sub};
use std::f64;
use math::approx_eq::ApproxEq;

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

const VEC2_EPSILON : Vec2  = Vec2{ x: f64::EPSILON, y: f64::EPSILON };
const VEC2_IDENTITY : Vec2 = Vec2{ x: 1.0, y: 1.0 };

impl Vec2 {

    /// Allocates a new Vec2 with the given `x` and `y`.
    ///
    /// # Examples
    /// ```
    /// use candle::math;
    /// let vec1 = math::Vec2::new(1.0, 2.0);
    /// assert_eq!(vec1.x, 1.0);
    /// assert_eq!(vec1.y, 2.0);
    ///
    /// // You don't actually need to call `new` to get a Vec2.
    /// let vec2 = math::Vec2{ x: 1.0, y: 2.0 };
    /// ```
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    /// Returns the dot product between two vectors.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(5.0, 12.0);
    /// let vec2 = Vec2::new(-6.0, 8.0);
    /// assert_eq!(vec1.dot(&vec2), 66.0);
    /// ```
    pub fn dot(&self, other: &Vec2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Calculates the length of the vector.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 4.0);
    /// assert_eq!(vec1.length(), 5.0);
    /// ```
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Alias for the `length` function.
    pub fn magnitude(&self) -> f64 {
        self.length()
    }

    /// Calculates the normalized values of the vector and returns a new vector with it's
    /// coordinates.
    /// # Examples
    /// ```
    /// use candle::math::{Vec2, ApproxEq};
    /// let vec = Vec2::new(4.0, 2.0);
    /// let nor = vec.normalized();
    /// assert!(nor.length().approx_eq(1.0));
    /// ```
    pub fn normalized(&self) -> Vec2 {
        let length = self.length();
        Vec2::new(self.x / length, self.y / length)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    /// Adds the coordinates of two vetors, returning a new result vector.
    ///
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(1.0, 5.0);
    /// let vec2 = Vec2::new(2.0, 1.0);
    /// let sum = vec1 + vec2;
    /// assert_eq!(sum.x, 3.0);
    /// assert_eq!(sum.y, 6.0);
    /// ```
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<'a> Add<&'a Vec2> for Vec2 {
    type Output = Vec2;

    /// Adds the coordinates of two vetors, returning a new result vector.
    ///
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(1.0, 5.0);
    /// let vec2 = Vec2::new(2.0, 1.0);
    /// let sum = vec1 + &vec2;
    /// assert_eq!(sum.x, 3.0);
    /// assert_eq!(sum.y, 6.0);
    /// ```
    fn add(self, other: &'a Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;

    /// Adds the coordinates of two vetors, returning a new result vector.
    ///
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(1.0, 5.0);
    /// let vec2 = Vec2::new(2.0, 1.0);
    /// let sum = &vec1 + &vec2;
    /// assert_eq!(sum.x, 3.0);
    /// assert_eq!(sum.y, 6.0);
    /// ```
    fn add(self, other: &'b Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<'b> Add<Vec2> for &'b Vec2 {
    type Output = Vec2;

    /// Adds the coordinates of two vetors, returning a new result vector.
    ///
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(1.0, 5.0);
    /// let vec2 = Vec2::new(2.0, 1.0);
    /// let sum = &vec1 + vec2;
    /// assert_eq!(sum.x, 3.0);
    /// assert_eq!(sum.y, 6.0);
    /// ```
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<f64> for Vec2 {
    type Output = Vec2;

    /// Adds the given number to each vector coordinate.
    ///
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 4.0);
    /// let sum = vec1 + 2.0;
    /// assert_eq!(sum.x, 5.0);
    /// assert_eq!(sum.y, 6.0);
    /// ```
    fn add(self, other: f64) -> Vec2 {
        Vec2 { x: self.x + other, y: self.y + other }
    }
}

impl<'b> Add<f64> for &'b Vec2 {
    type Output = Vec2;

    /// Adds the given number to each vector coordinate.
    ///
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 4.0);
    /// let sum = &vec1 + 2.0;
    /// assert_eq!(sum.x, 5.0);
    /// assert_eq!(sum.y, 6.0);
    /// ```
    fn add(self, other: f64) -> Vec2 {
        Vec2 { x: self.x + other, y: self.y + other }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    /// Substracts the coordinates from the two vectors returning a new result vector.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 5.0);
    /// let vec2 = Vec2::new(1.0, 6.0);
    /// let diff = vec1 - vec2;
    /// assert_eq!(diff.x, 2.0);
    /// assert_eq!(diff.y, -1.0);
    /// ```
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'a> Sub<&'a Vec2> for Vec2 {
    type Output = Vec2;

    /// Substracts the coordinates from the two vectors returning a new result vector.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 5.0);
    /// let vec2 = Vec2::new(1.0, 6.0);
    /// let diff = vec1 - &vec2;
    /// assert_eq!(diff.x, 2.0);
    /// assert_eq!(diff.y, -1.0);
    /// ```
    fn sub(self, other: &'a Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'a, 'b> Sub<&'a Vec2> for &'b Vec2 {
    type Output = Vec2;

    /// Substracts the coordinates from the two vectors returning a new result vector.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 5.0);
    /// let vec2 = Vec2::new(1.0, 6.0);
    /// let diff = &vec1 - &vec2;
    /// assert_eq!(diff.x, 2.0);
    /// assert_eq!(diff.y, -1.0);
    /// ```
    fn sub(self, other: &'a Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'b> Sub<Vec2> for &'b Vec2 {
    type Output = Vec2;

    /// Substracts the coordinates from the two vectors returning a new result vector.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 5.0);
    /// let vec2 = Vec2::new(1.0, 6.0);
    /// let diff = &vec1 - vec2;
    /// assert_eq!(diff.x, 2.0);
    /// assert_eq!(diff.y, -1.0);
    /// ```
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<f64> for Vec2 {
    type Output = Vec2;

    /// Substracts the vector coordinates from the given value.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 2.0);
    /// let diff = vec1 - 2.0;
    /// assert_eq!(diff.x, 1.0);
    /// assert_eq!(diff.y, 0.0);
    /// ```
    fn sub(self, value: f64) -> Vec2 {
        Vec2::new(self.x - value, self.y - value)
    }
}

impl<'b> Sub<f64> for &'b Vec2 {
    type Output = Vec2;

    /// Substracts the vector coordinates from the given value.
    /// # Examples
    /// ```
    /// use candle::math::Vec2;
    /// let vec1 = Vec2::new(3.0, 2.0);
    /// let diff = vec1 - 2.0;
    /// assert_eq!(diff.x, 1.0);
    /// assert_eq!(diff.y, 0.0);
    /// ```
    fn sub(self, value: f64) -> Vec2 {
        Vec2::new(self.x - value, self.y - value)
    }
}

impl ApproxEq<Vec2> for Vec2 {
    fn approx_eq_eps(self, other: Vec2, eps: Vec2) -> bool {
        (self.x - other.x).abs() < eps.x && (self.y - other.y).abs() < eps.y
    }

    /// Returns true if the vector is approximately equal the other vector, with EPSILON amount
    /// of tolerance for both coordinates.
    /// # Examples
    /// ```
    /// use candle::math::{Vec2, ApproxEq};
    /// let vec1 = Vec2::new(1.0, 1.0);
    /// let vec2 = Vec2::new(0.99999999999999999999, 0.99999999999999999999);
    /// assert!(vec1.approx_eq(vec2));
    /// ```
    fn approx_eq(self, other: Vec2) -> bool {
        self.approx_eq_eps(other, VEC2_EPSILON)
    }
}

impl<'a> ApproxEq<&'a Vec2> for Vec2 {
    fn approx_eq_eps(self, other: &Vec2, eps: &Vec2) -> bool {
        (self.x - other.x).abs() < eps.x && (self.y - other.y).abs() < eps.y
    }

    /// Returns true if the vector is approximately equal the other vector, with EPSILON amount
    /// of tolerance for both coordinates.
    /// # Examples
    /// ```
    /// use candle::math::{Vec2, ApproxEq};
    /// let vec1 = Vec2::new(1.0, 1.0);
    /// let vec2 = Vec2::new(0.99999999999999999999, 0.99999999999999999999);
    /// assert!(vec1.approx_eq(vec2));
    /// ```
    fn approx_eq(self, other: &Vec2) -> bool {
        self.approx_eq_eps(other, &VEC2_EPSILON)
    }
}

impl<'a, 'b> ApproxEq<&'a Vec2> for &'b Vec2 {
    fn approx_eq_eps(self, other: &Vec2, eps: &Vec2) -> bool {
        (self.x - other.x).abs() < eps.x && (self.y - other.y).abs() < eps.y
    }

    /// Returns true if the vector is approximately equal the other vector, with EPSILON amount
    /// of tolerance for both coordinates.
    /// # Examples
    /// ```
    /// use candle::math::{Vec2, ApproxEq};
    /// let vec1 = &Vec2::new(1.0, 1.0);
    /// let vec2 = Vec2::new(0.99999999999999999999, 0.99999999999999999999);
    /// assert!(vec1.approx_eq(vec2));
    /// ```
    fn approx_eq(self, other: &Vec2) -> bool {
        self.approx_eq_eps(other, &VEC2_EPSILON)
    }
}
