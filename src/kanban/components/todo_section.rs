use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 待办事项管理区域
    pub TodoSection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 5,

        <Label> {
            width: Fill,
            height: Fit,
            text: "待办事项"
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

            card_todos = <Label> {
                width: Fill,
                height: Fit,
                text: "暂无待办事项"
                draw_text: {
                    color: #333333
                    text_style: { font_size: 14.0 }
                }
            }

            add_todo_button = <Button> {
                width: 80,
                height: 30,
                text: "添加待办"
                draw_bg: { color: #45B7D1 }
                draw_text: {
                    color: #FFFFFF
                    text_style: { font_size: 12.0 }
                }
            }
        }

        // Todo下拉框
        todo_dropdown = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,
            visible: false,

            <Label> {
                width: Fill,
                height: Fit,
                text: "待办事项管理:"
                draw_text: {
                    color: #666666
                    text_style: { font_size: 12.0 }
                }
            }

            // 新增Todo区域
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
                        text: "新增待办:"
                        draw_text: {
                            color: #666666
                            text_style: { font_size: 12.0 }
                        }
                    }

                    new_todo_button = <Button> {
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

                new_todo_input_container = <View> {
                    width: Fill,
                    height: Fit,
                    visible: false,

                    new_todo_input = <TextInput> {
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
