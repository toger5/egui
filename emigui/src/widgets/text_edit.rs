use crate::*;

#[derive(Debug)]
pub struct TextEdit<'t> {
    text: &'t mut String,
    id: Option<Id>,
    text_style: TextStyle, // TODO: Option<TextStyle>, where None means "use the default for the current Ui"
    text_color: Option<Color>,
}

impl<'t> TextEdit<'t> {
    pub fn new(text: &'t mut String) -> Self {
        TextEdit {
            text,
            id: None,
            text_style: TextStyle::Body,
            text_color: Default::default(),
        }
    }

    pub fn id(mut self, id_source: impl std::hash::Hash) -> Self {
        self.id = Some(Id::new(id_source));
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }
}

impl<'t> Widget for TextEdit<'t> {
    fn ui(self, ui: &mut Ui) -> GuiResponse {
        let id = ui.make_child_id(self.id);

        let font = &ui.fonts()[self.text_style];
        let line_spacing = font.line_spacing();
        let galley = font.layout_multiline(self.text.as_str(), ui.available().width());
        let desired_size = galley.size.max(vec2(ui.available().width(), line_spacing));
        let interact = ui.reserve_space(desired_size, Some(id));

        if interact.clicked {
            ui.request_kb_focus(id);
        }
        if interact.hovered {
            ui.output().cursor_icon = CursorIcon::Text;
        }
        let has_kb_focus = ui.has_kb_focus(id);

        if has_kb_focus {
            for event in &ui.input().events {
                match event {
                    Event::Copy | Event::Cut => {
                        // TODO: cut
                        ui.ctx().output().copied_text = self.text.clone();
                    }
                    Event::Text(text) => {
                        if text == "\u{7f}" {
                            // backspace
                        } else {
                            *self.text += text;
                        }
                    }
                    Event::Key { key, pressed: true } => {
                        if *key == Key::Backspace {
                            self.text.pop(); // TODO: unicode aware
                        }
                    }
                    _ => {}
                }
            }
        }

        ui.add_paint_cmd(PaintCmd::Rect {
            rect: interact.rect,
            corner_radius: 0.0,
            // fill_color: Some(color::BLACK),
            fill_color: ui.style().interact(&interact).fill_color,
            // fill_color: Some(ui.style().background_fill_color()),
            outline: None, //Some(Outline::new(1.0, color::WHITE)),
        });

        if has_kb_focus {
            let cursor_blink_hz = ui.style().cursor_blink_hz;
            let show_cursor =
                (ui.input().time * cursor_blink_hz as f64 * 3.0).floor() as i64 % 3 != 0;
            if show_cursor {
                let cursor_pos = if let Some(last) = galley.lines.last() {
                    interact.rect.min + vec2(last.max_x(), last.y_offset)
                } else {
                    interact.rect.min
                };
                ui.add_paint_cmd(PaintCmd::line_segment(
                    [cursor_pos, cursor_pos + vec2(0.0, line_spacing)],
                    color::WHITE,
                    ui.style().text_cursor_width,
                ));
            }
        }

        ui.add_galley(interact.rect.min, galley, self.text_style, self.text_color);

        ui.response(interact)
    }
}
