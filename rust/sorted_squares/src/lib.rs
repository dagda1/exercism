pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
  let mut squared: Vec<i32> = nums.iter().map(|n| n * n).collect();

  squared.sort_by(|a, b| a.cmp(b));
  squared
}
