use num::ToPrimitive;

pub const ENCODE_TABLE: [(usize, usize); 256] = [
  (2, 0x1),
  (2, 0x3),
  (3, 0x3),
  (4, 0x3),
  (4, 0xb),
  (5, 0x3),
  (5, 0x13),
  (5, 0xb),
  (6, 0x3),
  (6, 0x23),
  (6, 0x13),
  (6, 0xb),
  (6, 0x2b),
  (7, 0x3),
  (7, 0x43),
  (7, 0x23),
  (7, 0x13),
  (7, 0x53),
  (7, 0xb),
  (7, 0x4b),
  (7, 0x2b),
  (8, 0x3),
  (8, 0x83),
  (8, 0x43),
  (8, 0x23),
  (8, 0xa3),
  (8, 0x13),
  (8, 0x93),
  (8, 0x53),
  (8, 0xb),
  (8, 0x8b),
  (8, 0x4b),
  (8, 0x2b),
  (8, 0xab),
  (9, 0x3),
  (9, 0x103),
  (9, 0x83),
  (9, 0x43),
  (9, 0x143),
  (9, 0x23),
  (9, 0x123),
  (9, 0xa3),
  (9, 0x13),
  (9, 0x113),
  (9, 0x93),
  (9, 0x53),
  (9, 0x153),
  (9, 0xb),
  (9, 0x10b),
  (9, 0x8b),
  (9, 0x4b),
  (9, 0x14b),
  (9, 0x2b),
  (9, 0x12b),
  (9, 0xab),
  (10, 0x3),
  (10, 0x203),
  (10, 0x103),
  (10, 0x83),
  (10, 0x283),
  (10, 0x43),
  (10, 0x243),
  (10, 0x143),
  (10, 0x23),
  (10, 0x223),
  (10, 0x123),
  (10, 0xa3),
  (10, 0x2a3),
  (10, 0x13),
  (10, 0x213),
  (10, 0x113),
  (10, 0x93),
  (10, 0x293),
  (10, 0x53),
  (10, 0x253),
  (10, 0x153),
  (10, 0xb),
  (10, 0x20b),
  (10, 0x10b),
  (10, 0x8b),
  (10, 0x28b),
  (10, 0x4b),
  (10, 0x24b),
  (10, 0x14b),
  (10, 0x2b),
  (10, 0x22b),
  (10, 0x12b),
  (10, 0xab),
  (10, 0x2ab),
  (11, 0x3),
  (11, 0x403),
  (11, 0x203),
  (11, 0x103),
  (11, 0x503),
  (11, 0x83),
  (11, 0x483),
  (11, 0x283),
  (11, 0x43),
  (11, 0x443),
  (11, 0x243),
  (11, 0x143),
  (11, 0x543),
  (11, 0x23),
  (11, 0x423),
  (11, 0x223),
  (11, 0x123),
  (11, 0x523),
  (11, 0xa3),
  (11, 0x4a3),
  (11, 0x2a3),
  (11, 0x13),
  (11, 0x413),
  (11, 0x213),
  (11, 0x113),
  (11, 0x513),
  (11, 0x93),
  (11, 0x493),
  (11, 0x293),
  (11, 0x53),
  (11, 0x453),
  (11, 0x253),
  (11, 0x153),
  (11, 0x553),
  (11, 0xb),
  (11, 0x40b),
  (11, 0x20b),
  (11, 0x10b),
  (11, 0x50b),
  (11, 0x8b),
  (11, 0x48b),
  (11, 0x28b),
  (11, 0x4b),
  (11, 0x44b),
  (11, 0x24b),
  (11, 0x14b),
  (11, 0x54b),
  (11, 0x2b),
  (11, 0x42b),
  (11, 0x22b),
  (11, 0x12b),
  (11, 0x52b),
  (11, 0xab),
  (11, 0x4ab),
  (11, 0x2ab),
  (12, 0x3),
  (12, 0x803),
  (12, 0x403),
  (12, 0x203),
  (12, 0xa03),
  (12, 0x103),
  (12, 0x903),
  (12, 0x503),
  (12, 0x83),
  (12, 0x883),
  (12, 0x483),
  (12, 0x283),
  (12, 0xa83),
  (12, 0x43),
  (12, 0x843),
  (12, 0x443),
  (12, 0x243),
  (12, 0xa43),
  (12, 0x143),
  (12, 0x943),
  (12, 0x543),
  (12, 0x23),
  (12, 0x823),
  (12, 0x423),
  (12, 0x223),
  (12, 0xa23),
  (12, 0x123),
  (12, 0x923),
  (12, 0x523),
  (12, 0xa3),
  (12, 0x8a3),
  (12, 0x4a3),
  (12, 0x2a3),
  (12, 0xaa3),
  (12, 0x13),
  (12, 0x813),
  (12, 0x413),
  (12, 0x213),
  (12, 0xa13),
  (12, 0x113),
  (12, 0x913),
  (12, 0x513),
  (12, 0x93),
  (12, 0x893),
  (12, 0x493),
  (12, 0x293),
  (12, 0xa93),
  (12, 0x53),
  (12, 0x853),
  (12, 0x453),
  (12, 0x253),
  (12, 0xa53),
  (12, 0x153),
  (12, 0x953),
  (12, 0x553),
  (12, 0xb),
  (12, 0x80b),
  (12, 0x40b),
  (12, 0x20b),
  (12, 0xa0b),
  (12, 0x10b),
  (12, 0x90b),
  (12, 0x50b),
  (12, 0x8b),
  (12, 0x88b),
  (12, 0x48b),
  (12, 0x28b),
  (12, 0xa8b),
  (12, 0x4b),
  (12, 0x84b),
  (12, 0x44b),
  (12, 0x24b),
  (12, 0xa4b),
  (12, 0x14b),
  (12, 0x94b),
  (12, 0x54b),
  (12, 0x2b),
  (12, 0x82b),
  (12, 0x42b),
  (12, 0x22b),
  (12, 0xa2b),
  (12, 0x12b),
  (12, 0x92b),
  (12, 0x52b),
  (12, 0xab),
  (12, 0x8ab),
  (12, 0x4ab),
  (12, 0x2ab),
  (12, 0xaab),
  (13, 0x3),
  (13, 0x1003),
  (13, 0x803),
  (13, 0x403),
  (13, 0x1403),
  (13, 0x203),
  (13, 0x1203),
  (13, 0xa03),
  (13, 0x103),
  (13, 0x1103),
  (13, 0x903),
  (13, 0x503),
  (13, 0x1503),
  (13, 0x83),
  (13, 0x1083),
  (13, 0x883),
  (13, 0x483),
  (13, 0x1483),
  (13, 0x283),
  (13, 0x1283),
  (13, 0xa83),
  (13, 0x43),
  (13, 0x1043),
];

