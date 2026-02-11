use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 模态框标题栏组件
    pub ModalHeader = <RoundedView> {
        width: Fill,
        height: Fit,
        flow: Right,
        align: {y: 0.5}

        <Label> {
            width: Fill,
            height: Fit,
            text: "卡片详情"
            draw_text: {
                color: #333333
                text_style: {
                    font_size: 20.0
                }
            }
        }

        close_button = <Button> {
            width: 30,
            height: 30,
            text: "×"
            draw_bg: {
                color: #FF6B6B
            }
            draw_text: {
                color: #FFFFFF
                text_style: {
                    font_size: 18.0
                }
            }
        }
    }
}
