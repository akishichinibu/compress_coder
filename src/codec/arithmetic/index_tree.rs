pub struct IndexTree<const LENGTH: usize> {
  data: [usize; LENGTH],
  accum: [usize; LENGTH],
}

impl<const LENGTH: usize> IndexTree<LENGTH> {
  pub fn new() -> IndexTree<LENGTH> {
    IndexTree {
      data: [0; LENGTH],
      accum: [0; LENGTH],
    }
  }

  pub fn at(&self, i: usize) -> usize {
    self.data[i]
  }

  pub fn add(&mut self, _i: usize, val: usize) {
    let mut i = _i;

    i += 1;
    self.data[i - 1] += val;

    while i <= LENGTH {
      self.accum[i - 1] += val;
      i += IndexTree::<LENGTH>::low_bit(i);
    }
  }

  fn low_bit(num: usize) -> usize {
    let inum = num as isize;
    (inum & (-inum)) as usize
  }

  pub fn sum(&self) -> usize {
    self.accum[LENGTH - 1]
  }

  pub fn accumulate_sum(&self, _i: usize) -> usize {
    let mut i = _i;

    if i < 0 {
      return 0;
    }

    if i >= LENGTH {
      return self.sum();
    }

    let mut result = 0;

    i += 1;
    while i > 0 {
      result += self.accum[i - 1];
      i -= IndexTree::<LENGTH>::low_bit(i);
    }
    result
  }

  pub fn find(&self, s: usize) -> usize {
    
    if s < self.accum[0] {
      return 0;
    }

    if s >= self.accum[LENGTH - 1] {
      return LENGTH;
    }

    let mut r_l: usize = 0;
    let mut r_r: usize = LENGTH;

    while r_l + 1 != r_r {
      let r_m = (r_l + r_r) >> 1;
      let v_m = self.accumulate_sum(r_m);

      if s >= v_m {
        r_l = r_m;
      } else {
        r_r = r_m;
      }
    }
    return r_r;
  }
}

#[cfg(test)]
mod test {
  use super::IndexTree;

  #[test]
  fn test_index_tree() {
    let mut test_tree: IndexTree<8> = IndexTree::<8>::new();

    test_tree.add(0, 1);
    test_tree.add(1, 1);
    test_tree.add(4, 5);
    test_tree.add(5, 2);
    test_tree.add(7, 6);

    assert_eq!(test_tree.find(0), 0);
    assert_eq!(test_tree.find(3), 4);
    assert_eq!(test_tree.find(5), 4);
    assert_eq!(test_tree.find(8), 5);
    assert_eq!(test_tree.find(12), 7);
    assert_eq!(test_tree.find(13), 7);
    assert_eq!(test_tree.find(1000), 8);
  }
}
