use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub BoardBackgroundModal = <RoundedView> {
        width: 560,
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
            border_bottom: 1, #DFE1E6,
            padding: 16,

            header_title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 18}
                    color: #172B4D
                },
                text: "Change Background"
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

        background_content = {
            flow: Down,
            width: Fill,
            height: Fit,
            padding: 16,

            colors_section = {
                flow: Down,
                width: Fill,
                height: Fit,
                margin: {bottom: 16},

                section_title = <Label> {
                    width: Fill, height: Fit,
                    margin: {bottom: 8},
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 14}
                        color: #5E6C84
                    },
                    text: "COLORS"
                }

                colors_grid = {
                    flow: Right,
                    width: Fill,
                    height: Fit,
                    spacing: 8,

                    color_option = <BgColorOption> { color: #0079BF }
                    color_option2 = <BgColorOption> { color: #EB5A46 }
                    color_option3 = <BgColorOption> { color: #D99E0B }
                    color_option4 = <BgColorOption> { color: #61BD4F }
                    color_option5 = <BgColorOption> { color: #C377E0 }
                    color_option6 = <BgColorOption> { color: #F2D600 }
                    color_option7 = <BgColorOption> { color: #FF9F1A }
                    color_option8 = <BgColorOption> { color: #EFECE5 }
                }
            }

            photos_section = {
                flow: Down,
                width: Fill,
                height: Fit,

                section_title = <Label> {
                    width: Fill, height: Fit,
                    margin: {bottom: 8},
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 14}
                        color: #5E6C84
                    },
                    text: "PHOTOS"
                }

                photos_grid = {
                    flow: Right,
                    width: Fill,
                    height: Fit,
                    spacing: 8,

                    photo_option = <BgPhotoOption> { color: #EBECF0 }
                }
            }
        }
    }

    pub BgColorOption = <View> {
        width: 48, height: 48,
        cursor: Pointer,
        show_bg: true,
        draw_bg: {
            color: #0079BF
            border_radius: 4
        }
    }

    pub BgPhotoOption = <View> {
        width: 96, height: 48,
        cursor: Pointer,
        show_bg: true,
        draw_bg: {
            color: #EBECF0
            border_radius: 4
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardBackgroundModal {
    #[deref]
    view: View,
}

impl Widget for BoardBackgroundModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BgColorOption {
    #[deref]
    view: View,
}

impl Widget for BgColorOption {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BgPhotoOption {
    #[deref]
    view: View,
}

impl Widget for BgPhotoOption {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
