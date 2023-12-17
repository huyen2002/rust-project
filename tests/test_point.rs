
use crate::point::Point;
use crate::point::{add_point};
// test for add_point function
#[test]
fn check_add_point() {
  let point_1 = Point::init(1, 2);
  let point_2 = Point::init(3, 4);
  assert_eq!(add_point(point_1, point_2), Point::init(4, 6));
}