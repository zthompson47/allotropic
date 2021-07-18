use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    fmt::{Display, Formatter, Result},
    ops::{Add, /*Deref,*/ Div, Mul, Neg, Rem, Sub},
};

pub mod angle;
mod float;
pub mod si;
pub mod temperature;
pub mod time;

pub mod prelude {
    pub use super::{unit, Unit, Value};
}

#[derive(Clone, Copy)]
pub struct Value<U: Unit> {
    pub value: f64,
    pub unit: U,
}

impl<U: Unit + std::fmt::Debug> std::fmt::Debug for Value<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("norm", &self.norm())
            .field("value", &self.value)
            .field("unit", &self.unit)
            .finish()
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
        self.value * self.unit.factor() + self.unit.offset()
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
    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(f64) -> f64;
    fn offset(&self) -> f64;
    fn power(&self) -> Option<f64>;
}

#[derive(Clone, Copy, Debug)]
pub struct MulUnit<L: Unit, R: Unit>(L, R);

impl<L: Unit, R: Unit> Unit for MulUnit<L, R> {
    fn factor(&self) -> f64 {
        self.0.factor() * self.1.factor()
    }

    fn offset(&self) -> f64 {
        // TODO: this is wrong..  need to have a `convert()` method or something
        // to calculate based on offset and factor..
        self.0.offset() + self.1.offset()
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
    ( $unit:ty { $( $var:ident => ($new:ident, $abbr:literal, $factor:expr $(, $offset:expr )* ) ),+ } ) => {
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

                fn offset(&self) -> f64 {
                    match self {
                        $( $unit::$var(_) => 0. $( + $offset )*, )+
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

            impl Value<$unit> {
                $(
                    #[allow(dead_code)]
                    pub fn [<to_$new>](self) -> Value<$unit> {
                        Value {
                            value: (self.norm() $( - $offset )*) / $unit::$var(1.).factor(),
                            unit: $unit::$var(1.),
                        }
                    }
                )+
            }

        }
    };
}
