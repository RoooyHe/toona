use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanCard = <Button> {
        width: Fill, height: 60
        flow: Down
        padding: 8
        spacing: 6
        draw_bg: {
            color: #FFFFFF
        }

        card_title = <Label> {
            width: Fill, height: Fit
            text: "卡片标题"
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                color: #172B4D
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanCard {
    #[deref]
    button: Button,
    #[rust]
    card_id: Option<String>,
    #[rust]
    is_selected: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum KanbanCardAction {
    Clicked { card_id: String },
    None,
}

impl Widget for KanbanCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.button.draw_walk(cx, scope, walk)
    }
}

impl KanbanCard {
    fn update_visual_state(&mut self, cx: &mut Cx) {
        let bg_color = if self.is_selected {
            vec4(0.9, 0.95, 1.0, 1.0)
        } else {
            vec4(1.0, 1.0, 1.0, 1.0)
        };
        self.button
            .apply_over(cx, live! { draw_bg: { color: (bg_color) } });
    }

    pub fn set_card(&mut self, cx: &mut Cx, card_id: &str, title: &str, is_selected: bool) {
        self.card_id = Some(card_id.to_string());
        self.is_selected = is_selected;
        self.button.label(ids!(card_title)).set_text(cx, title);
        self.update_visual_state(cx);
    }
}

impl KanbanCardRef {
    pub fn set_card(&self, cx: &mut Cx, card_id: &str, title: &str, is_selected: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_card(cx, card_id, title, is_selected);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(btn) = self.borrow() {
            btn.clicked(actions)
        } else {
            false
        }
    }
}
