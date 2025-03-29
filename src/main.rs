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
use iced::{
    widget::{
        text_editor::Action,
        column, container, focus_next, row, vertical_rule
    },
    Element, Size, Task, Theme
};

use canvas::{Canvas, Dir, StrokeFactory};
use line_controls::LineControls;
use cubic::Cubic;
use define::Define;
use font_controls::FontControls;
use glyphs::Glyphs;
use graph::Graph;
use line::{Def, Line};
use lines::Lines;
use plot_point::PlotPoint;
use point::Point;
use quadratic::Quadratic;
use settings::Settings;
use view::View;

//*****************************************************************************
mod canvas;
mod line_controls;
mod cubic;
mod define;
mod font_controls;
mod glyphs;
mod graph;
mod line;
mod lines;
mod point;
mod plot_point;
mod quadratic;
mod settings;
mod view;

//*****************************************************************************
pub fn main() {
    let size   = Size {
        width:  1528.0,
        height: 800.0
    };

    let result = iced::application(FontView::title, FontView::update, FontView::view)
        .window_size(size)
        .theme(FontView::theme)
        .run();

    if let Err(err) = result {
        eprintln!("UI Error: {}", err);
    }
}

//*****************************************************************************
#[derive(Debug, PartialEq, Clone)]
enum Popin {
    None,
    AddLine,
    AddQuadratic,
    AddCubic
}

//*****************************************************************************
#[derive(Debug, Clone)]
enum Message {
    ChangeTheme(Theme),
    ChangeThickness(i32),
    ChangeFormula(Action),
    CreateTests,
    FontFilename(String),
    FontFilter(String),
    FontLoad(String),
    FontView(char),
    LineAdd(Def),
    LineChange(usize, Def),
    LineRemove(usize),
    LineShow(Popin, usize, String),
    LineToggle(usize, bool),
    Show(Popin)
}

//*****************************************************************************
struct FontView {
    define:     Define,
    settings:   Settings,
    lines:      Lines,
    view:       View,
    l_controls: LineControls,
    f_controls: FontControls,
    theme:      Theme,
}

impl Default for FontView {
    //*************************************************************************
    fn default() -> Self {
        Self {
            define:     Define::default(),
            settings:   Settings::new(5),
            lines:      Lines::new(),
            view:       View::new(5),
            l_controls: LineControls::new(),
            f_controls: FontControls::new(),
            theme:      Theme::Dark,
        }
    }
}

impl FontView {
    //*************************************************************************
    fn view(&self) -> Element<Message> {
        let screen = column![
            self.settings.view(&self.theme),
            self.view.view(),
            container(
                row![
                    self.l_controls.view(&self.lines),
                    vertical_rule(8),
                    self.f_controls.view(),
                ]
            ).height(240)
        ].spacing(5);

        if self.define.is_visible() == false {
            screen.into()
        } else {
            self.define.view(screen).into()
        }
    }

