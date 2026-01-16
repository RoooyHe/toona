use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanCard = <RoundedView> {
        width: Fill, height: Fit
        min_height: 60
        flow: Down
        padding: 8
        spacing: 6
        show_bg: true
        draw_bg: {
            color: #FFFFFF
            border_radius: 4
        }
        cursor: Pointer

        card_title = <Label> {
            id: card_title
            width: Fill, height: Fit
            text: "示例卡片"
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                color: #172B4D
                wrap: Word
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanCard {
    #[deref]
    view: View,
}

impl Widget for KanbanCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl KanbanCardRef {
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(card) = self.borrow_mut() {
            card.view.label(ids!(card_title)).set_text(cx, title);
        }
    }
}
