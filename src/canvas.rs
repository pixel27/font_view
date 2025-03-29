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
use std::mem;
use iced::widget::image::Handle;

use super::{Cubic, Point, Quadratic};

use paths::Paths;
use stroke::Stroke;

pub use stroke_factory::StrokeFactory;
//*****************************************************************************
mod stroke;
mod stroke_factory;
mod paths;

//*****************************************************************************
#[derive(Clone, Copy)]
pub enum Dir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

//*****************************************************************************
pub struct Canvas {
    width:   usize,
    height:  usize,
    paths:   Vec<Paths>,
    current: usize,
    data:    Vec<u32>
}

impl Canvas {
    //*************************************************************************
    pub fn new(
                width:  usize,
                height: usize
            ) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, 0xffffffff);

        Self {
            width, height, data,
            paths:   Vec::new(),
            current: usize::MAX
        }
    }

    //*************************************************************************
    pub fn width(&self)  -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    //*************************************************************************
    pub fn add_stroke(
                &mut self,
                stroke: &Stroke
            ) -> usize {
        self.paths.push(
            Paths::new(stroke, self.width)
        );
        self.paths.len() - 1
    }

    //*************************************************************************
    pub fn set_stroke(
                &mut self,
                stroke: usize
            ) {
        if stroke >= self.paths.len() {
            panic!("Unknown stroke.");
        }
        self.current = stroke;
    }

    //*************************************************************************
    fn write_full(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_full(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_north(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_north(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_northeast(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_northeast(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_east(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_east(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_southeast(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_southeast(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_south(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_south(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_southwest(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_southwest(&mut self.data, offset, color);
    }

        //*************************************************************************
    fn write_west(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_west(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn write_northwest(
                &mut self,
                offset: usize,
                color:  u32
            ) {
        self.paths[self.current].write_northwest(&mut self.data, offset, color);
    }

    //*************************************************************************
    fn line_horz(
                &mut self,
                x0:    usize,
                x1:    usize,
                y:     usize,
                color: u32
            ) {
        assert!(x0 < x1);

        let mut offset = y * self.width + x0;

        self.write_full(offset, color);

        for _ in x0..x1 {
            offset += 1;
            self.write_east(offset, color);
        }
    }

    //*************************************************************************
    fn line_vert(
                &mut self,
                y0:    usize,
                y1:    usize,
                x:     usize,
                color: u32
            ) {
        assert!(y0 < y1);
        let mut offset = (y0 * self.width) + x;

        self.write_full(offset, color);

        for _ in y0..y1 {
            offset += self.width;
            self.write_south(offset, color);
        }
    }

    //*************************************************************************
    pub fn line_x_downward(
                &mut self,
                p0:    Point,
                p1:    Point,
                color: u32
            ) {
        let dx = p1.x as isize - p0.x as isize;
        let dy = p1.y as isize - p0.y as isize;
        let mut d = (2 * dy) - dx;

        let mut offset = p0.y as usize * self.width + p0.x as usize;

        self.write_full(offset, color);
        for _ in p0.x..p1.x {
            if d > 0 {
                offset += self.width + 1;
                d -= 2 * dx;
                self.write_southeast(offset, color);
            } else {
                offset += 1;
                self.write_east(offset, color);
            }
            d += 2 * dy;
        }
    }

    //*************************************************************************
    pub fn line_x_upward(
                &mut self,
                p0:    Point,
                p1:    Point,
                color: u32
            ) {
        let dx =   p1.x as isize - p0.x as isize;
        let dy = -(p1.y as isize - p0.y as isize);
        let mut d = (2 * dy) - dx;

        let mut offset = p0.y as usize * self.width + p0.x as usize;

        self.write_full(offset, color);
        for _ in p0.x..p1.x {
            if d > 0 {
                offset -= self.width - 1;
                d -= 2 * dx;
                self.write_northeast(offset, color);
            } else {
                offset += 1;
                self.write_east(offset, color);
            }
            d += 2 * dy;
        }
    }

    //*************************************************************************
    pub fn line_y_rightward(
                &mut self,
                p0:    Point,
                p1:    Point,
                color: u32
            ) {
        let dx = p1.x as isize - p0.x as isize;
        let dy = p1.y as isize - p0.y as isize;
        let mut d = (2 * dx) - dy;

        let mut offset = p0.y as usize * self.width + p0.x as usize;
        self.write_full(offset, color);

        for _ in p0.y..p1.y {
            if d > 0 {
                offset += 1 + self.width;
                d -= 2 * dy;
                self.write_southeast(offset, color);
            } else {
                offset += self.width;
                self.write_south(offset, color);
            }
            d += 2 * dx;
        }
    }

    //*************************************************************************
    pub fn line_y_leftward(
                &mut self,
                p0:    Point,
                p1:    Point,
                color: u32
            ) {
        let dx = -(p1.x as isize - p0.x as isize);
        let dy = p1.y as isize - p0.y as isize;
        let mut d = (2 * dx) - dy;

        let mut offset = p0.y as usize * self.width + p0.x as usize;
        self.write_full(offset, color);

        for _ in p0.y..p1.y {
            if d > 0 {
                offset += self.width - 1;
                d -= 2 * dy;
                self.write_southwest(offset, color);
            } else {
                offset += self.width;
                self.write_south(offset, color);
            }
            d += 2 * dx;

        }
    }

    //*************************************************************************
    pub fn line(
                &mut self,
                p0:    Point,
                p1:    Point,
                color: u32
            ) {
        if p0 == p1 { return }

        if p0.y == p1.y { // Horizontal Line
            if p0.x < p1.x {
                self.line_horz(p0.x as usize, p1.x as usize, p0.y as usize, color);
            } else {
                self.line_horz(p1.x as usize, p0.x as usize, p0.y as usize, color);
            }
            return;
        } else if p0.x == p1.x { // Vertical Line
            if p0.y < p1.y {
                self.line_vert(p0.y as usize, p1.y as usize, p0.x as usize, color);
            } else {
                self.line_vert(p1.y as usize, p0.y as usize, p0.x as usize, color);
            }
            return;
        }


        if p0.x < p1.x {
            if p0.y < p1.y {
                if p1.x - p0.x >= p1.y - p0.y {
                    self.line_x_downward(p0, p1, color);
                } else {
                    self.line_y_rightward(p0, p1, color);
                }
            } else {
                if p1.x - p0.x >= p0.y - p1.y {
                    self.line_x_upward(p0, p1, color);
                } else {
                    self.line_y_leftward(p1, p0, color);
                }
            }
        } else {
            if p1.y < p0.y {
                if p0.x - p1.x >= p0.y - p1.y {
                    self.line_x_downward(p1, p0, color);
                } else {
                    self.line_y_rightward(p1, p0, color);
                }
            } else {
                if p0.x - p1.x >= p1.y - p0.y {
                    self.line_x_upward(p1, p0, color);
                } else {
                    self.line_y_leftward(p0, p1, color);
                }
            }
        }
    }

    //*************************************************************************
    pub fn quadratic(
                &mut self,
                p0:    Point,
                p1:    Point,
                p2:    Point,
                color: u32
            ) {
        let mut offset = (p0.y * self.width as isize + p0.x) as usize;
        self.write_full(offset, color);

        let curve = Quadratic::new(p0, p1, p2);

        for &dir in curve.path().iter() {
            match dir {
                Dir::North => {
                    offset = offset - self.width;
                    self.write_north(offset, color);
                },
                Dir::NorthEast => {
                    offset = offset - self.width + 1;
                    self.write_northeast(offset, color);
                },
                Dir::East => {
                    offset = offset + 1;
                    self.write_east(offset, color);
                },
                Dir::SouthEast => {
                    offset = offset + self.width + 1;
                    self.write_southeast(offset, color);
                },
                Dir::South => {
                    offset = offset + self.width;
                    self.write_south(offset, color);
                },
                Dir::SouthWest => {
                    offset = offset + self.width - 1;
                    self.write_southwest(offset, color);
                },
                Dir::West => {
                    offset = offset - 1;
                    self.write_west(offset, color);
                },
                Dir::NorthWest => {
                    offset = offset - self.width - 1;
                    self.write_northwest(offset, color);
                },
            }
        }
    }

    //*************************************************************************
    pub fn cubic(
                &mut self,
                p0:    Point,
                p1:    Point,
                p2:    Point,
                p3:    Point,
                color: u32
            ) {
        let mut offset = (p0.y * self.width as isize + p0.x) as usize;
        self.write_full(offset, color);

        let curve = Cubic::new(p0, p1, p2, p3);

        for &dir in curve.path().iter() {
            match dir {
                Dir::North => {
                    offset = offset - self.width;
                    self.write_north(offset, color);
                },
                Dir::NorthEast => {
                    offset = offset - self.width + 1;
                    self.write_northeast(offset, color);
                },
                Dir::East => {
                    offset = offset + 1;
                    self.write_east(offset, color);
                },
                Dir::SouthEast => {
                    offset = offset + self.width + 1;
                    self.write_southeast(offset, color);
                },
                Dir::South => {
                    offset = offset + self.width;
                    self.write_south(offset, color);
                },
                Dir::SouthWest => {
                    offset = offset + self.width - 1;
                    self.write_southwest(offset, color);
                },
                Dir::West => {
                    offset = offset - 1;
                    self.write_west(offset, color);
                },
                Dir::NorthWest => {
                    offset = offset - self.width - 1;
                    self.write_northwest(offset, color);
                },
            }
        }
    }


    //*************************************************************************
    pub fn handle(&self) -> Handle {
        assert!(mem::size_of::<u32>() == 4, "We expect u32 to be 4 bytes.");
        assert!(mem::size_of::<u8>()  == 1, "We expect u8 to be 1 byte.");

        let mut copy = self.data.clone();

        let pixels = unsafe {
            let ptr      = copy.as_mut_ptr() as *mut u8;
            let len      = copy.len() * 4;
            let capacity = copy.capacity() * 4;

            mem::forget(copy);

            Vec::from_raw_parts(ptr, len, capacity)
        };
        Handle::from_rgba(self.width as u32, self.height as u32, pixels)
    }
}