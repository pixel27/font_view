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
use iced::{Color, Element};
use iced::alignment::Horizontal;
use iced::widget::{
    Column, Space, button, center, column, container,
    mouse_area, opaque, row, text, stack, text_editor
};
use iced::widget::text_editor::{Action, Content};

use regex::Regex;

use super::{Def, Message, Popin, PlotPoint};

//*****************************************************************************
struct Value {
    x_val:  f32,
    x_text: String,
    y_val:  f32,
    y_text: String
}

impl Value {
    //*************************************************************************
    pub fn new() -> Self {
        Self {
            x_val: f32::NAN,
            x_text: String::from("*error*"),
            y_val: f32::NAN,
            y_text: String::from("*error*")
        }
    }
}

//*****************************************************************************
pub struct Define {
    show:    Popin,
    p0:      Value,
    p1:      Value,
    p2:      Value,
    p3:      Value,
    formula: Content,
    error:   bool,
    edit:    Option<usize>
}

impl Default for Define {
    //*************************************************************************
    fn default() -> Self {
        Self {
            show:    Popin::None,
            p0:      Value::new(),
            p1:      Value::new(),
            p2:      Value::new(),
            p3:      Value::new(),
            formula: Content::default(),
            error:   false,
            edit:    None
        }
    }
}

impl Define {
    //*************************************************************************
    fn create(&self) -> Option<Message> {
        if self.error || self.show == Popin::None {
            return None
        }
        let p0 = PlotPoint { x: self.p0.x_val, y: self.p0.y_val };
        let p1 = PlotPoint { x: self.p1.x_val, y: self.p1.y_val };
        let p2 = PlotPoint { x: self.p2.x_val, y: self.p2.y_val };
        let p3 = PlotPoint { x: self.p3.x_val, y: self.p3.y_val };

        let def = match self.show {
            Popin::None         => panic!("We shouldn't get here."),
            Popin::AddLine      => Def::Line(p0, p1),
            Popin::AddQuadratic => Def::Quadratic(p0, p1, p2),
            Popin::AddCubic     => Def::Cubic(p0, p1, p2, p3),
        };

        if let Some(idx) = self.edit {
            Some(Message::LineChange(idx, def))
        } else {
            Some(Message::LineAdd(def))
        }
    }

    //*************************************************************************
    fn display(
                &self
            ) -> Column<Message> {
        let mut result = Vec::new();

        let row = |l, p:&Value| {
            row![
                text(l).align_x(Horizontal::Right).width(20),
                text(&p.x_val).align_x(Horizontal::Right).width(100),
                text(&p.y_val).align_x(Horizontal::Right).width(100)
            ].spacing(5)
             .into()
        };

        result.push(row("p0", &self.p0));
        result.push(row("p1", &self.p1));

        if self.show == Popin::AddQuadratic || self.show == Popin::AddCubic {
            result.push(row("p2", &self.p2));
        }
        if self.show == Popin::AddCubic {
            result.push(row("p3", &self.p3));
        }

        Column::from_vec(result)
    }

    //*************************************************************************
    pub fn view<'a>(
                &'a self,
                screen: impl Into<Element<'a, Message>>
            ) -> impl Into<Element<'a, Message>> {
        let name   = match self.show {
            Popin::None         => "none",
            Popin::AddLine      => "line",
            Popin::AddQuadratic => "quadratic",
            Popin::AddCubic     => "cubic"
        };
        let addedit = if self.edit.is_none() {
            "Add"
        } else {
            "Edit"
        };

        let popin = container(
            column![
                text(format!("Add {}", name)).size(24),
                Space::with_height(10),
                self.display(),
                Space::with_height(10),
                text_editor(&self.formula)
                    .on_action(Message::ChangeFormula),
                Space::with_height(10),
                row![
                    button(addedit).on_press_maybe(self.create()),
                    button("Cancel").style(button::danger).on_press(Message::Show(Popin::None))
                ].spacing(10)
            ]
        ).width(300)
         .padding(10)
         .style(container::rounded_box);

         stack![
            screen.into(),
            opaque(
                mouse_area(center(opaque(popin)).style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                }))
                .on_press(Message::Show(Popin::None))
            )
        ]
    }

    //*************************************************************************
    pub fn is_visible(&self) -> bool {
        self.show != Popin::None
    }

    //*************************************************************************
    pub fn handle_show(
                &mut self,
                popin: Popin
            ) {
        self.show = popin;
        self.edit = None;
        self.p0.x_text = String::new();
        self.p0.y_text = String::new();
        self.p1.x_text = String::new();
        self.p1.y_text = String::new();
        self.p2.x_text = String::new();
        self.p2.y_text = String::new();
        self.p3.x_text = String::new();
        self.p3.y_text = String::new();
        self.error = true;
        self.formula = Content::new();
    }

    //*************************************************************************
    pub fn handle_edit(
                &mut self,
                popin:   Popin,
                idx:     usize,
                formula: String
            ) {
        self.show = popin;
        self.formula = Content::with_text(&formula);
        self.edit    = Some(idx);
        self.parse_text(&formula);
        self.error = false;
    }

    //*************************************************************************
    fn parse_text(
                &mut self,
                text: &str
            ) -> usize {
        let re = Regex::new(r"-?[0-9]+(\.[0-9]+)?").unwrap();

        let mut pos = 0;
        let output = [
            (&mut self.p0.x_text, &mut self.p0.x_val),
            (&mut self.p0.y_text, &mut self.p0.y_val),
            (&mut self.p1.x_text, &mut self.p1.x_val),
            (&mut self.p1.y_text, &mut self.p1.y_val),
            (&mut self.p2.x_text, &mut self.p2.x_val),
            (&mut self.p2.y_text, &mut self.p2.y_val),
            (&mut self.p3.x_text, &mut self.p3.x_val),
            (&mut self.p3.y_text, &mut self.p3.y_val),
        ];

        self.error = false;
        for cap in re.captures_iter(&text) {
            if let Some(val) = cap.get(0) {
                if pos < output.len() {
                    let value      = val.as_str();
                    *output[pos].0 = String::from(value);
                    *output[pos].1 = value.parse().unwrap();
                    pos += 1;
                } else {
                    self.error = true;
                }
            }
        }

        pos
    }

    //*************************************************************************
    pub fn handle_change_formula(
                &mut self,
                action: Action
            ) {
        self.formula.perform(action);

        let read = self.parse_text(
            &self.formula.text()
        );

        if self.show == Popin::AddLine && read != 4 {
            self.error = true;
        } else if self.show == Popin::AddQuadratic && read != 6 {
            self.error = true;
        } else if self.show == Popin::AddCubic && read != 8 {
            self.error = true;
        }
    }
}
