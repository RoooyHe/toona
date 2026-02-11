use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 标签管理区域
    pub TagSection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 5,

        <Label> {
            width: Fill,
            height: Fit,
            text: "标签"
            draw_text: {
                color: #666666
                text_style: {
                    font_size: 14.0
                }
            }
        }

        <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 10,
            align: {y: 0.5}

            card_tags = <Label> {
                width: Fill,
                height: Fit,
                text: "暂无标签"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 14.0
                    }
                }
            }

            add_tag_button = <Button> {
                width: 80,
                height: 30,
                text: "添加标签"
                draw_bg: {
                    color: #x4ECDC4
                }
                draw_text: {
                    color: #FFFFFF
                    text_style: {
                        font_size: 12.0
                    }
                }
            }
        }

        // 标签下拉框
        tag_dropdown = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,
            visible: false,

            <Label> {
                width: Fill,
                height: Fit,
                text: "选择标签:"
                draw_text: {
                    color: #666666
                    text_style: {
                        font_size: 12.0
                    }
                }
            }

            // 暂时使用固定的标签按钮，后续可以改为 PortalList
            <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 3,

                tag_button_1 = <Button> {
                    width: Fill,
                    height: 25,
                    text: "标签1"
                    draw_bg: { color: #45B7D1 }
                    draw_text: {
                        color: #FFFFFF
                        text_style: { font_size: 12.0 }
                    }
                }

                tag_button_2 = <Button> {
                    width: Fill,
                    height: 25,
                    text: "标签2"
                    draw_bg: { color: #45B7D1 }
                    draw_text: {
                        color: #FFFFFF
                        text_style: { font_size: 12.0 }
                    }
                }

                tag_button_3 = <Button> {
                    width: Fill,
                    height: 25,
                    text: "标签3"
                    draw_bg: { color: #45B7D1 }
                    draw_text: {
                        color: #FFFFFF
                        text_style: { font_size: 12.0 }
                    }
                }
            }

            // 新增标签区域
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
                        text: "新增标签:"
                        draw_text: {
                            color: #666666
                            text_style: { font_size: 12.0 }
                        }
                    }

                    new_tag_button = <Button> {
                        width: 60,
                        height: 25,
                        text: "新增"
                        draw_bg: { color: #45B7D1 }
                        draw_text: {
                            color: #FFFFFF
                            text_style: { font_size: 12.0 }
                        }
                    }
                }

                new_tag_input_container = <View> {
                    width: Fill,
                    height: Fit,
                    visible: false,

                    new_tag_input = <TextInput> {
                        width: Fill,
                        height: 30,
                        text: "",
                        draw_text: {
                            color: #333333
                            text_style: { font_size: 12.0 }
                        }
                        draw_bg: { color: #F8F9FA }
                        draw_cursor: { color: #333333 }
                    }
                }
            }
        }
    }
}
