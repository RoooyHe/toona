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
        cursor: Hand
        show_bg: true
        draw_bg: {
            color: #FFFFFF
        }

        labels_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 4
            visible: false

            label_blue = <View> {
                width: 40, height: 8
                show_bg: true
                draw_bg: { color: #0079BF }
            }
            label_green = <View> {
                width: 40, height: 8
                show_bg: true
                draw_bg: { color: #61BD4F }
            }
            label_orange = <View> {
                width: 40, height: 8
                show_bg: true
                draw_bg: { color: #FF9F1A }
            }
        }

        card_title = <Label> {
            width: Fill, height: Fit
            text: "卡片标题"
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                color: #172B4D
            }
        }

        badges_row = <View> {
            width: Fill, height: 20
            flow: Right
            spacing: 8
            visible: false

            description_icon = <View> {
                width: 16, height: 16
                show_bg: true
                draw_bg: { color: #EBECF0 }
            }

            attachment_icon = <View> {
                width: 16, height: 16
                show_bg: true
                draw_bg: { color: #EBECF0 }
            }
        }

        footer = <View> {
            width: Fill, height: 20
            flow: Right
            spacing: 4
            visible: false

            member_avatar_1 = <View> {
                width: 20, height: 20
                show_bg: true
                draw_bg: { color: #Dfe1e6 }
            }
            member_avatar_2 = <View> {
                width: 20, height: 20
                show_bg: true
                draw_bg: { color: #Dfe1e6 }
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
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) if fe.is_over && fe.is_primary_hit() && fe.was_tap() => {
                if let Some(card_id) = self.card_id.clone() {
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
    fn set_card(&mut self, cx: &mut Cx, card_id: &str, title: &str, is_selected: bool) {
        self.card_id = Some(card_id.to_string());
        self.view.label(ids!(card_title)).set_text(cx, title);
        self.set_selected(cx, is_selected);
    }

    fn set_selected(&mut self, cx: &mut Cx, is_selected: bool) {
        self.is_selected = is_selected;
        let bg_color = if is_selected {
            vec4(0.9, 0.95, 1.0, 1.0)
        } else {
            vec4(1.0, 1.0, 1.0, 1.0)
        };
        let text_color = if is_selected {
            vec4(0.06, 0.33, 0.7, 1.0)
        } else {
            vec4(0.09, 0.17, 0.3, 1.0)
        };
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { color: (bg_color) }
                card_title = { draw_text: { color: (text_color) } }
            },
        );
    }
}

impl KanbanCardRef {
    pub fn set_card(&self, cx: &mut Cx, card_id: &str, title: &str, is_selected: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_card(cx, card_id, title, is_selected);
    }

    pub fn set_selected(&self, cx: &mut Cx, is_selected: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_selected(cx, is_selected);
    }
}
