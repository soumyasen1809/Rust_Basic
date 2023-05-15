#[cfg(test)]
mod tests {
    use crate::{Rectangle, Point, rect_area};

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle{top_left: Point{x_coordinate: 0.0, y_coordinate: 10.0}, bottom_right: Point{x_coordinate: 10.0, y_coordinate: 0.0}};
        assert_eq!(rect_area(rect), 100.0);
    }
}