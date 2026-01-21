use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::home::kanban_card::KanbanCard;

    pub KanbanListView = {{KanbanListView}} {
        width: 280, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: #EBECF0
        }

        list_header = <View> {
            width: Fill, height: 40
            padding: 12
            show_bg: true
            draw_bg: {
                color: #EBECF0
            }

            list_title = <Label> {
                width: Fill, height: Fit
                text: "待办"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #172B4D
                }
            }
        }

        cards_container = <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: 8
            padding: 8

            card_1 = <KanbanCard> { visible: false }
            card_2 = <KanbanCard> { visible: false }
            card_3 = <KanbanCard> { visible: false }
            card_4 = <KanbanCard> { visible: false }
            card_5 = <KanbanCard> { visible: false }
        }
    }
}

#[derive(Clone, Debug)]
pub struct KanbanCardSummary {
    pub id: String,
    pub title: String,
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanListView {
    #[deref]
    view: View,
}

impl Widget for KanbanListView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl KanbanListView {
    fn set_list(
        &mut self,
        cx: &mut Cx,
        title: &str,
        cards: &[KanbanCardSummary],
        selected_card_id: Option<&str>,
    ) {
        self.view
            .label(ids!(list_header.list_title))
            .set_text(cx, title);

        let card_slots = [
            ids!(cards_container.card_1),
            ids!(cards_container.card_2),
            ids!(cards_container.card_3),
            ids!(cards_container.card_4),
            ids!(cards_container.card_5),
        ];

        for (slot_index, slot_id) in card_slots.iter().enumerate() {
            let card = self.view.kanban_card(*slot_id);
            if let Some(card_data) = cards.get(slot_index) {
                let is_selected = selected_card_id == Some(card_data.id.as_str());
                card.set_visible(cx, true);
                card.borrow_mut().unwrap().set_card(
                    cx,
                    &card_data.id,
                    &card_data.title,
                    is_selected,
                );
            } else {
                card.set_visible(cx, false);
            }
        }
    }
}

impl KanbanListViewRef {
    pub fn set_list(
        &self,
        cx: &mut Cx,
        title: &str,
        cards: &[KanbanCardSummary],
        selected_card_id: Option<&str>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_list(cx, title, cards, selected_card_id);
    }
}

pub trait KanbanListViewWidgetExt {
    fn kanban_list_view(&self, id: WidgetId) -> KanbanListViewRef;
}

impl KanbanListViewWidgetExt for View {
    fn kanban_list_view(&self, id: WidgetId) -> KanbanListViewRef {
        self.child(id).unwrap()
    }
}
