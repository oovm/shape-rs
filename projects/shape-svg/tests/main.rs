use shape_core::{Ellipse, Point, Rectangle, Triangle};
use shape_svg::ToSVG;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    println!("{:#}", Point::new(100, 100).to_svg());
    println!("{:#}", Rectangle::new(100, 100, 1000, 618).to_svg());
    println!("{:#}", Triangle::new((100, 100), (400, 500), (800, 1000)).to_svg());
    println!("{:#}", Ellipse::new((100.0, 100.0), (50.0, 25.0), 1.0).to_svg());
}
