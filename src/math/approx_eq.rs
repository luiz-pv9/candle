use std::f64;

pub trait ApproxEq<T> {
    fn approx_eq(self, other: T) -> bool;
    fn approx_eq_eps(self, other: T, eps: T) -> bool;
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(self, other: Self) -> bool {
        self.approx_eq_eps(other, f64::EPSILON)
    }

    fn approx_eq_eps(self, other: Self, eps: Self) -> bool {
        (self - other).abs() < eps
    }
}
