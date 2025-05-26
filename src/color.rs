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
use std::fmt::Display;

//*****************************************************************************
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Blue,
    Brown,
    Chartreuse,
    Crimson,
    Cyan,
    Gold,
    Gray,
    Green,
    Indigo,
    Lavender,
    Lime,
    Magenta,
    Maroon,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow
}

impl Color {
    //*************************************************************************
    pub const ALL: &'static [Self] = &[
        Self::Black,
        Self::Blue,
        Self::Brown,
        Self::Chartreuse,
        Self::Crimson,
        Self::Cyan,
        Self::Gold,
        Self::Gray,
        Self::Green,
        Self::Indigo,
        Self::Lavender,
        Self::Lime,
        Self::Magenta,
        Self::Maroon,
        Self::Orange,
        Self::Pink,
        Self::Purple,
        Self::Red,
        Self::Yellow
    ];

    //*************************************************************************
    pub fn value(&self) -> u32 {
        match self {
            Self::Black      => 0xFF000000,
            Self::Blue       => 0xFFFF0000,
            Self::Brown      => 0xFF2A2AA5,
            Self::Chartreuse => 0xFF00FF7F,
            Self::Crimson    => 0xFF3C14DC,
            Self::Cyan       => 0xFFFFFF00,
            Self::Gold       => 0xFF00D7FF,
            Self::Gray       => 0xFF808080,
            Self::Green      => 0xFF008000,
            Self::Indigo     => 0xFF82004B,
            Self::Lavender   => 0xFFFAE6E6,
            Self::Lime       => 0xFF00FF00,
            Self::Magenta    => 0xFFFF00FF,
            Self::Maroon     => 0xFF000080,
            Self::Orange     => 0xFF00A5FF,
            Self::Pink       => 0xFFFCB0FC,
            Self::Purple     => 0xFF890080,
            Self::Red        => 0xFF0000FF,
            Self::Yellow     => 0xFF00FFFF,
        }
    }
}

//*****************************************************************************
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Black      => "Black",
            Self::Blue       => "Blue",
            Self::Brown      => "Brown",
            Self::Chartreuse => "Chartreuse",
            Self::Crimson    => "Crimson",
            Self::Cyan       => "Cyan",
            Self::Gold       => "Gold",
            Self::Gray       => "Gray",
            Self::Green      => "Green",
            Self::Indigo     => "Indigo",
            Self::Lavender   => "Lavender",
            Self::Lime       => "Lime",
            Self::Magenta    => "Magenta",
            Self::Maroon     => "Maroon",
            Self::Orange     => "Orange",
            Self::Pink       => "Pink",
            Self::Purple     => "Purple",
            Self::Red        => "Red",
            Self::Yellow     => "Yellow",
        })
    }
}