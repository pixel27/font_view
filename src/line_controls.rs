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
use iced::widget::{button, column, horizontal_space, row, scrollable, Column};
use iced::widget::button::secondary;

use super::{Lines, Message, Popin};

//*****************************************************************************
pub struct LineControls {
}

impl LineControls {
    //*************************************************************************
    pub fn new() -> Self {
        Self { }
    }

    //*************************************************************************
    pub fn view<'a>(
                &self,
                lines: &'a Lines
            ) -> Element<'a, Message> {
        column![
            row![
                button("Add line").on_press(Message::Show(Popin::AddLine)),
                button("Add quadratic").on_press(Message::Show(Popin::AddQuadratic)),
                button("Add cubic").on_press(Message::Show(Popin::AddCubic)),
                horizontal_space(),
                button("Test").style(secondary).on_press(Message::CreateTests)
            ].spacing(5),
            scrollable(
                Column::with_children(
                    lines.iter().enumerate().map(|(i, x)|x.view(i))
                )
            ).spacing(4)
        ].padding(4)
         .spacing(4)
         .into()
    }
}