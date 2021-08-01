
use crate::iterator::IterArithmeticEncoder;
use crate::iterator::IterFibonacciEncode;

// #[test]
// fn it_works() {
//   println!("Here");
//   let s: Vec<u8> = vec![254, 33, 8];
//   // let b = s.iter().map(|x| *x).fibonacci_encode().collect::<Vec<u8>>();

//   let b2 = s
//     .iter()
//     .map(|x| *x)
//     .arithmetic_encode()
//     .collect::<Vec<u8>>();

//   println!("@@@ {:#?}", b2);
// }


// #[test]
// fn test_index_tree() {
//   let test_tree = cu::IndexTree(H);

//   test_tree.add(0, 1);
//   test_tree.add(1, 1);
//   test_tree.add(4, 5);
//   test_tree.add(5, 2);
//   test_tree.add(7, 6);

//   ASSERT_EQ(test_tree.find(-1), 8);
//   ASSERT_EQ(test_tree.find(3), 4);
//   ASSERT_EQ(test_tree.find(5), 4);
//   ASSERT_EQ(test_tree.find(8), 5);
//   ASSERT_EQ(test_tree.find(12), 7);
//   ASSERT_EQ(test_tree.find(13), 7);
//   ASSERT_EQ(test_tree.find(1000), 8);

// }