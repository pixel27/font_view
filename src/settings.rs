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

use iced::{Element, Theme};
use iced::widget::{horizontal_space, pick_list, row, slider, text};

use super::Message;

//*****************************************************************************
pub struct Settings {
    thickness: i32,
}

impl Settings {
    //*************************************************************************
    pub fn new(
                thickness: i32
            ) -> Self {
        Self {
            thickness
        }
    }

    //*************************************************************************
    pub fn view(
                &self,
                theme: &Theme
            ) -> Element<Message> {
        row![
            text("Thickness:"),
            slider(1..=31, self.thickness, Message::ChangeThickness)
                .default(5)
                .step(2),
            text(format!("{}", self.thickness)),
            horizontal_space(),
            pick_list(Theme::ALL, Some(theme.clone()), Message::ChangeTheme)
        ].padding(4)
         .spacing(8)
         .into()
    }

    //*************************************************************************
    pub fn handle_thickness(
                &mut self,
                thickness: i32
            ) {
        self.thickness = thickness;
    }
}