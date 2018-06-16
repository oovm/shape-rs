#[test]
fn ready() {
    println!("it works!")
}

// #[test]
// fn test() {
//     let mut graphics = Graphics::default();
//     graphics.push(Point::new(0.0, 0.0));
//     graphics.push(Point::new(0.0, 1.0));
//     println!("{:?}", Rectangle::new(0.0, 1.0, 10.0, 6.18).with_color((127, 64, 130)));
//     graphics.push(Rectangle::new(0.0, 1.0, 10.0, 6.18).with_color((127, 64, 130)));
//
//     let mut renderer = SvgRenderer::default();
//     let out = graphics.render_with(&mut renderer).unwrap();
//     std::fs::write("tests/show.svg", format!("{}", out)).unwrap();
// }
