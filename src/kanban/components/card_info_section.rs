use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 卡片基本信息区域（标题、描述、状态）
    pub CardInfoSection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 15,

        // 卡片标题
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            <Label> {
                width: Fill,
                height: Fit,
                text: "标题"
                draw_text: {
                    color: #666666
                    text_style: {
                        font_size: 14.0
                    }
                }
            }

            card_title = <Label> {
                width: Fill,
                height: Fit,
                text: "卡片标题"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 16.0
                    }
                }
            }
        }

        // 卡片描述
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {y: 0.5}

                <Label> {
                    width: Fill,
                    height: Fit,
                    text: "描述"
                    draw_text: {
                        color: #666666
                        text_style: {
                            font_size: 14.0
                        }
                    }
                }

                edit_description_button = <Button> {
                    width: 60,
                    height: 25,
                    text: "编辑"
                    draw_bg: {
                        color: #45B7D1
                    }
                    draw_text: {
                        color: #FFFFFF
                        text_style: {
                            font_size: 12.0
                        }
                    }
                }
            }

            // 描述显示区域
            card_description_label = <Label> {
                width: Fill,
                height: Fit,
                text: "暂无描述"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 14.0
                    }
                }
            }

            // 描述编辑区域
            description_edit_container = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,
                visible: false,

                card_description_input = <TextInput> {
                    width: Fill,
                    height: 80,
                    text: "",
                    draw_text: {
                        color: #333333
                        text_style: {
                            font_size: 14.0
                        }
                    }
                    draw_bg: {
                        color: #F8F9FA
                    }
                    draw_cursor: {
                        color: #333333
                    }
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10,

                    save_description_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "保存"
                        draw_bg: {
                            color: #45B7D1
                        }
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 12.0
                            }
                        }
                    }

                    cancel_description_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "取消"
                        draw_bg: {
                            color: #95A5A6
                        }
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 12.0
                            }
                        }
                    }
                }
            }
        }

        // 卡片状态
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            <Label> {
                width: Fill,
                height: Fit,
                text: "状态"
                draw_text: {
                    color: #666666
                    text_style: {
                        font_size: 14.0
                    }
                }
            }

            card_status = <Label> {
                width: Fill,
                height: Fit,
                text: "进行中"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 14.0
                    }
                }
            }
        }
    }
}
