trait Quantity {
    type Number;

    fn value(&self) -> Self::Number;
    fn norm(&self) -> Self::Number;
}

trait Unit: Quantity {
    fn factor(&self) -> Self::Number;
    fn power(&self) -> Option<Self::Number>;
    fn map_power<F>(&mut self, f: &F) -> Self
    where
        F: Fn(Self::Number) -> Self::Number;
}

struct Value<T, U: Unit> {
    value: T,
    unit: U,
}

impl<T, U: Unit> Quantity for Value<T, U> {
    type Number = T;

    fn value(&self) -> Self::Number {
        self.value
    }

    fn norm(&self) -> Self::Number {
        self.value * self.unit.factor()
    }
}
