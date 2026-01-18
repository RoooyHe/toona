use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub BoardMembersModal = <RoundedView> {
        width: 400,
        height: Fit,
        max_height: 600,
        show_bg: true,
        draw_bg: {
            color: #FFFFFF
            border_radius: 8
        },
        flow: Down,

        modal_header = {
            flow: Right,
            width: Fill,
            height: 56,
            align: {x: 0.5, y: 0.5},
            border_bottom: 1.0,
            border_color: #DFE1E6,
            padding: 16,

            header_title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 18}
                    color: #172B4D
                },
                text: "Members"
            }

            close_button = <Button> {
                width: 32, height: 32,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 20}
                    color: #5E6C84
                },
                text: "x"
            }
        }

        invite_section = {
            flow: Down,
            width: Fill,
            height: Fit,
            padding: 16,
            border_bottom: 1.0,
            border_color: #DFE1E6,

            invite_input_row = {
                flow: Right,
                width: Fill, height: Fit,
                spacing: 8,

                search_input = <TextInput> {
                    width: Fill, height: 36,
                    show_bg: true,
                    draw_bg: {
                        color: #FAFBFC
                        border_radius: 3
                        border_width: 1
                        border_color: #DFE1E6
                    },
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                        color: #172B4D
                    },
                    placeholder: "Invite by username..."
                }

                invite_button = <Button> {
                    width: Fit, height: 36,
                    padding: 12,
                    show_bg: true,
                    draw_bg: {
                        color: #0079BF
                        border_radius: 3
                    },
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                        color: #FFFFFF
                    },
                    text: "Invite"
                }
            }

            invite_hint = <Label> {
                width: Fill, height: Fit,
                margin: {top: 8},
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #5E6C84
                },
                text: "Enter Matrix username (@user:server) to invite members"
            }
        }

        members_section = {
            flow: Down,
            width: Fill,
            height: Fill,
            padding: 16,

            section_title = <Label> {
                width: Fill, height: Fit,
                margin: {bottom: 8},
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #5E6C84
                    text_transform: Uppercase
                },
                text: "MEMBERS"
            }

            members_list = {
                flow: Down,
                width: Fill,
                height: Fill,
                spacing: 4,
            }
        }
    }

    pub BoardMemberItem = <RoundedView> {
        flow: Right,
        width: Fill,
        height: 48,
        align: {x: 0.0, y: 0.5},
        padding: 8,
        spacing: 12,
        cursor: Pointer,
        show_bg: true,
        draw_bg: {
            color: #FFFFFF
            border_radius: 3
        },

        member_avatar = <Avatar> {
            width: 32, height: 32,
        }

        member_info = {
            flow: Down,
            width: Fill, height: Fit,

            member_name = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    color: #172B4D
                },
                text: "Member Name"
            }

            member_id = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #5E6C84
                },
                text: "@user:matrix.org"
            }
        }

        role_badge = <View> {
            width: Fit, height: 20,
            padding: 4,
            show_bg: true,
            draw_bg: {
                color: #EBECF0
                border_radius: 3
            },
            visible: false,

            role_label = <Label> {
                width: Fit, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 11}
                    color: #5E6C84
                },
                text: "Admin"
            }
        }

        member_actions = {
            flow: Right,
            width: Fit, height: Fit,
            spacing: 4,

            more_button = <Button> {
                width: 24, height: 24,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    color: #5E6C84
                },
                text: "..."
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardMembersModal {
    #[deref]
    view: View,
}

impl Widget for BoardMembersModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardMemberItem {
    #[deref]
    view: View,
}

impl Widget for BoardMemberItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
