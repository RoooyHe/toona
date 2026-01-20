use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::home::kanban_card::KanbanCard;

    pub KanbanListView = <View> {
        width: 280, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: #EBECF0
        }

        list_header = <View> {
            width: Fill, height: 40
            padding: 12
            show_bg: true
            draw_bg: {
                color: #EBECF0
            }

            list_title = <Label> {
                width: Fill, height: Fit
                text: "待办"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #172B4D
                }
            }
        }

        cards_container = <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: 8
            padding: 8

            card_1 = <KanbanCard> {}
            card_2 = <KanbanCard> {}
            card_3 = <KanbanCard> {}
            card_4 = <KanbanCard> {}
            card_5 = <KanbanCard> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanListView {
    #[deref]
    view: View,
}

impl Widget for KanbanListView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
