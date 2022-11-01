use sorted_squares::*;

#[test]
/// empty string
fn test_simple() {
    let input = vec![-7,-3,2,3,11];
    let expected = vec![4,9,9,49,121];
    assert_eq!(&sorted_squares(input), &expected)
}
