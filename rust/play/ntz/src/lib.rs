use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    fmt::{Display, Formatter, Result},
    ops::{Add, /*Deref,*/ Div, Mul, Neg, Rem, Sub},
};

pub mod angle;
mod float;
pub mod si;

pub mod prelude {
    pub use super::{constant, unit, Unit, Value};
}

#[derive(Clone, Copy, Debug)]
pub struct Value<U: Unit> {
    pub value: f64,
    pub unit: U,
}

pub fn constant(value: f64) -> Value<ConstantUnit> {
    Value {
        value,
        unit: ConstantUnit(1.),
    }
}

pub struct ConstantUnit(f64);

impl Unit for ConstantUnit {
    fn factor(&self) -> f64 {
        1.
    }

    fn power(&self) -> Option<f64> {
        Some(self.0)
    }

    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        ConstantUnit(f(self.0))
    }
}

impl<U: Unit> Value<U> {
    pub fn sqrt(mut self) -> Self {
        Value {
            value: self.value.sqrt(),
            unit: self.unit.map_power(&|p| p / 2.),
        }
    }
}

impl<U: Unit> Value<U> {
    pub fn norm(&self) -> f64 {
        self.value * self.unit.factor()
    }
}

impl<U: Unit> Display for Value<U> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "{}", self.value)
    }
}

/*
impl<U: Unit> Deref for Value<U> {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.value
    }
}
*/

impl<U: Unit> PartialEq for Value<U> {
    fn eq(&self, other: &Self) -> bool {
        self.norm() == other.norm()
    }
}

impl<U: Unit> PartialOrd for Value<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.norm().partial_cmp(&other.norm())
    }
}

impl<U: Unit> Add<Value<U>> for Value<U> {
    type Output = Value<U>;

    fn add(self, rhs: Value<U>) -> Value<U> {
        let rval = rhs.norm() / self.unit.factor();
        Value {
            value: self.value + rval,
            unit: self.unit,
        }
    }
}

impl<U: Unit> Sub<Value<U>> for Value<U> {
    type Output = Value<U>;

    fn sub(self, rhs: Value<U>) -> Value<U> {
        let rval = rhs.norm() / self.unit.factor();
        Value {
            value: self.value - rval,
            unit: self.unit,
        }
    }
}

impl<L: Unit, R: Unit> Mul<Value<R>> for Value<L> {
    type Output = Value<MulUnit<L, R>>;

    fn mul(self, rhs: Value<R>) -> Value<MulUnit<L, R>> {
        Value {
            value: self.value * rhs.value,
            unit: MulUnit(self.unit, rhs.unit),
        }
    }
}

impl<L: Unit, R: Unit> Div<Value<R>> for Value<L> {
    type Output = Value<MulUnit<L, R>>;

    fn div(self, mut rhs: Value<R>) -> Value<MulUnit<L, R>> {
        Value {
            value: self.value / rhs.value,
            unit: MulUnit(self.unit, rhs.unit.map_power(&|p| -p)),
        }
    }
}

impl<L: Unit, R: Unit> Rem<Value<R>> for Value<L> {
    type Output = Value<MulUnit<L, R>>;

    fn rem(self, mut rhs: Value<R>) -> Value<MulUnit<L, R>> {
        Value {
            value: self.value % rhs.value,
            unit: MulUnit(self.unit, rhs.unit.map_power(&|p| -p)),
        }
    }
}

impl<U: Unit> Neg for Value<U> {
    type Output = Self;

    fn neg(self) -> Self {
        Value {
            value: -self.value,
            unit: self.unit,
        }
    }
}

pub trait Unit {
    fn factor(&self) -> f64;
    fn power(&self) -> Option<f64>;
    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(f64) -> f64;
}

#[derive(Clone, Copy, Debug)]
pub struct MulUnit<L: Unit, R: Unit>(L, R);

impl<L: Unit, R: Unit> Unit for MulUnit<L, R> {
    fn factor(&self) -> f64 {
        self.0.factor() * self.1.factor()
    }

    fn power(&self) -> Option<f64> {
        None //self.0.power() + self.1.power()
    }

    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        MulUnit(self.0.map_power(&f), self.1.map_power(&f))
    }
}

#[macro_export]
macro_rules! unit {
    ( $unit:ty { $( $var:ident => ($new:ident, $abbr:literal, $factor:expr) ),+ } ) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug)]
            pub enum $unit {
                $( $var(f64), )+
            }

            impl Unit for $unit {
                fn factor(&self) -> f64 {
                    match self {
                        $( $unit::$var(_) => $factor, )+
                    }
                }

                fn power(&self) -> Option<f64> {
                    match self {
                        $( $unit::$var(p) => Some(*p), )+
                    }
                }

                fn map_power<F>(&mut self, f: &F) -> $unit
                where
                    F: Fn(f64) -> f64,
                {
                    match self {
                        $(
                            $unit::$var(p) => {
                                *p = f(*p);
                                *self
                            },
                        )+
                    }
                }
            }

            $(
                #[allow(dead_code)]
                pub fn $new(value: f64) -> Value<$unit> {
                    Value {
                        value,
                        unit: $unit::$var(1.),
                    }
                }
            )+

            pub trait [<Convert$unit>] {
                $(
                    fn [<to_$new>](&self) -> Value<$unit>;
                )+
            }

            impl [<Convert$unit>] for Value<$unit> {
                $(
                    #[allow(dead_code)]
                    fn [<to_$new>](&self) -> Value<$unit> {
                        Value {
                            value: self.norm() / $unit::$var(1.).factor(),
                            unit: $unit::$var(1.),
                        }
                    }
                )+
            }

        }
    };
}
