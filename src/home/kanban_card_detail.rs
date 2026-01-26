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
            color: #FFFFFF
        }

        padding: 16
        spacing: 16

        header_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            card_icon = <View> {
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
                text: "在列表 \"待办\" 中"
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #5E6C84
                }
            }
        }

        main_content = <View> {
            width: Fill, height: Fill
            flow: Right
            spacing: 16

            left_column = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 20

                labels_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    section_title = <Label> {
                        width: Fill, height: Fit
                        text: "标签"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 14}
                            color: #172B4D
                        }
                    }

                    labels_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 6

                        label_blue = <View> {
                            width: 60, height: 24
                            show_bg: true
                            draw_bg: { color: #0079BF }
                        }
                        label_green = <View> {
                            width: 60, height: 24
                            show_bg: true
                            draw_bg: { color: #61BD4F }
                        }
                        label_orange = <View> {
                            width: 60, height: 24
                            show_bg: true
                            draw_bg: { color: #FF9F1A }
                        }
                        label_red = <View> {
                            width: 60, height: 24
                            show_bg: true
                            draw_bg: { color: #EB5A46 }
                        }
                    }
                }

                description_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    desc_label = <Label> {
                        width: Fill, height: Fit
                        text: "描述"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 14}
                            color: #172B4D
                        }
                    }

                    desc_content = <View> {
                        width: Fill, height: Fit
                        padding: 10
                        show_bg: true
                        draw_bg: {
                            color: #F4F5F7
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

                checklist_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    checklist_title = <Label> {
                        width: Fill, height: Fit
                        text: "清单"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 14}
                            color: #172B4D
                        }
                    }

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

                    checklist_item_1 = <View> {
                        width: Fill, height: 28
                        flow: Right
                        spacing: 8

                        checkbox = <View> {
                            width: 16, height: 16
                            show_bg: true
                            draw_bg: { color: #EBECF0 }
                        }

                        item_text = <Label> {
                            width: Fill, height: Fit
                            text: "任务项 1"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                                color: #172B4D
                            }
                        }
                    }

                    checklist_item_2 = <View> {
                        width: Fill, height: 28
                        flow: Right
                        spacing: 8

                        checkbox_checked = <View> {
                            width: 16, height: 16
                            show_bg: true
                            draw_bg: { color: #61BD4F }
                        }

                        item_text_checked = <Label> {
                            width: Fill, height: Fit
                            text: "任务项 2"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                                color: #5E6C84
                            }
                        }
                    }

                    checklist_item_3 = <View> {
                        width: Fill, height: 28
                        flow: Right
                        spacing: 8

                        checkbox = <View> {
                            width: 16, height: 16
                            show_bg: true
                            draw_bg: { color: #EBECF0 }
                        }

                        item_text = <Label> {
                            width: Fill, height: Fit
                            text: "任务项 3"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                                color: #172B4D
                            }
                        }
                    }

                    checklist_item_4 = <View> {
                        width: Fill, height: 28
                        flow: Right
                        spacing: 8

                        checkbox_checked = <View> {
                            width: 16, height: 16
                            show_bg: true
                            draw_bg: { color: #61BD4F }
                        }

                        item_text_checked = <Label> {
                            width: Fill, height: Fit
                            text: "任务项 4"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                                color: #5E6C84
                            }
                        }
                    }
                }

                activity_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    activity_title = <Label> {
                        width: Fill, height: Fit
                        text: "活动"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 14}
                            color: #172B4D
                        }
                    }

                    activity_1 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8

                        user_avatar_1 = <View> {
                            width: 32, height: 32
                            show_bg: true
                            draw_bg: { color: #0079BF }
                        }

                        activity_content_1 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 2

                            activity_text_1 = <Label> {
                                width: Fill, height: Fit
                                text: "张三 添加了此卡片到 \"待办\""
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                                    color: #172B4D
                                }
                            }

                            activity_time_1 = <Label> {
                                width: Fill, height: Fit
                                text: "今天 上午 10:30"
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                                    color: #5E6C84
                                }
                            }
                        }
                    }

                    activity_2 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8

                        user_avatar_2 = <View> {
                            width: 32, height: 32
                            show_bg: true
                            draw_bg: { color: #61BD4F }
                        }

                        activity_content_2 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 2

                            activity_text_2 = <Label> {
                                width: Fill, height: Fit
                                text: "李四 将描述编辑为 \"这是一个卡片的描述...\""
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                                    color: #172B4D
                                }
                            }

                            activity_time_2 = <Label> {
                                width: Fill, height: Fit
                                text: "昨天 下午 3:45"
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                                    color: #5E6C84
                                }
                            }
                        }
                    }

                    comment_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 8

                        comment_title = <Label> {
                            width: Fill, height: Fit
                            text: "评论"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{font_size: 14}
                                color: #172B4D
                            }
                        }

                        comment_1 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 8

                            comment_avatar = <View> {
                                width: 32, height: 32
                                show_bg: true
                                draw_bg: { color: #FF9F1A }
                            }

                            comment_content = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 4

                                comment_header = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 8

                                    comment_user = <Label> {
                                        width: Fit, height: Fit
                                        text: "王五"
                                        draw_text: {
                                            text_style: <THEME_FONT_BOLD>{font_size: 13}
                                            color: #172B4D
                                        }
                                    }

                                    comment_time = <Label> {
                                        width: Fill, height: Fit
                                        text: "2 小时前"
                                        draw_text: {
                                            text_style: <THEME_FONT_REGULAR>{font_size: 12}
                                            color: #5E6C84
                                        }
                                    }
                                }

                                comment_text = <Label> {
                                    width: Fill, height: Fit
                                    text: "这个任务需要尽快完成，客户在等待反馈。"
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                                        color: #172B4D
                                    }
                                }
                            }
                        }
                    }
                }
            }

            right_column = <View> {
                width: 100, height: Fit
                flow: Down
                spacing: 12

                add_to_card_label = <Label> {
                    width: Fill, height: Fit
                    text: "添加到卡片"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                member_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    member_icon = <View> {
                        width: 20, height: 20
                        show_bg: true
                        draw_bg: { color: #5E6C84 }
                    }

                    member_label = <Label> {
                        width: Fill, height: Fit
                        text: "成员"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                label_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    label_icon = <View> {
                        width: 20, height: 20
                        show_bg: true
                        draw_bg: { color: #FF9F1A }
                    }

                    label_text = <Label> {
                        width: Fill, height: Fit
                        text: "标签"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                checklist_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    checklist_icon = <View> {
                        width: 20, height: 20
                        show_bg: true
                        draw_bg: { color: #61BD4F }
                    }

                    checklist_text = <Label> {
                        width: Fill, height: Fit
                        text: "清单"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                date_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    date_icon = <View> {
                        width: 20, height: 20
                        show_bg: true
                        draw_bg: { color: #EB5A46 }
                    }

                    date_text = <Label> {
                        width: Fill, height: Fit
                        text: "日期"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                attachment_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    attachment_text = <Label> {
                        width: Fill, height: Fit
                        text: "附件"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                cover_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    cover_text = <Label> {
                        width: Fill, height: Fit
                        text: "封面"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
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

                move_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    move_text = <Label> {
                        width: Fill, height: Fit
                        text: "移动"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                copy_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8, right: 8}

                    copy_text = <Label> {
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
                    padding: {left: 8, right: 8}

                    archive_text = <Label> {
                        width: Fill, height: Fit
                        text: "归档"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #EB5A5A
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
            .label(ids!(header_section.card_title))
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
