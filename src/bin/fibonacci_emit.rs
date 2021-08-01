use std::collections::HashSet;


fn main() {
  let mut fibs: Vec<u64> = Vec::with_capacity(256);
  
  fibs[0] = 1;
  fibs[1] = 1;
  fibs[2] = 2;

  for i in 3..256 {
    fibs[i] = fibs[i - 1] + fibs[i - 2];
  }

  let fibs_set = fibs.iter().cloned().collect::<HashSet<u64>>();

  let mut coding: Vec<(u64, usize)> = Vec::with_capacity(256);
  coding[0] = (0b11, 2);
  coding[1] = (0b101, 3);

  let mut t = 0;

  while true {
    

  }

  for i in 3..256 {
    let t = i - 1;

    let mut status: Vec<(u64, usize)> = Vec::new();
    status.push((t, 0));
    



  }
}
