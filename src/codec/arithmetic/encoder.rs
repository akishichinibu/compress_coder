use num::ToPrimitive;
use std::collections::VecDeque;
use super::index_tree::IndexTree;

trait CodecSpace<const LENGTH: usize> {
  const LENGTH: usize = LENGTH;
  const CL: usize = LENGTH >> 2;
  const CM: usize = LENGTH >> 1;
  const CR: usize = Self::CL + Self::CM;
}

pub struct ArithmeticEncoder<I, const LENGTH: usize> {
  iter: I,
  output_buffer: (VecDeque<u8>, usize),

  l: usize,
  r: usize,
  d: usize,
  follow: usize,
  index_tree: IndexTree<LENGTH>,
}

impl<I> CodecSpace<256> for ArithmeticEncoder<I, 256> where I: Iterator {}

impl<I, const LENGTH: usize> ArithmeticEncoder<I, LENGTH>
where
  I: Iterator,
{
  pub fn new(iter: I) -> ArithmeticEncoder<I, LENGTH> {
    ArithmeticEncoder {
      iter,
      output_buffer: (VecDeque::with_capacity(16), 0),
      l: 0,
      r: LENGTH,
      d: LENGTH,
      follow: 0,
      index_tree: IndexTree::<LENGTH>::new(),
    }
  }

  pub fn push(&mut self, bit: bool) -> (&VecDeque<u8>, usize) {
    let (ref mut ds, ref mut l) = self.output_buffer;

    let bv = if bit { 1 } else { 0 };

    if ds.len() == 0 || *l == 8 {
      ds.push_back(bv);
      *l = 1;
    } else {
      let last = ds.back_mut().unwrap();
      *last = (*last << 1) | bv;
      *l += 1;
    }

    (ds, *l)
  }

  pub fn pop(&mut self) -> Option<(u8, usize)> {
    let (ref mut ds, ref mut l) = self.output_buffer;

    let n = ds.len();

    if n == 0 {
      return None;
    } else {
      if n > 1 {
        Some((ds.pop_front().unwrap(), 8))
      } else {
        let padding_length = 8 - *l;
        let front = ds.pop_front().unwrap();
        return Some((front << padding_length, *l));
      }
    }
  }

  fn output_length(&self) -> usize {
    let (ref ds, ref l) = self.output_buffer;
    let n = ds.len();
    println!("{} {}", ds.len(), l);
    if n == 0 { 0 } else { n - (if *l == 8 { 0 } else { 1 }) }
  }

  fn try_output(&mut self) -> Option<u8> {
    if self.output_length() > 0 {
      let (d, _) = self.pop().unwrap();
      return Some(d);
    } else {
      return None;
    }
  }
}

impl<I, T> Iterator for ArithmeticEncoder<I, 256>
where
  I: Iterator<Item = T>,
  T: ToPrimitive,
{
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    let temp = self.try_output();

    if temp.is_some() {
      return temp;
    }

    let token = self.iter.next();

    match token {
      None => {
        let (d, l) = self.pop().unwrap_or((0, 0));
        return if l == 0 { None } else { Some(d) };
      }
      Some(rv) => {
        let v = rv.to_u8().unwrap() as usize;

        let sum_r = self.index_tree.accumulate_sum(v);
        let sum_l = sum_r - self.index_tree.at(v);

        let s = self.index_tree.sum();

        let ratio_r = (sum_r as f64) / (s as f64);
        let ratio_l = (sum_l as f64) / (s as f64);

        self.r = self.l + (ratio_r * (self.d as f64)) as usize;
        self.l = self.l + (ratio_l * (self.d as f64)) as usize;
        self.d = self.r - self.l;

        loop {
          if (self.l >= Self::CM) || (self.r < Self::CM) {
            let output_bit = self.l >= Self::CM;
            self.push(output_bit);

            if output_bit {
              self.r -= Self::CM;
              self.l -= Self::CM;
            }

            let follow_bit = !output_bit;

            for _ in 0..self.follow {
              self.push(follow_bit);
            }

            self.follow = 0;

          } else if (Self::CL <= self.l) || (self.r < Self::CR) {
            self.follow += 1;
            self.l -= Self::CL;
            self.r -= Self::CL;
          } else {
            break;
          }
        }

        self.r <<= 1;
        self.l <<= 1;
        self.d = self.r - self.l;
        self.index_tree.add(v, 1);
      }
    }

    let temp = self.try_output();
    return temp;
  }
}
