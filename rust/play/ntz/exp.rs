#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    fmt::{Display, Formatter, Result},
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};
use paste::paste;
fn main() {
    let inch = inches(22.);
    let gram = grams(99.);
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["sqrt of gram: ", "\n"],
            &match (&gram.sqrt(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
    let meter = meters(4.);
    let kilogram = kilograms(47.);
    if inch > meter {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", " greater than ", "\n"],
                &match (&inch.as_meters(), &meter) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
        };
    } else {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", " less than ", "\n"],
                &match (&inch.as_meters(), &meter) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
        };
    }
    let a_to_m = inches(42.);
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["a_to_m: ", "\n"],
            &match (&a_to_m.as_meters(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", " + ", " = ", "\n"],
            &match (&inch, &inch, &(inch + inch)) {
                (arg0, arg1, arg2) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                ],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", " * ", " = ", "\n"],
            &match (&inch, &gram, &(inch * gram)) {
                (arg0, arg1, arg2) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                ],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", " * ", " = ", "\n"],
            &match (&inch, &inch, &(inch * inch)) {
                (arg0, arg1, arg2) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                ],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", " / ", " = ", "\n"],
            &match (&inch, &gram, &(inch / gram)) {
                (arg0, arg1, arg2) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                ],
            },
        ));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&(((inch * gram) + (inch * gram) + (meter * kilogram)) * meter * kilogram),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
}
struct Value<U> {
    value: f64,
    unit: U,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<U: ::core::clone::Clone> ::core::clone::Clone for Value<U> {
    #[inline]
    fn clone(&self) -> Value<U> {
        match *self {
            Value {
                value: ref __self_0_0,
                unit: ref __self_0_1,
            } => Value {
                value: ::core::clone::Clone::clone(&(*__self_0_0)),
                unit: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<U: ::core::marker::Copy> ::core::marker::Copy for Value<U> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<U: ::core::fmt::Debug> ::core::fmt::Debug for Value<U> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Value {
                value: ref __self_0_0,
                unit: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Value");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "value", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "unit", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl<U: Unit> Value<U> {
    fn norm(&self) -> f64 {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["]]]]]]]]]]]]]", "/", "[[[[[[[[[[[[[[[[[[[[[\n"],
                &match (&self.value, &self.unit.factor()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
        };
        self.value * self.unit.factor()
    }
}
impl<U> Display for Value<U> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.write_fmt(::core::fmt::Arguments::new_v1(
            &[""],
            &match (&self.value,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ))
    }
}
impl<U> Deref for Value<U> {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.value
    }
}
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
impl Value<LengthUnit> {
    fn as_meters(&self) -> Value<LengthUnit> {
        Value {
            value: self.norm() / LengthUnit::Meter.factor(),
            unit: LengthUnit::Meter,
        }
    }
}
trait Unit {
    fn factor(&self) -> f64;
}
enum LengthUnit {
    AstronomicalUnit,
    CentImeter,
    Decimeter,
    Foot,
    Inch,
    Kilometer,
    LightYear,
    Meter,
    Mile,
    Millimeter,
    NauticalMile,
    Parsec,
    Yard,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for LengthUnit {
    #[inline]
    fn clone(&self) -> LengthUnit {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for LengthUnit {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for LengthUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&LengthUnit::AstronomicalUnit,) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "AstronomicalUnit");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::CentImeter,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "CentImeter");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Decimeter,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Decimeter");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Foot,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Foot");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Inch,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Inch");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Kilometer,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Kilometer");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::LightYear,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "LightYear");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Meter,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Meter");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Mile,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Mile");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Millimeter,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Millimeter");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::NauticalMile,) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "NauticalMile");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Parsec,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Parsec");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LengthUnit::Yard,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Yard");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl Unit for LengthUnit {
    fn factor(&self) -> f64 {
        match self {
            LengthUnit::AstronomicalUnit => 1.495979e11,
            LengthUnit::CentImeter => 0.01,
            LengthUnit::Decimeter => 0.1,
            LengthUnit::Foot => 12. * 0.0254,
            LengthUnit::Inch => 0.0254,
            LengthUnit::Kilometer => 1000.,
            LengthUnit::LightYear => 9.4607304725808e15,
            LengthUnit::Meter => 1.,
            LengthUnit::Mile => 1609.344,
            LengthUnit::Millimeter => 0.001,
            LengthUnit::NauticalMile => 1852.,
            LengthUnit::Parsec => 3.085678e16,
            LengthUnit::Yard => 0.9144,
        }
    }
}
#[allow(dead_code)]
fn astronomical_units(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::AstronomicalUnit,
    }
}
#[allow(dead_code)]
fn centimeters(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::CentImeter,
    }
}
#[allow(dead_code)]
fn decimeters(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Decimeter,
    }
}
#[allow(dead_code)]
fn feet(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Foot,
    }
}
#[allow(dead_code)]
fn inches(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Inch,
    }
}
#[allow(dead_code)]
fn kilometers(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Kilometer,
    }
}
#[allow(dead_code)]
fn light_years(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::LightYear,
    }
}
#[allow(dead_code)]
fn meters(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Meter,
    }
}
#[allow(dead_code)]
fn miles(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Mile,
    }
}
#[allow(dead_code)]
fn millimeters(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Millimeter,
    }
}
#[allow(dead_code)]
fn nautical_miles(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::NauticalMile,
    }
}
#[allow(dead_code)]
fn parsecs(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Parsec,
    }
}
#[allow(dead_code)]
fn yards(value: f64) -> Value<LengthUnit> {
    Value {
        value,
        unit: LengthUnit::Yard,
    }
}
enum MassUnit {
    Gram,
    Kilogram,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for MassUnit {
    #[inline]
    fn clone(&self) -> MassUnit {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for MassUnit {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MassUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&MassUnit::Gram,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Gram");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MassUnit::Kilogram,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Kilogram");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl Unit for MassUnit {
    fn factor(&self) -> f64 {
        match self {
            MassUnit::Gram => 0.001,
            MassUnit::Kilogram => 1.,
        }
    }
}
#[allow(dead_code)]
fn grams(value: f64) -> Value<MassUnit> {
    Value {
        value,
        unit: MassUnit::Gram,
    }
}
#[allow(dead_code)]
fn kilograms(value: f64) -> Value<MassUnit> {
    Value {
        value,
        unit: MassUnit::Kilogram,
    }
}
struct MulUnit<UnitL, UnitR>(UnitL, UnitR);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<UnitL: ::core::clone::Clone, UnitR: ::core::clone::Clone> ::core::clone::Clone
    for MulUnit<UnitL, UnitR>
{
    #[inline]
    fn clone(&self) -> MulUnit<UnitL, UnitR> {
        match *self {
            MulUnit(ref __self_0_0, ref __self_0_1) => MulUnit(
                ::core::clone::Clone::clone(&(*__self_0_0)),
                ::core::clone::Clone::clone(&(*__self_0_1)),
            ),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<UnitL: ::core::marker::Copy, UnitR: ::core::marker::Copy> ::core::marker::Copy
    for MulUnit<UnitL, UnitR>
{
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<UnitL: ::core::fmt::Debug, UnitR: ::core::fmt::Debug> ::core::fmt::Debug
    for MulUnit<UnitL, UnitR>
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            MulUnit(ref __self_0_0, ref __self_0_1) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "MulUnit");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
struct DivUnit<UnitL, UnitR>(UnitL, UnitR);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<UnitL: ::core::clone::Clone, UnitR: ::core::clone::Clone> ::core::clone::Clone
    for DivUnit<UnitL, UnitR>
{
    #[inline]
    fn clone(&self) -> DivUnit<UnitL, UnitR> {
        match *self {
            DivUnit(ref __self_0_0, ref __self_0_1) => DivUnit(
                ::core::clone::Clone::clone(&(*__self_0_0)),
                ::core::clone::Clone::clone(&(*__self_0_1)),
            ),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<UnitL: ::core::marker::Copy, UnitR: ::core::marker::Copy> ::core::marker::Copy
    for DivUnit<UnitL, UnitR>
{
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<UnitL: ::core::fmt::Debug, UnitR: ::core::fmt::Debug> ::core::fmt::Debug
    for DivUnit<UnitL, UnitR>
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            DivUnit(ref __self_0_0, ref __self_0_1) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "DivUnit");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl<U> Add<Value<U>> for Value<U> {
    type Output = Value<U>;
    fn add(self, rhs: Value<U>) -> Value<U> {
        Value {
            value: self.value + rhs.value,
            unit: self.unit,
        }
    }
}
impl<U> Sub<Value<U>> for Value<U> {
    type Output = Value<U>;
    fn sub(self, rhs: Value<U>) -> Value<U> {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &[">>>>>>>", "/", "<<", "<<<<<<<\n"],
                &match (&self.value, &rhs.value, &(self.value - rhs.value)) {
                    (arg0, arg1, arg2) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                    ],
                },
            ));
        };
        Value {
            value: self.value - rhs.value,
            unit: self.unit,
        }
    }
}
impl<UnitL, UnitR> Mul<Value<UnitR>> for Value<UnitL> {
    type Output = Value<MulUnit<UnitL, UnitR>>;
    fn mul(self, rhs: Value<UnitR>) -> Value<MulUnit<UnitL, UnitR>> {
        Value {
            value: self.value * rhs.value,
            unit: MulUnit(self.unit, rhs.unit),
        }
    }
}
impl<UnitL, UnitR> Div<Value<UnitR>> for Value<UnitL> {
    type Output = Value<MulUnit<UnitL, UnitR>>;
    fn div(self, rhs: Value<UnitR>) -> Value<MulUnit<UnitL, UnitR>> {
        Value {
            value: self.value / rhs.value,
            unit: MulUnit(self.unit, rhs.unit),
        }
    }
}
impl<U> Neg for Value<U> {
    type Output = Self;
    fn neg(self) -> Self {
        Value {
            value: -self.value,
            unit: self.unit,
        }
    }
}
