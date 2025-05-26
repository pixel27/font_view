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
use std::collections::HashSet;

//*****************************************************************************
pub struct Stroke {
    half:      isize,
    full:      Vec<(isize, isize)>,
    north:     Vec<isize>,
    northeast: Vec<(isize, isize)>,
    east:      Vec<isize>,
    southeast: Vec<(isize, isize)>,
    south:     Vec<isize>,
    southwest: Vec<(isize, isize)>,
    west:      Vec<isize>,
    northwest: Vec<(isize, isize)>
}

impl Stroke {
    //*************************************************************************
    pub fn new(
                width:  usize
            ) -> Self {
        assert!(width & 0x1 == 0x1, "The stroke must be an odd number.");

        let mut result = Self {
            half:      width as isize / 2,
            full:      Vec::with_capacity(width * width),
            north:     Vec::with_capacity(width),
            northeast: Vec::new(),
            east:      Vec::with_capacity(width),
            southeast: Vec::new(),
            south:     Vec::with_capacity(width),
            southwest: Vec::new(),
            west:      Vec::with_capacity(width),
            northwest: Vec::new(),
        };
        result.north.resize(width, isize::MAX);
        result.south.resize(width, isize::MIN);
        result.east.resize(width, isize::MIN);
        result.west.resize(width, isize::MAX);

        let half = result.half;

        for y in -half..=half {
            for x in -half..=half {
                let dst = ((x*x + y*y) as f64).sqrt();

                if dst > half as f64 { continue }

                result.full.push((x, y));

                let x_off = (half + x) as usize;
                let y_off = (half + y) as usize;

                if result.north[x_off] == isize::MAX { result.north[x_off] = y; }
                if result.west[y_off]  == isize::MAX { result.west[y_off] = x; }

                result.south[x_off] = y;
                result.east[y_off]  = x;
            }
        }

        result.northeast = diff(&result.full, 1, -1);
        result.southeast = diff(&result.full, 1, 1);
        result.southwest = diff(&result.full, -1, 1);
        result.northwest = diff(&result.full, -1, -1);

        result
    }

    //*************************************************************************
    fn to_offsets(
                stride: usize,
                points: &[(isize, isize)]
            ) -> Vec<isize> {
        let mut result = Vec::with_capacity(points.len());

        for (x, y) in points.iter() {
            result.push(y * stride as isize + x);
        }

        result
    }

    //*************************************************************************
    pub fn full(
                &self,
                stride: usize
            ) -> Vec<isize> {
        Self::to_offsets(stride, &self.full)
    }

    //*************************************************************************
    pub fn north_south(
                &self,
                stride: usize,
                points: &[isize]
            ) -> Vec<isize> {
        let mut result  = Vec::with_capacity(points.len());

        for (i, &y) in points.iter().enumerate() {
            let x = i as isize - self.half;
            result.push(y * stride as isize + x);
        }

        result
    }

    //*************************************************************************
    pub fn east_west(
                &self,
                stride: usize,
                points: &[isize]
            ) -> Vec<isize> {
        let mut result  = Vec::with_capacity(points.len());

        for (i, &x) in points.iter().enumerate() {
            let y = i as isize - self.half;
            result.push(y * stride as isize + x);
        }

        result
    }

    //*************************************************************************
    pub fn north(
                &self,
                stride: usize
            ) -> Vec<isize> {
        self.north_south(stride, &self.north)
    }

    //*************************************************************************
    pub fn north_east(
                &self,
                stride: usize
            ) -> Vec<isize> {
        Self::to_offsets(stride, &self.northeast)
    }

    //*************************************************************************
    pub fn east(
                &self,
                stride: usize
            ) -> Vec<isize> {
        self.east_west(stride, &self.east)
    }

    //*************************************************************************
    pub fn south_east(
                &self,
                stride: usize
            ) -> Vec<isize> {
        Self::to_offsets(stride, &self.southeast)
    }

    //*************************************************************************
    pub fn south(
                &self,
                stride: usize
            ) -> Vec<isize> {
        self.north_south(stride, &self.south)
    }

    //*************************************************************************
    pub fn south_west(
                &self,
                stride: usize
            ) -> Vec<isize> {
        Self::to_offsets(stride, &self.southwest)
    }

    //*************************************************************************
    pub fn west(
                &self,
                stride: usize
            ) -> Vec<isize> {
        self.east_west(stride, &self.west)
    }

    //*************************************************************************
    pub fn north_west(
                &self,
                stride: usize
            ) -> Vec<isize> {
        Self::to_offsets(stride, &self.northwest)
    }

}

//*****************************************************************************
pub fn diff(
            data:     &[(isize, isize)],
            x_offset: isize,
            y_offset: isize,
        ) -> Vec<(isize, isize)> {
    let mut existing = HashSet::new();

    for &(x, y) in data.iter() {
        existing.insert((x, y));
    }

    let mut result = Vec::new();
    for &(x, y) in data.iter() {
        let shifted = (x + x_offset, y + y_offset);

        if existing.contains(&shifted) == false {
            result.push((x, y));
        }
    }

    result
}