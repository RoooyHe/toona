use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub BoardsSidebar = <RoundedView> {
        flow: Down,
        width: 272,
        height: Fill,
        draw_bg: { color: #FFFFFF },

        sidebar_header = <View> {
            flow: Right,
            width: Fill,
            height: 48,
            align: {x: 0.0, y: 0.5},
            padding: 12,
            draw_bg: { color: #F8F9FA }

            header_title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #172B4D
                },
                text: "Boards"
            }

            add_board_button = <Button> {
                width: 28, height: 28,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 16}
                    color: #5E6C84
                },
                text: "+"
            }
        }

        boards_list = <View> {
            flow: Down,
            width: Fill,
            height: Fill,
            padding: 8,
            spacing: 4,
        }

        sidebar_footer = <View> {
            flow: Down,
            width: Fill,
            height: Fit,
            padding: 12,
            draw_bg: { color: #F8F9FA }

            settings_button = <Button> {
                width: Fill, height: 32,
                padding: 8,
                draw_bg: { color: #FFFFFF },
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #5E6C84
                },
                text: "Settings"
            }

            archived_button = <Button> {
                width: Fill, height: 32,
                padding: 8,
                draw_bg: { color: #FFFFFF },
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #5E6C84
                },
                text: "Archived"
            }
        }
    }

    pub BoardListItem = <RoundedView> {
        flow: Right,
        width: Fill,
        height: 32,
        align: {x: 0.0, y: 0.5},
        padding: 8,
        spacing: 8,
        cursor: Pointer,
        draw_bg: { color: #FFFFFF },

        board_icon = <View> {
            width: 20, height: 20,
            draw_bg: { color: #0079BF }
        }

        board_name = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                color: #172B4D
            },
            text: "Board Name"
        }

        selected_indicator = <View> {
            width: 4, height: 20,
            draw_bg: { color: #0079BF },
            visible: false,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardsSidebar {
    #[deref]
    view: View,
}

impl Widget for BoardsSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardListItem {
    #[deref]
    view: View,
}

impl Widget for BoardListItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
