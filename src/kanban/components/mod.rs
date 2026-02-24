use makepad_widgets::*;

pub mod space;
pub mod card_list;
pub mod card_item;
pub mod card_modal;
pub mod boards_list;
pub mod card_detail_view;

// 模态框子组件
pub mod modal_header;
pub mod card_info_section;
pub mod tag_section;
pub mod todo_section;
pub mod endtime_section;
pub mod active_section;

pub use space::*;
pub use card_list::*;
pub use boards_list::*;
pub use card_detail_view::*;

// 导出组件的 live_design
pub fn live_design(cx: &mut Cx) {
    space::live_design(cx);
    card_list::live_design(cx);
    card_item::live_design(cx);
    boards_list::live_design(cx);
    card_detail_view::live_design(cx);
    
    // 模态框子组件
    modal_header::live_design(cx);
    card_info_section::live_design(cx);
    tag_section::live_design(cx);
    todo_section::live_design(cx);
    endtime_section::live_design(cx);
    active_section::live_design(cx);
    
    // 主模态框组件（依赖子组件，所以放在最后）
    card_modal::live_design(cx);
}