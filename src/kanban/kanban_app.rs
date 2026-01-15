use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub KanbanApp = {{KanbanApp}} {
        width: Fill, height: Fill
        flow: Down
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanApp {
    #[deref]
    view: View,
}

impl Widget for KanbanApp {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        // Handle basic events
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, _scope, walk)
    }
}