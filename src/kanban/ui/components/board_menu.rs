use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub BoardMenu = <RoundedView> {
        width: 280,
        height: Fit,
        show_bg: true,
        draw_bg: {
            color: #FFFFFF
            border_radius: 4
            shadow_color: #00000033
            shadow_radius: 8
            shadow_oset: vec2(0, 4)
        },
        flow: Down,

        menu_header = {
            flow: Right,
            width: Fill,
            height: 40,
            align: {x: 0.5, y: 0.5},
            border_bottom: 1, #DFE1E6,
            padding: 12,

            header_title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #172B4D
                },
                text: "Board Menu"
            }

            close_button = <Button> {
                width: 24, height: 24,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 16}
                    color: #5E6C84
                },
                text: "x"
            }
        }

        menu_content = {
            flow: Down,
            width: Fill,
            height: Fit,
            padding: 8,
            spacing: 4,

            change_background = <MenuItem> { text: "Change Background" }
            change_settings = <MenuItem> { text: "Settings" }

            more_items = {
                flow: Down,
                width: Fill,
                height: Fit,
                padding: {top: 4, bottom: 4},

                divider = <MenuDivider> {}
                activity_item = <MenuItem> { text: "Activity" }
                archive_item = <MenuItem> { text: "Archive" }
                share_item = <MenuItem> { text: "Share" }
            }
        }
    }

    pub MenuItem = <RoundedView> {
        flow: Right,
        width: Fill,
        height: 32,
        align: {x: 0.0, y: 0.5},
        padding: 8,
        spacing: 8,
        cursor: Pointer,
        show_bg: true,
        draw_bg: { color: #FFFFFF, border_radius: 3 },

        item_label = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                color: #172B4D
            },
            text: "Menu Item"
        }
    }

    pub MenuDivider = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: { color: #DFE1E6 },
        margin: {top: 4, bottom: 4}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardMenu {
    #[deref]
    view: View,
}

impl Widget for BoardMenu {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MenuItem {
    #[deref]
    view: View,
}

impl Widget for MenuItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MenuDivider {
    #[deref]
    view: View,
}

impl Widget for MenuDivider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
