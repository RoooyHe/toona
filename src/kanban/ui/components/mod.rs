// Kanban UI Components
pub mod boards_sidebar;
pub mod board_header;
pub mod board_menu;
pub mod board_members;
pub mod board_background;
pub mod board_toolbar;
pub mod kanban_list;
pub mod kanban_card;

// Re-export main types for easy access
pub use boards_sidebar::{BoardsSidebar, BoardListItem};
pub use board_header::BoardHeader;
pub use board_menu::{BoardMenu, MenuItem, MenuDivider};
pub use board_members::{BoardMembersModal, BoardMemberItem};
pub use board_background::{BoardBackgroundModal, BgColorOption, BgPhotoOption};
pub use board_toolbar::{BoardToolbar, BoardViewMode};
pub use kanban_list::{KanbanList, KanbanListData, KanbanCardData};
pub use kanban_card::{KanbanCard, KanbanCardData as CardData, LabelColor, CardLabel};
