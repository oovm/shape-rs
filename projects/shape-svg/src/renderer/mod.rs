use crate::SVG;
use graphics_core::{Graphics, GraphicsBackend, Line, Pixel, Point, Rectangle, StyleResolver};
use std::mem::take;

pub struct SvgRenderer {
    width: f32,
    height: f32,
    buffer: Vec<SVG>,
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self { width: 100.0, height: 100.0, buffer: Vec::new() }
    }
}

impl GraphicsBackend for SvgRenderer {
    type Output = SVG;
    type Error = String;

    fn get_output(&mut self, _: &Graphics) -> Result<Self::Output, Self::Error> {
        let attributes = &[
            ("width", format!("{}", self.width)),
            ("height", format!("{}", self.height)),
            ("viewBox", format!("0 0 {} {}", self.width, self.height)),
            ("xmlns", "http://www.w3.org/2000/svg".to_string()),
        ];
        Ok(SVG::new("svg", attributes, take(&mut self.buffer)))
    }

    fn on_start(&mut self, context: &Graphics, _: &mut StyleResolver) -> Result<(), Self::Error> {
        self.width = context.setting.width;
        self.height = context.setting.height;
        Ok(())
    }

    fn draw_pixel(&mut self, _: &Graphics, _: &mut StyleResolver, shape: &Pixel) -> Result<(), Self::Error> {
        let attributes = &[
            ("x", format!("{}", shape.x)),
            ("y", format!("{}", shape.y)),
            ("width", format!("{}", 1)),
            ("height", format!("{}", 1)),
            // ("fill", format!("{:#X}", shape.get_color(state))),
        ];
        let svg = SVG::new("rect", attributes, vec![]);
        Ok(self.buffer.push(svg))
    }

    fn draw_point(&mut self, _: &Graphics, state: &mut StyleResolver, shape: &Point) -> Result<(), Self::Error> {
        let attributes = &[
            ("cx", format!("{}", shape.get_x())),
            ("cy", format!("{}", shape.get_y())),
            ("r", format!("{}", shape.get_size(state))),
            ("fill", format!("{:#X}", shape.get_color(state))),
        ];
        let svg = SVG::new("circle", attributes, vec![]);
        Ok(self.buffer.push(svg))
    }

    fn draw_line(&mut self, _: &Graphics, _: &mut StyleResolver, _: &Line) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn draw_rectangle(&mut self, _: &Graphics, state: &mut StyleResolver, shape: &Rectangle) -> Result<(), Self::Error> {
        let attributes = &[
            ("x", format!("{}", shape.get_x())),
            ("y", format!("{}", shape.get_y())),
            ("width", format!("{}", shape.get_width())),
            ("height", format!("{}", shape.get_height())),
            ("fill", format!("{:#X}", shape.get_color(state))),
        ];
        let svg = SVG::new("rect", attributes, vec![]);
        Ok(self.buffer.push(svg))
    }
}