    //*************************************************************************
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeTheme(theme) => {
                self.handle_change_theme(theme);
                Task::none()
            },
            Message::ChangeThickness(val) => {
                self.settings.handle_thickness(val);
                self.view.handle_thickness(val);
                self.view.update(&self.lines);
                Task::none()
            },
            Message::ChangeFormula(action) => {
                self.define.handle_change_formula(action);
                Task::none()
            },
            Message::CreateTests => {
                self.handle_tests();
                Task::none()
            },
            Message::FontFilename(filename) => {
                self.f_controls.handle_font_filename(filename);
                Task::none()
            },
            Message::FontFilter(filter) => {
                self.f_controls.handle_font_filter(filter);
                Task::none()
            },
            Message::FontLoad(filename) => {
                self.f_controls.handle_font_load(filename);
                Task::none()
            },
            Message::FontView(glyph) => {
                self.f_controls.handle_font_view(glyph, &mut self.lines);
                self.view.update(&self.lines);
                Task::none()
            },
            Message::LineAdd(def) => {
                self.define.handle_show(Popin::None);
                self.lines.handle_line_add(def);
                self.view.update(&self.lines);
                Task::none()
            },
            Message::LineChange(idx, def) => {
                self.define.handle_show(Popin::None);
                self.lines.handle_line_change(idx, def);
                self.view.update(&self.lines);
                Task::none()
            },
            Message::LineShow(popin, idx, formula) => {
                self.define.handle_edit(popin, idx, formula);
                focus_next()
            },
            Message::LineToggle(idx, show) => {
                self.lines.handle_line_toggle(idx, show);
                self.view.update(&self.lines);
                Task::none()
            },
            Message::LineRemove(idx) => {
                self.lines.handle_line_remove(idx);
                self.view.update(&self.lines);
                Task::none()
            },
            Message::Show(popin) => {
                self.define.handle_show(popin);
                focus_next()
            },
        }
    }

    //*************************************************************************
    fn handle_change_theme(
                &mut self,
                theme: Theme
            ) {
        self.theme = theme;
    }

    //*************************************************************************
    fn handle_tests(&mut self) {
        self.lines.clear();

        // Horizontal
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: -500.0, y: -500.0}, PlotPoint{ x:500.0, y:-500.0})
        );
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 500.0, y:500.0}, PlotPoint{ x:-500.0, y:500.0})
        );
        // Vertical
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: -500.0, y: 500.0}, PlotPoint{ x: -500.0, y:-500.0})
        );
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 500.0, y: -500.0}, PlotPoint{ x: 500.0, y: 500.0})
        );

        // line_x_downward
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: -500.0, y: 500.0}, PlotPoint{ x: 0.0, y: 250.0})
        );
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 0.0, y: 0.0}, PlotPoint{ x: -500.0, y: 250.0})
        );
        // line_x_upward
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: -500.0, y: 250.0}, PlotPoint{ x: 0.0, y: 500.0})
        );
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 0.0, y: 250.0}, PlotPoint{ x: -500.0, y: 0.0})
        );
        // line_y_downward (right)
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 0.0, y: 500.0}, PlotPoint{ x: 250.0, y: 0.0})
        );
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 500.0, y: 0.0}, PlotPoint{ x: 250.0, y: 500.0})
        );
        // line_y_upward (left)
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 250.0, y: 0.0}, PlotPoint{ x: 500.0, y: 500.0})
        );
        self.lines.handle_line_add(
            Def::Line(PlotPoint{x: 250.0, y: 500.0}, PlotPoint{ x: 0.0, y: 0.0})
        );

        // Quadratic
        self.lines.handle_line_add(
            Def::Quadratic(PlotPoint{x: -500.0, y: 0.0}, PlotPoint { x: -250.0, y: -250.0}, PlotPoint { x: -500.0, y: -500.0})
        );

        self.lines.handle_line_add(
            Def::Quadratic(PlotPoint{x: 0.0, y: -500.0}, PlotPoint { x: -250.0, y: -250.0}, PlotPoint { x: 0.0, y: 0.0})
        );
        self.lines.handle_line_add(
            Def::Quadratic(PlotPoint{x: -300.0, y: 0.0}, PlotPoint { x: -250.0, y: -500.0}, PlotPoint { x: -200.0, y: 0.0})
        );
        self.lines.handle_line_add(
            Def::Quadratic(PlotPoint{x: -200.0, y: -500.0}, PlotPoint { x: -250.0, y: 0.0}, PlotPoint { x: -300.0, y: -500.0})
        );
        self.lines.handle_line_add(
            Def::Quadratic(PlotPoint{x: -500.0, y: -250.0}, PlotPoint { x: -250.0, y: -250.0}, PlotPoint { x: -250.0, y: 0.0})
        );
        self.lines.handle_line_add(
            Def::Quadratic(PlotPoint{x: -250.0, y: -500.0}, PlotPoint { x: -250.0, y: -250.0}, PlotPoint { x: 0.0, y: -250.0})
        );

        self.lines.handle_line_add(
            Def::Cubic(
                PlotPoint { x: 0.0, y: 0.0},
                PlotPoint { x: 1000.0, y: -650.0},
                PlotPoint { x: -500.0, y: -650.0},
                PlotPoint { x: 500.0, y: 0.0}
            )
        );
        self.lines.handle_line_add(
            Def::Cubic(
                PlotPoint { x: 0.0, y: -500.0},
                PlotPoint { x: 300.0, y: 0.0},
                PlotPoint { x: 200.0, y: 0.0},
                PlotPoint { x: 500.0, y: -500.0}
            )
        );


        self.view.update(&self.lines);
    }

    //*************************************************************************
    fn title(&self) -> String {
        String::from("Font View")
    }

    //*************************************************************************
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
