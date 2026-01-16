pub mod workspace;
pub mod components;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    // 调用各组件的 live_design 函数
    // 每个组件的 live_design! 宏已生成对应的 live_design 函数
    components::boards_sidebar::live_design(cx);
    components::board_header::live_design(cx);
    components::board_menu::live_design(cx);
    components::board_members::live_design(cx);
    components::board_background::live_design(cx);
    components::board_toolbar::live_design(cx);
    components::kanban_list::live_design(cx);
    components::kanban_card::live_design(cx);
}
