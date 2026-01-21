use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanCardDetail = {{KanbanCardDetail}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: #F4F5F7
        }

        content = <View> {
            width: Fill, height: Fill
            flow: Right
            spacing: 16
            padding: 16

            main_column = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16

                title_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    title_icon = <View> {
                        width: 24, height: 24
                        show_bg: true
                        draw_bg: { color: #EBECF0 }
                    }

                    card_title = <Label> {
                        width: Fill, height: Fit
                        text: "卡片标题"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 20}
                            color: #172B4D
                        }
                    }

                    in_list_label = <Label> {
                        width: Fill, height: Fit
                        text: "在列表 中"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #5E6C84
                        }
                    }
                }

                description_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    section_header = <View> {
                        width: Fill, height: 32
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        desc_icon = <View> {
                            width: 24, height: 24
                            show_bg: true
                            draw_bg: { color: #EBECF0 }
                        }

                        desc_label = <Label> {
                            width: Fit, height: Fit
                            text: "描述"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{font_size: 16}
                                color: #172B4D
                            }
                        }
                    }

                    desc_content = <View> {
                        width: Fill, height: Fit
                        padding: 8
                        show_bg: true
                        draw_bg: {
                            color: #FFFFFF
                        }

                        desc_text = <Label> {
                            width: Fill, height: Fit
                            text: "这是一个卡片的描述内容。可以添加详细信息、链接、列表等。"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                                color: #172B4D
                            }
                        }
                    }
                }

                activity_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    activity_header = <View> {
                        width: Fill, height: 32
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        activity_icon = <View> {
                            width: 24, height: 24
                            show_bg: true
                            draw_bg: { color: #EBECF0 }
                        }

                        activity_label = <Label> {
                            width: Fit, height: Fit
                            text: "活动"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{font_size: 16}
                                color: #172B4D
                            }
                        }
                    }

                    activity_item_1 = <View> {
                        width: Fill, height: 40
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        avatar_1 = <View> {
                            width: 32, height: 32
                            show_bg: true
                            draw_bg: { color: #0079BF, border_radius: 16 }
                        }

                        activity_text_1 = <Label> {
                            width: Fill, height: Fit
                            text: "Roy 添加了此卡片到 待办"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                                color: #172B4D
                            }
                        }
                    }
                }
            }

            sidebar = <View> {
                width: 180, height: Fill
                flow: Down
                spacing: 8

                add_to_card_label = <Label> {
                    width: Fill, height: Fit
                    text: "添加到卡片"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                members_label = <Label> {
                    width: Fill, height: Fit
                    text: "成员"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                labels_label = <Label> {
                    width: Fill, height: Fit
                    text: "标签"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                labels_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 4

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

                dates_label = <Label> {
                    width: Fill, height: Fit
                    text: "日期"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                date_value = <Label> {
                    width: Fill, height: Fit
                    text: "2026年1月25日"
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                        color: #172B4D
                    }
                }

                checklist_label = <Label> {
                    width: Fill, height: Fit
                    text: "清单"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                checklist_progress = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 4

                    progress_bar = <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: { color: #EBECF0 }

                        progress_fill = <View> {
                            width: 50, height: 8
                            show_bg: true
                            draw_bg: { color: #00C2E0 }
                        }
                    }

                    checklist_text = <Label> {
                        width: Fill, height: Fit
                        text: "2/4 已完成"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 13}
                            color: #5E6C84
                        }
                    }
                }

                actions_label = <Label> {
                    width: Fill, height: Fit
                    text: "操作"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                move_card_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8}

                    move_label = <Label> {
                        width: Fill, height: Fit
                        text: "移动"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                copy_card_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8}

                    copy_label = <Label> {
                        width: Fill, height: Fit
                        text: "复制"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                archive_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8}

                    archive_label = <Label> {
                        width: Fill, height: Fit
                        text: "归档"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanCardDetail {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<String>,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum KanbanCardDetailAction {
    Close,
    None,
}

impl Widget for KanbanCardDetail {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl KanbanCardDetail {
    pub fn set_card(&mut self, cx: &mut Cx, card_id: &str, title: &str) {
        self.card_id = Some(card_id.to_string());
        self.view
            .label(ids!(main_column.title_section.card_title))
            .set_text(cx, title);
    }
}

impl KanbanCardDetailRef {
    pub fn set_card(&self, cx: &mut Cx, card_id: &str, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_card(cx, card_id, title);
        }
    }
}