pub struct FibonacciEncoder<I> {
  iter: I,

  stat: [usize; 256],
  symbol_map: [usize; 256],

  t: usize,
  block_size: usize,
  stat_buffer: Vec<u8>,

  output_buffer: (usize, usize),
}

impl<I> FibonacciEncoder<I>
where
  I: Iterator,
{
  pub fn new(iter: I, block_size: usize) -> FibonacciEncoder<I> {
    let mut symbol_map: [usize; 256] = [0; 256];

    for i in 0..256 {
      symbol_map[i] = i;
    }

    FibonacciEncoder {
      iter,
      stat: [0; 256],
      symbol_map,

      t: 0,
      block_size,
      stat_buffer: Vec::with_capacity(block_size),

      output_buffer: (0, 0),
    }
  }
}

impl<I, T> Iterator for FibonacciEncoder<I>
where
  I: Iterator<Item = T>,
  T: ToPrimitive,
{
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    while self.stat_buffer.len() < self.block_size {
      let token = self.iter.next();

      match token {
        Some(rv) => {
          let v = rv.to_u8().unwrap();
          println!("!! read {}", v);
          self.stat_buffer.push(v);
          self.stat[v as usize] += 1;
        }
        None => break,
      }
    }

    let n = self.stat_buffer.len();

    if self.t == 0 {
      let mut idx = (0..256).collect::<Vec<_>>();
      idx.sort_by(|&a, &b| self.stat[b].cmp(&self.stat[a]));
      idx
        .iter()
        .enumerate()
        .for_each(|(i, &v)| self.symbol_map[i] = v);
    }

    let (b, l) = self.output_buffer;
    println!("{} @@@ {} {} {}", n, b, l, self.t);

    if l < 8 {
      if self.t < n {
        let (el, eb) = ENCODE_TABLE[self.symbol_map[self.stat_buffer[self.t] as usize]];
        self.output_buffer = (((b as usize) << el) | eb, l + el);
        self.t += 1;
      }
    }

    if l == 0 {
      return None;
    } else {
      let ob = (b & 0b11111111) as u8;
      let ol = if l >= 8 { 8 } else { l };
      self.output_buffer = (b >> 8, l - ol);
      println!("{} {}", ob, ol);
      return Some(ob);
    }
  }
}
