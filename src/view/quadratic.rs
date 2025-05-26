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
use super::{Dir, Point};

//*****************************************************************************
pub struct Quadratic {
    p0: Point,
    p1: Point,
    p2: Point
}

impl Quadratic {
    //*************************************************************************
    pub fn new(
                p0: Point,
                p1: Point,
                p2: Point
            ) -> Self {
        Self { p0, p1, p2 }
    }

    //*************************************************************************
    fn for_t(
                &self,
                t:    isize,
                step: isize
            ) -> Point {
        let m = step - t;
        let a = m * m ;
        let b = 2 * m * t;
        let c = t * t;

        (self.p0 * a  + self.p1 * b + self.p2 * c) / step / step
    }

    //*****************************************************************************
    pub fn bounds(&self) -> (Point, Point) {
        let mut x_min = self.p0.x.min(self.p2.x);
        let mut x_max = self.p0.x.max(self.p2.x);
        let mut y_min = self.p0.y.min(self.p2.y);
        let mut y_max = self.p0.y.max(self.p2.y);

        let a = self.p0 - self.p1;
        let b = self.p0 - self.p1 * 2 + self.p2;

        if b.x != 0 && (a.x>0) == (b.x>0) {
            let t = a.x.abs();
            let s = b.x.abs();

            if t < s {
                let point = self.for_t(t, s);
                x_min = x_min.min(point.x);
                x_max = x_max.max(point.x);
                y_min = y_min.min(point.y);
                y_max = y_max.max(point.y);
            }
        }

        if b.y != 0 && (a.y>0) == (b.y>0) {
            let t = a.y.abs();
            let s = b.y.abs();

            if t < s {
                let point = self.for_t(t, s);
                x_min = x_min.min(point.x);
                x_max = x_max.max(point.x);
                y_min = y_min.min(point.y);
                y_max = y_max.max(point.y);
            }
        }

        (
            Point{ x: x_min, y: y_min },
            Point{ x: x_max, y: y_max }
        )
    }

    //*************************************************************************
    fn calc_step(&self) -> isize {
        let (min, max) = self.bounds();
        let x = min.x.abs_diff(max.x) as isize;
        let y = min.y.abs_diff(max.y) as isize;

        if x > y {
            2 * (2 * x + y)
        } else {
            2 * (2 * y + x)
        }
    }

    //*************************************************************************
    fn dir(
                from:  Point,
                to:    Point
            ) -> Result<Option<Dir>, ()> {
        const DIRS : [Option<Dir>; 9] = [
            Some(Dir::NorthWest), Some(Dir::North), Some(Dir::NorthEast),
            Some(Dir::West),      None,             Some(Dir::East),
            Some(Dir::SouthWest), Some(Dir::South), Some(Dir::SouthEast)
        ];
        let diff = to - from;

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            Err(())
        } else {
            Ok(
                DIRS[(diff.y + 1) as usize * 3 + (diff.x + 1) as usize]
            )
        }
    }

    //*************************************************************************
    pub fn path(
                &self
            ) -> Vec<Dir> {
        let resolution   = self.calc_step();
        let mut step     = resolution;

        let mut result = Vec::new();
        loop {
            let mut current = self.for_t(0, step);
            let mut good    = true;

            result.clear();
            for t in 1..=step {
                let next = self.for_t(t, step);

                if let Ok(wrapped) = Quadratic::dir(current, next) {
                    if let Some(dir) = wrapped {
                        result.push(dir);
                        current = next;
                    }
                } else {
                    good = false;
                    step = step + resolution;
                    break;
                }
            }
            if good == true { break }
        }

        result
    }
}

