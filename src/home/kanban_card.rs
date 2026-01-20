use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanCard = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: 8
        spacing: 6
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
}

impl Widget for KanbanCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::MouseDown(_) = event {
            log!("KanbanCard clicked!");
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
