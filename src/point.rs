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
use std::ops::{Add, Sub, Mul, Div};

use super::PlotPoint;

//*****************************************************************************
#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    //*************************************************************************
    pub fn new(
                x: isize,
                y: isize
            ) -> Self {
        Self { x, y }
    }

    //*************************************************************************
    pub fn to_plot(&self) -> PlotPoint {
        PlotPoint::new(self.x as f32, self.y as f32)
    }
}

impl PartialEq for Point {
    //*************************************************************************
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    //*************************************************************************
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y

    }
}
impl Eq for Point {}

impl Add for Point {
    type Output = Point;

    //*************************************************************************
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<isize> for Point {
    type Output = Point;

    //*************************************************************************
    fn add(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Sub for Point {
    type Output = Point;

    //*************************************************************************
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Sub<isize> for Point {
    type Output = Point;

    //*************************************************************************
    fn sub(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl Mul for Point {
    type Output = Point;

    //*************************************************************************
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    //*************************************************************************
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div for Point {
    type Output = Point;

    //*************************************************************************
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Div<isize> for Point {
    type Output = Point;

    //*************************************************************************
    fn div(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}
