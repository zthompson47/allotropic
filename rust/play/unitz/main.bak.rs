use std::fmt::{Debug, Result};

fn main() {
    let i = Length::inches(42.);
    let m = Length::meters(2.);
    println!("i: {:?}, m: {:?}", &i, &m);

    let g = Mass::grams(47.);
    let k = Mass::kilograms(1.3);
    println!("g: {:?}, k: {:?}", &g, &k);

    let im = &i * &m;
    println!("{:?}", &im);

    let ig = &i * &g;
    println!("{:?}", &ig);
}

trait Value {
    fn value(&self) -> f64;
    fn unit(&self) -> Box<dyn Unit>;
}

impl Debug for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "[V v:{:?}, u:{:?}]",
            self.value(),
            self.unit()
        ))
    }
}

#[derive(Clone, Debug)]
struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Value for Length {
    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Box<dyn Unit> {
        Box::new(self.unit)
    }
}

impl Length {
    fn inches(value: f64) -> Box<dyn Value> {
        Box::new(Length {
            value,
            unit: LengthUnit::Inch,
        })
    }

    fn meters(value: f64) -> Box<dyn Value> {
        Box::new(Length {
            value,
            unit: LengthUnit::Meter,
        })
    }
}

#[derive(Clone, Debug)]
struct Mass {
    value: f64,
    unit: MassUnit,
}

impl Value for Mass {
    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Box<dyn Unit> {
        Box::new(self.unit)
    }
}

impl Mass {
    fn kilograms(value: f64) -> Box<dyn Value> {
        Box::new(Mass {
            value,
            unit: MassUnit::Kilogram,
        })
    }

    fn grams(value: f64) -> Box<dyn Value> {
        Box::new(Mass {
            value,
            unit: MassUnit::Gram,
        })
    }
}

#[derive(Clone)]
struct Derived<L, R>
where
    L: Unit + Clone,
    R: Unit + Clone,
{
    value: f64,
    unit: DerivedUnit<L, R>,
}

impl<L, R> Value for Derived<L, R>
where
    L: Unit + 'static + Clone,
    R: Unit + 'static + Clone,
{
    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Box<dyn Unit> {
        Box::new(DerivedUnit {
            lhs: self.unit.lhs.clone(),
            rhs: self.unit.rhs.clone(),
        })
    }
}

impl std::ops::Mul for &Box<dyn Value> {
    type Output = Box<dyn Value>;

    fn mul(self, rhs: Self) -> Box<dyn Value> {
        Box::new(Derived {
            value: self.value() * rhs.value(),
            unit: DerivedUnit {
                //lhs: MassUnit::Gram,
                //rhs: MassUnit::Gram,
                lhs: self.unit().clone(),
                rhs: rhs.unit().clone(),
            },
        })
    }
}

trait Unit {
    fn map(&self) -> (f64, String);

    fn factor(&self) -> f64 {
        self.map().0
    }

    fn label(&self) -> String {
        self.map().1
    }
}

impl Debug for dyn Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "[U f:{:?}, n:{:?}]",
            self.factor(),
            self.label()
        ))
    }
}

#[derive(Clone, Copy, Debug)]
enum LengthUnit {
    Inch,
    Meter,
}

#[derive(Clone, Copy, Debug)]
enum MassUnit {
    Kilogram,
    Gram,
}

#[derive(Clone)]
struct DerivedUnit<L: Unit + Clone, R: Unit + Clone> {
    lhs: L,
    rhs: R,
}

struct DerivedPart {
    factor: f64,
    label: String,
}

impl Unit for DerivedPart {
    fn map(&self) -> (f64, String) {
        (self.factor, self.label)
    }
}

impl<L: Unit + Clone + 'static, R: Unit + Clone + 'static> Debug for DerivedUnit<L, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "[f:{:?}, n:{:?}]",
            self.factor(),
            self.label()
        ))
    }
}

impl<L: Unit + Clone + 'static, R: Unit + Clone + 'static> Unit for DerivedUnit<L, R> {
    fn map(&self) -> (f64, String) {
        (
            self.lhs.factor() * self.rhs.factor(),
            format!("({} * {})", self.lhs.label(), self.rhs.label()),
        )
    }
}

impl Unit for LengthUnit {
    fn map(&self) -> (f64, String) {
        match self {
            LengthUnit::Inch => (0.0254, String::from("in")),
            LengthUnit::Meter => (1., String::from("m")),
        }
    }
}

impl Unit for MassUnit {
    fn map(&self) -> (f64, String) {
        match self {
            MassUnit::Kilogram => (1., String::from("kg")),
            MassUnit::Gram => (0.001, String::from("g")),
        }
    }
}
