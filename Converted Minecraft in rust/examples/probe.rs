fn main(){
  let upper: i64 = 0x2ec82d1;
  println!("0x2ec82d1 << 27 = 0x{:x}", upper << 27);
  println!("+ 0x6a6ca89    = 0x{:x}", (upper << 27) + 0x6a6ca89);
  println!("java combined   = 0x1764168ea6ca89");
  // Rust `<<` on i64 with shift 27: does Rust mask the shift? No, i64 shift is fine.
  // but the JAVA `next(26)` returns an INT which is then cast to long and << 27.
  let combined = (upper << 27) + 0x6a6ca89;
  println!("rust product = 0x{:016x}", (combined as f64 * (1.110223E-16f32 as f64)).to_bits());
  // The difference: maybe java's `next(26)` result differs. Check: is upper NEGATIVE?
  println!("upper as int32 = {}", upper as i32);
}
