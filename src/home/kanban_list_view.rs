use makepad_widgets::*;

use crate::home::kanban_card::KanbanCardAction;

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

            card_1 = <KanbanCard> { width: Fill, height: 60 }
            card_2 = <KanbanCard> { width: Fill, height: 60 }
            card_3 = <KanbanCard> { width: Fill, height: 60 }
            card_4 = <KanbanCard> { width: Fill, height: 60 }
            card_5 = <KanbanCard> { width: Fill, height: 60 }
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
    #[rust]
    cards: Vec<KanbanCardSummary>,
    #[rust]
    card_widgets: Vec<(String, String)>, // (card_id, title)
}

impl Widget for KanbanListView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::Actions(actions) = event {
            log!("KanbanListView received actions");
            let card_ids = [
                ids!(cards_container.card_1),
                ids!(cards_container.card_2),
                ids!(cards_container.card_3),
                ids!(cards_container.card_4),
                ids!(cards_container.card_5),
            ];

            for (index, card_id) in card_ids.iter().enumerate() {
                if let Some((stored_card_id, _)) = self.card_widgets.get(index) {
                    if self.view.button(*card_id).clicked(actions) {
                        log!("KanbanListView: card {} clicked", stored_card_id);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            KanbanCardAction::Clicked {
                                card_id: stored_card_id.clone(),
                            },
                        );
                    }
                }
            }
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl KanbanListView {
    pub fn set_list(
        &mut self,
        cx: &mut Cx,
        title: &str,
        cards: &[KanbanCardSummary],
        selected_card_id: Option<&str>,
    ) {
        self.view
            .label(ids!(list_header.list_title))
            .set_text(cx, title);

        self.cards = cards.to_vec();
        self.card_widgets = cards
            .iter()
            .map(|card| (card.id.clone(), card.title.clone()))
            .collect();

        let card_ids = [
            ids!(cards_container.card_1),
            ids!(cards_container.card_2),
            ids!(cards_container.card_3),
            ids!(cards_container.card_4),
            ids!(cards_container.card_5),
        ];

        for (slot_index, card_id) in card_ids.iter().enumerate() {
            if let Some(card_data) = cards.get(slot_index) {
                let is_selected = selected_card_id == Some(card_data.id.as_str());
                self.view.view(*card_id).set_visible(cx, true);

                // 设置卡片标题
                if let Some(mut btn) = self.view.button(*card_id).borrow_mut() {
                    btn.label(ids!(card_title)).set_text(cx, &card_data.title);

                    // 设置背景颜色
                    let bg_color = if is_selected {
                        vec4(0.9, 0.95, 1.0, 1.0)
                    } else {
                        vec4(1.0, 1.0, 1.0, 1.0)
                    };
                    btn.apply_over(cx, live! { draw_bg: { color: (bg_color) } });
                }
            } else {
                self.view.view(*card_id).set_visible(cx, false);
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
