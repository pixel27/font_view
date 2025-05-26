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
use iced::Element;
use iced::widget::{
    Space, button, checkbox, column, container, horizontal_space, row,
    text
};

use super::{Color, Message, PlotPoint, Popin};

//*****************************************************************************
#[derive(Debug, Clone)]
pub enum Def {
    Line(Color, PlotPoint, PlotPoint),
    Quadratic(Color, PlotPoint, PlotPoint, PlotPoint),
    Cubic(Color, PlotPoint, PlotPoint, PlotPoint, PlotPoint)
}

//*****************************************************************************
pub struct Line {
    draw: bool,
    def:  Def,
    text: String
}

impl Line {
    //*************************************************************************
    pub fn new(
                def: Def
            ) -> Self {
        let text = match def {
            Def::Line(_, p0, p1)           => format!("({} {}) ({} {})", p0.x, p0.y, p1.x, p1.y),
            Def::Quadratic(_, p0, p1,  p2) => format!("({} {}) ({} {}) ({} {})", p0.x, p0.y, p1.x, p1.y, p2.x, p2.y),
            Def::Cubic(_, p0, p1,  p2, p3) => format!("({} {}) ({} {}) ({} {}) ({} {})", p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y),
        };

        Self {
            draw: true,
            def, text
        }
    }

    //*************************************************************************
    pub fn view(
                &self,
                idx: usize
            ) -> Element<Message> {
        let color;
        let data = match &self.def {
            Def::Line(c, p0, p1) => {
                color = *c;
                row![
                    text(format!("Line({:?}) - ", c)),
                    text(format!("p0: ({}, {})", p0.x, p0.y)),
                    text(format!("p1: ({}, {})", p1.x, p1.y)),
                ].spacing(5)
            },
            Def::Quadratic(c, p0, p1, p2) => {
                color = *c;
                row![
                    text(format!("Quadratic({:?}) - ", c)),
                    text(format!("p0: ({}, {})", p0.x, p0.y)),
                    text(format!("p1: ({}, {})", p1.x, p1.y)),
                    text(format!("p2: ({}, {})", p2.x, p2.y)),
                ].spacing(5)
            },
            Def::Cubic(c, p0, p1, p2, p3) => {
                color = *c;
                row![
                    text(format!("Cubic({:?}) - ", c)),
                    text(format!("p0: ({}, {})", p0.x, p0.y)),
                    text(format!("p1: ({}, {})", p1.x, p1.y)),
                    text(format!("p2: ({}, {})", p2.x, p2.y)),
                    text(format!("p3: ({}, {})", p3.x, p3.y)),
                ].spacing(5)
            }
        };
        let popin = match self.def {
            Def::Line(_, _, _) => Popin::AddLine,
            Def::Quadratic(_, _, _, _) => Popin::AddQuadratic,
            Def::Cubic(_, _, _, _, _) => Popin::AddCubic
        };

        column![
            Space::new(0, 8),
            container(
                row![
                    checkbox("", self.draw).on_toggle(move |x|Message::LineToggle(idx, x)),
                    data.padding([4, 0]),
                    horizontal_space(),
                    button("Edit")
                        .style(button::success)
                        .on_press(Message::LineShow(popin, idx, color, self.text.clone())),
                    button("Remove").style(button::danger).on_press(Message::LineRemove(idx))
                ].padding(4)
                 .spacing(8)
            ).style(container::bordered_box)
        ].into()
    }

    //*************************************************************************
    pub fn enable(
                &mut self,
                enable: bool
            ) {
        self.draw = enable;
    }

    //*************************************************************************
    pub fn change(
                &mut self,
                def: Def
            ) {
        self.def  = def;
        self.draw = true;
    }

    //*************************************************************************
    pub fn is_enabled(&self) -> bool {
        self.draw
    }

    //*************************************************************************
    pub fn def(&self) -> &Def {
        &self.def
    }

}
