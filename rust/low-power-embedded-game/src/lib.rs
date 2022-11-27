// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
    // we do not care about the value for this filter, so we use _ - i is checked in the closure as we are filtering on even indexes
    .filter(|(i, _)| i % 2 == 0)
    // takes a closure & creates an iterator from the closure, in this instance it provides the value of the iteration as an iterator
    .map(|(_i, v)| v)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
