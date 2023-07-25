pub fn diffli(li: &mut Vec<u64>) {
  li.sort();
  for i in (1..li.len()).rev() {
    li[i] = li[i] - li[i - 1];
  }
}
