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
use super::Stroke;

//*****************************************************************************
pub struct Paths {
    full:      Vec<isize>,
    north:     Vec<isize>,
    northeast: Vec<isize>,
    east:      Vec<isize>,
    southeast: Vec<isize>,
    south:     Vec<isize>,
    southwest: Vec<isize>,
    west:      Vec<isize>,
    northwest: Vec<isize>
}

impl Paths {
    //*************************************************************************
    pub fn new(
                stroke: &Stroke,
                stride: usize
            ) -> Self {
        Self {
            full:      stroke.full(stride),
            north:     stroke.north(stride),
            northeast: stroke.north_east(stride),
            east:      stroke.east(stride),
            southeast: stroke.south_east(stride),
            south:     stroke.south(stride),
            southwest: stroke.south_west(stride),
            west:      stroke.west(stride),
            northwest: stroke.north_west(stride),
        }
    }

    //*************************************************************************
    fn write(
                target: &mut [u32],
                offset: usize,
                color:  u32,
                path:   &[isize],
            ) {
        for &diff in path.iter() {
            target[(offset as isize + diff) as usize] = color;
        }
    }

    //*************************************************************************
    pub fn write_full(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.full);
    }

    //*************************************************************************
    pub fn write_north(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.north);
    }

    //*************************************************************************
    pub fn write_northeast(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.northeast);
    }

    //*************************************************************************
    pub fn write_east(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.east);
    }

    //*************************************************************************
    pub fn write_southeast(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.southeast);
    }

    //*************************************************************************
    pub fn write_south(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.south);
    }

    //*************************************************************************
    pub fn write_southwest(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.southwest);
    }

    //*************************************************************************
    pub fn write_west(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.west);
    }

    //*************************************************************************
    pub fn write_northwest(
                &self,
                target: &mut [u32],
                offset: usize,
                color:  u32
            ) {
        Paths::write(target, offset, color, &self.northwest);
    }

}
