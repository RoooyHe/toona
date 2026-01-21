use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanCard = {{KanbanCard}} {
        width: Fill, height: Fit
        flow: Down
        padding: 8
        spacing: 6
        show_bg: true
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
    view: View,
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
        match event {
            Event::MouseDown(_) => {
                let card_id = self.card_id.clone();
                if let Some(card_id) = card_id {
                    log!("KanbanCard clicked: {}", card_id);
                    self.is_selected = !self.is_selected;
                    self.update_visual_state(cx);
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        KanbanCardAction::Clicked { card_id },
                    );
                }
            }
            _ => {}
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl KanbanCard {
    fn update_visual_state(&mut self, cx: &mut Cx) {
        let bg_color = if self.is_selected {
            vec4(0.9, 0.95, 1.0, 1.0)
        } else {
            vec4(1.0, 1.0, 1.0, 1.0)
        };
        self.view
            .apply_over(cx, live! { draw_bg: { color: (bg_color) } });
    }

    pub fn set_card(&mut self, cx: &mut Cx, card_id: &str, title: &str, is_selected: bool) {
        self.card_id = Some(card_id.to_string());
        self.is_selected = is_selected;
        self.view.label(ids!(card_title)).set_text(cx, title);
        self.update_visual_state(cx);
    }
}
