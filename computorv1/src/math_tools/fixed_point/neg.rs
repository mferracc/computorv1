use std::ops::Neg;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Neg for FixedPoint {
    type Output = FixedPoint;

    fn neg(self) -> FixedPoint {
        -&self
    }
}

impl Neg for &FixedPoint {
    type Output = FixedPoint;

    fn neg(self) -> FixedPoint {
        FixedPoint {
            integer: self.integer,
            decimal: self.decimal,
            sign: -self.sign,
            scale: self.scale,
        }
    }
}
