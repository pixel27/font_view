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
*/use iced::{
    widget::{
        button, column, container, horizontal_space, row,
        text, text_input, vertical_space
    },
    Element
};

use super::{Glyphs, Lines, Message};

//*****************************************************************************
pub  struct FontControls {
    filename:   String,
    filter:     String,
    glyphs:     Option<Glyphs>
}

impl FontControls {
    //**************************************************************************
    pub fn new() -> Self {
        Self {
            filename: String::from("fonts/NotoSansMono-Regular.ttf"),
            filter:   String::new(),
            glyphs:   None
        }
    }

    //*************************************************************************
    pub fn view(&self) -> Element<Message> {
        let msg = if let Some(glyphs) = self.glyphs.as_ref() {
            glyphs.view()
        } else {
            container(
                column![
                    vertical_space(),
                    row![
                        horizontal_space(),
                        text("No file loaded."),
                        horizontal_space(),
                    ],
                    vertical_space()
                ]
            ).into()
        };

        column![
            row![
                text_input("", &self.filename)
                    .on_input(Message::FontFilename),
                button("Load")
                    .style(button::danger)
                    .on_press(Message::FontLoad(self.filename.clone()))
            ].spacing(8),
            msg,
            row![
                text("Filter:"),
                text_input("", &self.filter)
                    .on_input(Message::FontFilter)
            ].spacing(8),
        ].spacing(4)
         .padding(4)
         .width(320)
         .into()
    }

    //**************************************************************************
    pub fn handle_font_filename(
                &mut self,
                filename: String
            ) {
        self.filename = filename;
    }

    //**************************************************************************
    pub fn handle_font_filter(
                &mut self,
                filter: String
            ) {
        self.filter = filter;

        if let Some(glyphs) = self.glyphs.as_mut() {
            glyphs.filter(&self.filter);
        }
    }

    //**************************************************************************
    pub fn handle_font_load(
                &mut self,
                filename: String
            ) {
        self.filter = String::new();
        self.glyphs = None;

        match Glyphs::new(&filename) {
            Ok(glyphs) => self.glyphs = Some(glyphs),
            Err(err)   => eprintln!("{:?}", err)
        }
    }

    //**************************************************************************
    pub fn handle_font_view(
                &mut self,
                glyph: char,
                lines: &mut Lines
            ) {
        if let Some(glyphs) = self.glyphs.as_mut() {
            if let Err(_) = glyphs.view_glyph(glyph, lines) {
                eprintln!("Error decoding character: U+{:X}", glyph as u32);
            }
        }
    }
}

