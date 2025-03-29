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
use std::ops::Add;

use font::Offset;

use super::Point;

//*****************************************************************************
#[derive(Debug, Clone, Copy)]
pub struct PlotPoint {
    pub x: f32,
    pub y: f32,
}

impl PlotPoint {
    //*************************************************************************
    pub fn new(
                x: f32,
                y: f32
            ) -> Self {
        Self { x, y }
    }

    //*************************************************************************
    pub fn to_point(&self) -> Point {
        Point::new(self.x.trunc() as isize, self.y.trunc() as isize)
    }
}

impl Add<Offset> for PlotPoint {
    type Output = PlotPoint;

    //*************************************************************************
    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}
