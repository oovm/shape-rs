use shape_core::{Ellipse, Point, Rectangle, Triangle};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    println!("{:#}", Point::new(100, 100).as_wolfram());
    println!("{:#}", Rectangle::new((100, 100), (1000, 618)).as_wolfram());
    println!("{:#}", Triangle::new([(100, 100), (400, 500), (800, 1000)]).as_wolfram());
    println!("{:#}", Ellipse::new((100.0, 100.0), (50.0, 25.0), 1.0).as_wolfram());
}
