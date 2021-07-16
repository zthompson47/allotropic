use std::fmt::Debug;

use float_eq::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
};

use crate::{Unit, Value};

//#[derive(Debug, Clone, Copy, PartialEq)]
type ValueUlps = UlpsTol<f64>;
/*
pub struct ValueUlps<U: Unit> {
    value: UlpsTol<f64>,
    unit: U,
}
*/

impl<U: Unit> FloatEqUlpsTol for Value<U> {
    type UlpsTol = ValueUlps;
}

impl<U: Unit> FloatEq for Value<U> {
    type Tol = Value<U>;

    fn eq_abs(&self, other: &Self, tol: &Value<U>) -> bool {
        self.value.eq_abs(&other.value, &tol.value)
    }

    fn eq_rmax(&self, other: &Self, tol: &Value<U>) -> bool {
        self.value.eq_rmax(&other.value, &tol.value)
    }

    fn eq_rmin(&self, other: &Self, tol: &Value<U>) -> bool {
        self.value.eq_rmin(&other.value, &tol.value)
    }

    fn eq_r1st(&self, other: &Self, tol: &Value<U>) -> bool {
        self.value.eq_r1st(&other.value, &tol.value)
    }

    fn eq_r2nd(&self, other: &Self, tol: &Value<U>) -> bool {
        self.value.eq_r2nd(&other.value, &tol.value)
    }

    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Value<U>>) -> bool {
        self.value.eq_ulps(&other.value, &tol)
    }
}

impl<U: Unit> FloatEqAll for Value<U> {
    type AllTol = f64;

    fn eq_abs_all(&self, other: &Self, tol: &f64) -> bool {
        self.value.eq_abs_all(&other.value, tol)
    }

    fn eq_rmax_all(&self, other: &Self, tol: &f64) -> bool {
        self.value.eq_rmax_all(&other.value, tol)
    }

    fn eq_rmin_all(&self, other: &Self, tol: &f64) -> bool {
        self.value.eq_rmin_all(&other.value, tol)
    }

    fn eq_r1st_all(&self, other: &Self, tol: &f64) -> bool {
        self.value.eq_r1st_all(&other.value, tol)
    }

    fn eq_r2nd_all(&self, other: &Self, tol: &f64) -> bool {
        self.value.eq_r2nd_all(&other.value, tol)
    }

    fn eq_ulps_all(&self, other: &Self, tol: &UlpsTol<f64>) -> bool {
        self.value.eq_ulps_all(&other.value, tol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueDebugUlpsDiff<U: Unit> {
    value: DebugUlpsDiff<f64>,
    unit: U,
}

impl<U: Unit> FloatEqDebugUlpsDiff for Value<U> {
    type DebugUlpsDiff = ValueDebugUlpsDiff<U>;
}

impl<U> AssertFloatEq for Value<U>
where
    U: Unit + Debug + Copy,
{
    type DebugAbsDiff = Self;
    type DebugTol = Self;

    fn debug_abs_diff(&self, other: &Self) -> Value<U> {
        Value {
            value: self.value.debug_abs_diff(&other.value),
            unit: self.unit,
        }
    }

    fn debug_ulps_diff(&self, other: &Self) -> ValueDebugUlpsDiff<U> {
        ValueDebugUlpsDiff {
            value: self.value.debug_ulps_diff(&other.value),
            unit: self.unit,
        }
    }

    fn debug_abs_tol(&self, other: &Self, tol: &Value<U>) -> Value<U> {
        Value {
            value: self.value.debug_abs_tol(&other.value, &tol.value),
            unit: self.unit,
        }
    }

    fn debug_rmax_tol(&self, other: &Self, tol: &Value<U>) -> Value<U> {
        Value {
            value: self.value.debug_rmax_tol(&other.value, &tol.value),
            unit: self.unit,
        }
    }

    fn debug_rmin_tol(&self, other: &Self, tol: &Value<U>) -> Value<U> {
        Value {
            value: self.value.debug_rmin_tol(&other.value, &tol.value),
            unit: self.unit,
        }
    }

    fn debug_r1st_tol(&self, other: &Self, tol: &Value<U>) -> Value<U> {
        Value {
            value: self.value.debug_r1st_tol(&other.value, &tol.value),
            unit: self.unit,
        }
    }

    fn debug_r2nd_tol(&self, other: &Self, tol: &Value<U>) -> Value<U> {
        Value {
            value: self.value.debug_r2nd_tol(&other.value, &tol.value),
            unit: self.unit,
        }
    }

    fn debug_ulps_tol(&self, other: &Self, tol: &ValueUlps) -> ValueUlps {
        self.value.debug_ulps_tol(&other.value, &tol)
    }
}

impl<U> AssertFloatEqAll for Value<U>
where
    U: Unit + Debug + Copy,
{
    type AllDebugTol = Self;

    fn debug_abs_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Value {
            value: self.value.debug_abs_all_tol(&other.value, tol),
            unit: self.unit,
        }
    }

    fn debug_rmax_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Value {
            value: self.value.debug_rmax_all_tol(&other.value, tol),
            unit: self.unit,
        }
    }

    fn debug_rmin_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Value {
            value: self.value.debug_rmin_all_tol(&other.value, tol),
            unit: self.unit,
        }
    }

    fn debug_r1st_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Value {
            value: self.value.debug_r1st_all_tol(&other.value, tol),
            unit: self.unit,
        }
    }

    fn debug_r2nd_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
        Value {
            value: self.value.debug_r2nd_all_tol(&other.value, tol),
            unit: self.unit,
        }
    }

    fn debug_ulps_all_tol(
        &self,
        other: &Self,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol> {
        self.value.debug_ulps_all_tol(&other.value, tol)
    }
}

#[cfg(test)]
mod tests {
    mod u {
        use crate::prelude::*;

        unit! {
            Qaulity {
                Good => (goods, "gds", 1.),
                Better => (betters, "bts", 2.),
                Best => (bests, "bsts", 4.)
            }
        }
    }

    use float_eq::assert_float_eq;

    use crate::constant;

    use u::{goods, betters, bests};

    #[test]
    fn equality() {
        let a = goods(99.32);
        let b = betters(49.61);
        let c = bests(2.434234343);

        assert_float_eq!(a, a, ulps <= 1);
        assert_float_eq!(a, a, r2nd <= goods(f64::EPSILON));
        assert_float_eq!(a, b, r2nd <= goods(f64::EPSILON));
    }
}
