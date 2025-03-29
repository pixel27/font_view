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
    column, horizontal_rule, horizontal_space, image,
    responsive, row, text, vertical_space
};
use iced::widget::image::Handle;

use super::{Graph, Lines, Message};

//*****************************************************************************
pub struct View {
    top:    f32,
    bottom: f32,
    left:   f32,
    right:  f32,
    graph:  Graph,
    width:  f32,
    height: f32,
    handle: Handle
}

impl View {
    //*************************************************************************
    pub fn new(
                thickness: i32
            ) -> Self {
        let mut graph = Graph::new(thickness);
        graph.draw(&Lines::new());

        Self {
            top:    0.0,
            bottom: 0.0,
            left:   0.0,
            right:  0.0,
            width:  10.0,
            height: 10.0,
            handle: graph.canvas().handle(),
            graph
        }
    }
    //*************************************************************************
    pub fn view(
                &self
            ) -> Element<Message> {
        let top_left     = format!("({}, {})", self.left, self.top);
        let top_right    = format!("({}, {})", self.right, self.top);
        let bottom_left  = format!("({}, {})", self.left, self.bottom);
        let bottom_right = format!("({}, {})", self.right, self.bottom);

        let width  = self.width;
        let height = self.height;

        column![
            horizontal_rule(4),
            row![
                text(top_left),
                horizontal_space(),
                text(top_right)
            ].padding(4),
            responsive(move |size| {
                if width < size.width && height < size.height {
                    column![
                        vertical_space(),
                        row![
                            horizontal_space(),
                            image(&self.handle),
                            horizontal_space(),
                        ],
                        vertical_space(),
                    ].into()
                } else if width - size.width > height - size.height {
                    column![
                        vertical_space(),
                        image(&self.handle),
                        vertical_space(),
                    ].into()
                } else if height - size.height > width - size.width {
                    row![
                        horizontal_space(),
                        image(&self.handle),
                        horizontal_space(),
                    ].into()
                } else {
                    image(&self.handle).into()
                }
            }),
            row![
                text(bottom_left),
                horizontal_space(),
                text(bottom_right)
            ].padding(4),
            horizontal_rule(4),
        ].into()
    }

    //*************************************************************************
    pub fn handle_thickness(
                &mut self,
                thickness: i32
            ) {
        self.graph.handle_thickness(thickness);
    }

    //*************************************************************************
    pub fn update(
                &mut self,
                lines: &Lines
            ) {
        self.graph.draw(lines);
        self.top    = self.graph.y_max();
        self.bottom = self.graph.y_min();
        self.left   = self.graph.x_min();
        self.right  = self.graph.x_max();

        let canvas  = self.graph.canvas();
        self.width  = canvas.width() as f32;
        self.height = canvas.height() as f32;
        self.handle = canvas.handle();
    }
}