use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 活动记录区域
    pub ActiveSection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 5,

        <Label> {
            width: Fill,
            height: Fit,
            text: "活动记录"
            draw_text: {
                color: #666666
                text_style: { font_size: 14.0 }
            }
        }

        <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 10,
            align: {y: 0.5}

            card_active = <Label> {
                width: Fill,
                height: Fit,
                text: "暂无活动记录"
                draw_text: {
                    color: #333333
                    text_style: { font_size: 14.0 }
                }
            }

            add_active_button = <Button> {
                width: 80,
                height: 30,
                text: "添加活动"
                draw_bg: { color: #45B7D1 }
                draw_text: {
                    color: #FFFFFF
                    text_style: { font_size: 12.0 }
                }
            }
        }

        // Active下拉框
        active_dropdown = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,
            visible: false,

            <Label> {
                width: Fill,
                height: Fit,
                text: "活动记录管理:"
                draw_text: {
                    color: #666666
                    text_style: { font_size: 12.0 }
                }
            }

            // 新增Active区域
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
                        text: "新增活动:"
                        draw_text: {
                            color: #666666
                            text_style: { font_size: 12.0 }
                        }
                    }

                    new_active_button = <Button> {
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

                new_active_input_container = <View> {
                    width: Fill,
                    height: Fit,
                    visible: false,

                    new_active_input = <TextInput> {
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
