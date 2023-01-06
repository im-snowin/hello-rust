

fn main() {
  let mut point: (i64, i32, i8) = (120, 32, 2);
  
  let x = point.0;
  let y = point.1;
  let z = point.2;

  println!("{}, {}, {}", x,y,z);

  println!("{:?}", point);

  point.0 = 2000;

  println!("{:?}", point)
  
}