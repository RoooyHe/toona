use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::kanban::components::modal_header::ModalHeader;
    use crate::kanban::components::card_info_section::CardInfoSection;
    use crate::kanban::components::tag_section::TagSection;
    use crate::kanban::components::tag_management_modal::TagManagementModal;
    use crate::kanban::components::todo_section::TodoSection;
    use crate::kanban::components::endtime_section::EndTimeSection;
    use crate::kanban::components::active_section::ActiveSection;

    pub CardDetailModal = {{CardDetailModal}} {
        modal = <Modal> {
            content: <RoundedView> {
                width: 800,
                height: 600,
                padding: 20,
                flow: Down,
                spacing: 15,
                draw_bg: {
                    color: #FFFFFF
                }

                modal_header = <ModalHeader> {}

                <View> {
                    width: Fill,
                    height: Fill,
                    flow: Right,
                    spacing: 20,

                    <ScrollYView> {
                        width: Fit,
                        height: Fill,
                        scroll_bars: <ScrollBars> {
                            show_scroll_y: true
                        }

                        <View> {
                            width: 450,
                            height: Fit,
                            flow: Down,
                            spacing: 15,

                            <CardInfoSection> {}
                            <TagSection> {}
                            <EndTimeSection> {}
                            <TodoSection> {}
                        }
                    }

                    <ActiveSection> {
                        width: 350,
                        height: Fill,
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardDetailModal {
    #[deref]
    view: View,
}

impl Widget for CardDetailModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl CardDetailModalRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            inner.view.modal(ids!(modal)).open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            inner.view.modal(ids!(modal)).close(cx);
        }
    }
}
