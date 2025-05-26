/*
Copyright 2025 Joshua E Gentry

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the “Software”), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is furnished
to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use crate::{Color, Def, Lines, PlotPoint};
use super::{Canvas, Cubic, Point, Quadratic, StrokeFactory};

//*****************************************************************************
pub struct Graph {
    strokes:   StrokeFactory,
    lstroke:   usize,
    astroke:   usize,
    thickness: i32,
    x_min:     f32,
    y_min:     f32,
    x_max:     f32,
    y_max:     f32,
    canvas:    Canvas,
    prepare:   bool
}

impl Graph {
    //*************************************************************************
    pub fn new(
                thickness: i32
            ) -> Self {
        Self {
            strokes:   StrokeFactory::new(),
            lstroke:   usize::MAX,
            astroke:   usize::MAX,
            x_min:     0.0,
            y_min:     0.0,
            x_max:     0.0,
            y_max:     0.0,
            canvas:    Canvas::new(100, 100),
            prepare:   true,
            thickness
        }
    }

    //*************************************************************************
    pub fn x_min(&self)  -> f32 { self.x_min }
    pub fn x_max(&self)  -> f32 { self.x_max }
    pub fn y_min(&self)  -> f32 { self.y_min }
    pub fn y_max(&self)  -> f32 { self.y_max }
    pub fn canvas(&self) -> &Canvas { &self.canvas }

    //*************************************************************************
    pub fn handle_thickness(
                &mut self,
                thickness: i32
            ) {
        if thickness & 0x1 == 1 {
            self.thickness = thickness;
        }
    }

    //*************************************************************************
    fn map(
                &mut self,
                p: PlotPoint
            ) -> Point {
        if self.prepare {
            self.x_min = self.x_min.min(p.x - self.thickness as f32);
            self.x_max = self.x_max.max(p.x + self.thickness as f32);
            self.y_min = self.y_min.min(p.y - self.thickness as f32);
            self.y_max = self.y_max.max(p.y + self.thickness as f32);
        }
        let x = p.x - self.x_min;
        let y = self.y_max - p.y;

        Point::new(x.trunc() as isize, y.trunc() as isize)
    }

    //*************************************************************************
    fn line(
                &mut self,
                c:  Color,
                p0: PlotPoint,
                p1: PlotPoint
            ) {
        let start = self.map(p0);
        let end   = self.map(p1);

        if !self.prepare {
            self.canvas.line(start, end, c.value());
        }
    }

    //*************************************************************************
    fn quadratic(
                &mut self,
                c:  Color,
                p0: PlotPoint,
                p1: PlotPoint,
                p2: PlotPoint,
            ) {

        if self.prepare {
            let curve = Quadratic::new(
                p0.to_point(),
                p1.to_point(),
                p2.to_point()
            );

            let (min, max) = curve.bounds();
            self.map(min.to_plot());
            self.map(max.to_plot());

        } else {
            let p0 = self.map(p0);
            let p1 = self.map(p1);
            let p2 = self.map(p2);

            self.canvas.quadratic(
                p0, p1, p2, c.value()
            );
        }
    }

    //*************************************************************************
    fn cubic(
                &mut self,
                c:  Color,
                p0: PlotPoint,
                p1: PlotPoint,
                p2: PlotPoint,
                p3: PlotPoint,
            ) {
        if self.prepare {
            let curve = Cubic::new(
                p0.to_point(),
                p1.to_point(),
                p2.to_point(),
                p3.to_point()
            );

            let (min, max) = curve.bounds();
            self.map(min.to_plot());
            self.map(max.to_plot());

        } else {
            let p0 = self.map(p0);
            let p1 = self.map(p1);
            let p2 = self.map(p2);
            let p3 = self.map(p3);

            self.canvas.cubic(
                p0, p1, p2, p3, c.value()
            );
        }
    }

    //*************************************************************************
    fn create_canvas(&mut self) {
        let width  = (self.x_max - self.x_min + 1.0) as usize;
        let height = (self.y_max - self.y_min + 1.0) as usize;

        if width == 0 && height == 0 {
            self.canvas = Canvas::new(100, 100);
        } else if width == 0 {
            self.canvas = Canvas::new(height, height);
        } else if height == 0 {
            self.canvas = Canvas::new(width, width);
        } else {
            self.canvas = Canvas::new(width, height);
        }
        self.lstroke = self.canvas.add_stroke(self.strokes.get(self.thickness as usize));
        self.astroke = self.canvas.add_stroke(self.strokes.get(3));
    }

    //*************************************************************************
    pub fn draw(
                &mut self,
                lines: &Lines
            ) {
        self.x_min = 0.0;
        self.x_max = 0.0;
        self.y_min = 0.0;
        self.y_max = 0.0;
        self.prepare = true;

        for line in lines.iter() {
            if line.is_enabled() {
                match line.def() {
                    Def::Line(c, p0, p1)          => self.line(*c, *p0, *p1),
                    Def::Quadratic(c, p0, p1, p2) => self.quadratic(*c, *p0, *p1, *p2),
                    Def::Cubic(c, p0, p1, p2, p3) => self.cubic(*c, *p0, *p1, *p2, *p3)
                }
            }
        }

        self.prepare = false;
        self.create_canvas();

        self.canvas.set_stroke(self.lstroke);
        for line in lines.iter() {
            if line.is_enabled() {
                match line.def() {
                    Def::Line(c, p0, p1)          => self.line(*c, *p0, *p1),
                    Def::Quadratic(c, p0, p1, p2) => self.quadratic(*c, *p0, *p1, *p2),
                    Def::Cubic(c, p0, p1, p2, p3) => self.cubic(*c, *p0, *p1, *p2, *p3)
                }
            }
        }
        self.canvas.set_stroke(self.astroke);
    }
}
