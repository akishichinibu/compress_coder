use crate::codec::arithmetic::encoder::ArithmeticEncoder;
use crate::codec::fibonacci::encoder::FibonacciEncoder;

pub trait IterFibonacciEncode: Iterator {
  fn fibonacci_encode(self) -> FibonacciEncoder<Self>
  where
    Self: Sized,
  {
    FibonacciEncoder::new(self, 4096)
  }
}

impl<T: ?Sized> IterFibonacciEncode for T where T: Iterator {}

pub trait IterArithmeticEncoder: Iterator {
  fn arithmetic_encode(self) -> ArithmeticEncoder<Self, 256>
  where
    Self: Sized,
  {
    ArithmeticEncoder::new(self)
  }
}

impl<T: ?Sized> IterArithmeticEncoder for T where T: Iterator {}
