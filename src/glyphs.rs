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
use std::{
    cell::Cell,
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Error,
    rc::Rc
};
use iced::{
    widget::{
        Row,
        button, container, responsive, scrollable
    },
    Element
};
use font::{
    self,
    formats::opentype::characters::Character,
    glyph::Segment,
    Font
};

use crate::{Color, PlotPoint};
use super::{Lines, Message};

//*****************************************************************************
pub struct Glyphs {
    font:       Font<File>,
    selected:   Option<Rc<Glyph>>,
    by_chars:   HashMap<char, Rc<Glyph>>,
    by_unicode: BTreeMap<String, Rc<Glyph>>,
    filtered:   Vec<Rc<Glyph>>
}

impl Glyphs {
    //**************************************************************************
    pub fn new(
                filename: &str
            ) -> Result<Self, String> {
        let input                    = File::open(filename).map_err(|x|format!("{:?}", x))?;
        let font::File { mut fonts } = font::File::read(input).map_err(|x|format!("{:?}", x))?;

        if fonts.len() != 1 {
            return Err(format!("Found {} fonts in the file expected only 1.", fonts.len()));
        }

        let mut by_chars  = HashMap::new();
        let mut by_unicode = BTreeMap::new();
        let mut filtered   = Vec::new();

        if let Ok(chars) = fonts[0].characters() {
            for ch in chars.iter() {
                match ch {
                    Character::Scalar(x) => {
                        let g = Rc::new(Glyph::new(*x));
                        by_chars.insert(*x, g.clone());
                        by_unicode.insert(g.code(), g.clone());
                        filtered.push(g);
                    },
                    Character::Range((start, end)) => {
                        for x in *start..=*end {
                            let g = Rc::new(Glyph::new(x));
                            by_chars.insert(x, g.clone());
                            by_unicode.insert(g.code(), g.clone());
                            filtered.push(g);
                        }
                    },
                }
            }
        }

        Ok(
            Self {
                font:     fonts.pop().unwrap(),
                selected: None,
                by_chars, by_unicode, filtered
            }
        )
    }

    //*************************************************************************
    fn glyph_button(
                glyph: &Rc<Glyph>
            ) -> Element<Message> {
        let style = if glyph.is_selected() {
            button::success
        } else {
            button::secondary
        };
        button(
            glyph.label()
        ).on_press(Message::FontView(glyph.value))
         .style(style)
         .into()
    }

    //*************************************************************************
    pub fn view(&self) -> Element<Message> {
        container(
            responsive(|size|{
                scrollable(
                    Row::with_children(
                        self.filtered.iter().map(|x|Glyphs::glyph_button(x))
                    ).spacing(9)
                     .width(size.width)
                     .wrap()
                ).spacing(4)
                 .into()
            })
        ).height(160)
         .into()
    }

    //**************************************************************************
    fn filter_by_char(
                &mut self,
                filter: &str
            ) -> bool {
        self.filtered.clear();

        for ch in filter.chars() {
            if self.filtered.len() > 0 {
                return false;
            }
            if let Some(val) = self.by_chars.get(&ch) {
                self.filtered.push(val.clone());
            }
        }

        return true;
    }

    //**************************************************************************
    fn filter_by_unicode(
                &mut self,
                filter: &str
            ) {
        self.filtered.clear();

        let actual = filter.trim().to_uppercase();

        for (key, glyph) in self.by_unicode.iter() {
            if key.contains(&actual) {
                self.filtered.push(glyph.clone());
            }
        }
    }

    //**************************************************************************
    pub fn filter(
                &mut self,
                filter: &str
            ) {
        if filter.trim().len() == 0 {
            for glyph in self.by_unicode.values() {
                self.filtered.push(glyph.clone());
            }
        } else if self.filter_by_char(filter) == false {
            self.filter_by_unicode(filter);
        }
    }

    //**************************************************************************
    pub fn view_glyph(
                &mut self,
                glyph: char,
                lines: &mut Lines
            ) -> Result<(), Error> {
        if let Some(glyph) = self.selected.take() {
            glyph.clear_selected();
        }

        if let Some(glyph) = self.by_chars.get(&glyph) {
            glyph.set_selected();
            self.selected = Some(glyph.clone());
        }

        lines.clear();

        if let Some(def) = self.font.glyph(glyph)? {
            let mut p0 = PlotPoint::new(0.0, 0.0);
            for contour in def.iter() {
                p0 = p0 + contour.offset;

                for segment in contour.iter() {
                    match segment {
                        Segment::Linear(o1) => {
                            let p1 = p0 + *o1;
                            lines.add_line(Color::Black, p0, p1);
                            p0 = p1
                        },
                        Segment::Quadratic(o1, o2) => {
                            let p1 = p0 + *o1;
                            let p2 = p1 + *o2;
                            lines.add_quadratic(Color::Black, p0, p1, p2);
                            p0 = p2;
                        },
                        Segment::Cubic(o1, o2, o3) => {
                            let p1 = p0 + *o1;
                            let p2 = p1 + *o2;
                            let p3 = p2 + *o3;
                            lines.add_cubic(Color::Black, p0, p1, p2, p3);
                            p0 = p3;
                        }
                    };
                }
                let ( left , _, right, _ ) = def.bounding_box;

                if let Ok(metrics) = self.font.metrics() {
                    lines.add_line(Color::Gold, PlotPoint::new(left - def.side_bearings.0, metrics.baseline), PlotPoint::new(right + def.side_bearings.1, metrics.baseline));

                    lines.add_line(Color::Crimson, PlotPoint::new(0.0, metrics.ascender), PlotPoint::new(def.advance_width, metrics.ascender));
                    lines.add_line(Color::Crimson, PlotPoint::new(left - def.side_bearings.0, metrics.descender), PlotPoint::new(left - def.side_bearings.0, metrics.ascender));
                    lines.add_line(Color::Crimson, PlotPoint::new(right + def.side_bearings.1, metrics.descender), PlotPoint::new(right + def.side_bearings.1, metrics.ascender));
                    lines.add_line(Color::Crimson, PlotPoint::new(0.0, metrics.descender), PlotPoint::new(def.advance_width, metrics.descender));
                }

            }
        }

        Ok(())
    }
}

//*****************************************************************************
struct Glyph {
    value:    char,
    label:    String,
    selected: Cell<bool>
}

//*****************************************************************************
impl Glyph {
    //**************************************************************************
    pub fn new(
                value: char
            ) -> Self {
        let label = if value.is_control() {
            format!("U+{:04X}", u32::from(value))
        } else {
            format!("U+{:04X} ({})", u32::from(value), value)
        };
        let selected = Cell::new(false);

        Self { value, label, selected }
    }

    //**************************************************************************
    pub fn label(&self) -> &str {
        &self.label
    }

    //**************************************************************************
    pub fn code(&self) -> String {
        format!("{:04X}", u32::from(self.value))
    }

    //**************************************************************************
    pub fn is_selected(&self) -> bool {
        self.selected.get()
    }

    //**************************************************************************
    pub fn set_selected(&self) {
        self.selected.replace(true);
    }

    //**************************************************************************
    pub fn clear_selected(&self) {
        self.selected.replace(false);
    }
}
