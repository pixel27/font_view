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
use std::slice::Iter;

use super::{Color, Def, Line, PlotPoint};

//*****************************************************************************
pub struct Lines {
    segments: Vec<Line>
}

impl Lines {
    //*************************************************************************
    pub fn new() -> Self {
        Self {
            segments: Vec::new()
        }
    }

    //*************************************************************************
    pub fn iter(&self) -> Iter<Line> {
        self.segments.iter()
    }

    //*************************************************************************
    pub fn clear(&mut self) {
        self.segments.clear();
    }

    //*************************************************************************
    pub fn handle_line_add(
                &mut self,
                def: Def
            ) {
        let line = Line::new(def);
        self.segments.push(line);
    }

    //*************************************************************************
    pub fn handle_line_change(
                &mut self,
                idx: usize,
                def: Def
            ) {
        self.segments[idx].change(def);
    }

    //*************************************************************************
    pub fn handle_line_toggle(
                &mut self,
                idx:    usize,
                enable: bool
            ) {
        self.segments[idx].enable(enable);
    }

    //*************************************************************************
    pub fn handle_line_remove(
                &mut self,
                idx: usize
            ) {
        self.segments.remove(idx);
    }

    //*************************************************************************
    pub fn add_line(
                &mut self,
                color: Color,
                p0:    PlotPoint,
                p1:    PlotPoint
            ) {
        let line = Line::new(
            Def::Line(color, p0, p1)
        );
        self.segments.push(line);
    }

    //*************************************************************************
    pub fn add_quadratic(
                &mut self,
                color: Color,
                p0:    PlotPoint,
                p1:    PlotPoint,
                p2:    PlotPoint
            ) {
        let curve = Line::new(
            Def::Quadratic(color, p0, p1, p2)
        );
        self.segments.push(curve);
    }

    //*************************************************************************
    pub fn add_cubic(
                &mut self,
                color: Color,
                p0:    PlotPoint,
                p1:    PlotPoint,
                p2:    PlotPoint,
                p3:    PlotPoint
            ) {
        let curve = Line::new(
            Def::Cubic(color, p0, p1, p2, p3)
        );
        self.segments.push(curve);
    }

}