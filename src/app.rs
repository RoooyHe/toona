//! The top-level application content.
//!
//! See `handle_startup()` for the first code that runs on app startup.

use crate::{
    avatar_cache::clear_avatar_cache,
    home::{
        main_desktop_ui::MainDesktopUiAction,
        navigation_tab_bar::{NavigationBarAction, SelectedTab},
        new_message_context_menu::NewMessageContextMenuWidgetRefExt,
        room_screen::{clear_timeline_states, InviteAction, MessageAction},
        rooms_list::{
            clear_all_invited_rooms, enqueue_rooms_list_update, RoomsListAction, RoomsListRef,
            RoomsListUpdate,
        },
    },
    join_leave_room_modal::{
        JoinLeaveModalKind, JoinLeaveRoomModalAction, JoinLeaveRoomModalWidgetRefExt,
    },
    kanban::{
        KanbanActions, KanbanAppState,
    },
    kanban::components::edit_list_name_modal::EditListNameModalWidgetRefExt,
    login::login_screen::LoginAction,
    logout::logout_confirm_modal::{
        LogoutAction, LogoutConfirmModalAction, LogoutConfirmModalWidgetRefExt,
    },
    persistence,
    profile::user_profile_cache::clear_user_profile_cache,
    room::BasicRoomDetails,
    shared::{
        callout_tooltip::{CalloutTooltipWidgetRefExt, TooltipAction},
        confirmation_modal::ConfirmationModalWidgetRefExt,
        image_viewer::{ImageViewerAction, LoadState},
    },
    sliding_sync::{current_user_id, get_client, submit_async_request, MatrixRequest},
    utils::RoomNameId,
    verification::VerificationAction,
    verification_modal::{VerificationModalAction, VerificationModalWidgetRefExt},
};
use makepad_widgets::*;
use matrix_sdk::{
    ruma::{OwnedRoomId, RoomId},
    RoomState,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::home::home_screen::HomeScreen;
    use crate::verification_modal::VerificationModal;
    use crate::join_leave_room_modal::JoinLeaveRoomModal;
    use crate::login::login_screen::LoginScreen;
    use crate::logout::logout_confirm_modal::LogoutConfirmModal;
    use crate::shared::confirmation_modal::*;
    use crate::shared::popup_list::*;
    use crate::home::new_message_context_menu::*;
    use crate::shared::callout_tooltip::CalloutTooltip;
    use crate::shared::image_viewer::ImageViewer;
    use link::tsp_link::TspVerificationModal;
    use crate::kanban::components::card_modal::CardDetailModal;
    use crate::kanban::components::edit_list_name_modal::EditListNameModal;


    App = {{App}} {
        ui: <Root>{
            main_window = <Window> {
                window: {inner_size: vec2(1280, 800), title: "Toona"},
                pass: {clear_color: #FFFFFF00}
                caption_bar = {
                    caption_label = {
                        label = {
                            margin: {left: 65},
                            align: {x: 0.5},
                            text: "Toona",
                            draw_text: {color: (COLOR_TEXT)}
                        }
                    }
                    windows_buttons = {
                        // Note: these are the background colors of the buttons used in Windows:
                        // * idle: Clear, for all three buttons.
                        // * hover: #E9E9E9 for minimize and maximize, #E81123 for close.
                        // * down: either darker (on light mode) or lighter (on dark mode).
                        //
                        // However, the DesktopButton widget doesn't support drawing a background color yet,
                        // so these colors are the colors of the icon itself, not the background highlight.
                        // When it supports that, we will keep the icon color always black,
                        // and change the background color instead based on the above colors.
                        min   = { draw_bg: {color: #0, color_hover: #9, color_down: #3} }
                        max   = { draw_bg: {color: #0, color_hover: #9, color_down: #3} }
                        close = { draw_bg: {color: #0, color_hover: #E81123, color_down: #FF0015} }
                    }
                    draw_bg: {color: #F3F3F3},
                }


                body = {
                    padding: 0,

                    <View> {
                        width: Fill, height: Fill,
                        flow: Overlay,

                        home_screen_view = <View> {
                            visible: false
                            home_screen = <HomeScreen> {}
                        }
                        join_leave_modal = <Modal> {
                            content: {
                                join_leave_modal_inner = <JoinLeaveRoomModal> {}
                            }
                        }
                        login_screen_view = <View> {
                            visible: true
                            login_screen = <LoginScreen> {}
                        }

                        image_viewer_modal = <Modal> {
                            content: {
                                width: Fill, height: Fill,
                                image_viewer_modal_inner = <ImageViewer> {}
                            }
                        }
                        <PopupList> {}

                        // Context menus should be shown in front of other UI elements,
                        // but behind verification modals.
                        new_message_context_menu = <NewMessageContextMenu> { }

                        // A modal to confirm sending out an invite to a room.
                        invite_confirmation_modal = <Modal> {
                            content: {
                                invite_confirmation_modal_inner = <PositiveConfirmationModal> {
                                    wrapper = { buttons_view = { accept_button = {
                                        draw_icon: {
                                            svg_file: (ICON_INVITE),
                                        }
                                        icon_walk: {width: 28, height: Fit, margin: {left: -10} }
                                    } } }
                                }
                            }
                        }

                        // Show the logout confirmation modal.
                        logout_confirm_modal = <Modal> {
                            content: {
                                logout_confirm_modal_inner = <LogoutConfirmModal> {}
                            }
                        }

                        // Show the card detail modal for kanban cards
                        card_detail_modal = <CardDetailModal> {}
                        
                        // Show the edit list name modal for kanban lists
                        edit_list_name_modal = <Modal> {
                            content: {
                                edit_list_name_modal_inner = <EditListNameModal> {}
                            }
                        }

                        // Show incoming verification requests in front of the aforementioned UI elements.
                        verification_modal = <Modal> {
                            content: {
                                verification_modal_inner = <VerificationModal> {}
                            }
                        }
                        tsp_verification_modal = <Modal> {
                            content: {
                                tsp_verification_modal_inner = <TspVerificationModal> {}
                            }
                        }

                        // Tooltips must be shown in front of all other UI elements,
                        // since they can be shown as a hover atop any other widget.
                        app_tooltip = <CalloutTooltip> {}
                    }
                } // end of body
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
    /// The top-level app state, shared across various parts of the app.
    #[rust]
    app_state: AppState,
    /// The details of a room we're waiting on to be loaded so that we can navigate to it.
    /// This can be either a room we're waiting to join, or one we're waiting to be invited to.
    /// Also includes an optional room ID to be closed once the awaited room has been loaded.
    #[rust]
    waiting_to_navigate_to_room: Option<(BasicRoomDetails, Option<OwnedRoomId>)>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Order matters here, as some widget definitions depend on others.
        // `makepad_widgets` must be registered first,
        // then `shared`` widgets (in which styles are defined),
        // then other modules widgets.
        makepad_widgets::live_design(cx);
        // Override Makepad's default desktop dark theme with the desktop light theme.
        cx.link(id!(theme), id!(theme_desktop_light));
        crate::shared::live_design(cx);

        // If the `tsp` cargo feature is enabled, we create a new "tsp_link" DSL namespace
        // and link it to the real `tsp_enabled` DSL namespace, which contains real TSP widgets.
        // If the `tsp` feature is not enabled, link the "tsp_link" DSL namespace
        // to the `tsp_disabled` DSL namespace instead, which defines dummy placeholder widgets.
        #[cfg(feature = "tsp")]
        {
            crate::tsp::live_design(cx);
            cx.link(id!(tsp_link), id!(tsp_enabled));
        }
        #[cfg(not(feature = "tsp"))]
        {
            crate::tsp_dummy::live_design(cx);
            cx.link(id!(tsp_link), id!(tsp_disabled));
        }

        crate::settings::live_design(cx);
        crate::room::live_design(cx);
        crate::join_leave_room_modal::live_design(cx);
        crate::verification_modal::live_design(cx);
        crate::home::live_design(cx);
        crate::kanban::live_design(cx);
        crate::profile::live_design(cx);
        crate::login::live_design(cx);
        crate::logout::live_design(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // Here we set the global singleton for the PopupList widget,
        // which is used to access PopupList Widget from anywhere in the app.
        crate::shared::popup_list::set_global_popup_list(cx, &self.ui);
    }

    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        self.update_login_visibility(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // only init logging/tracing once
        let _ = tracing_subscriber::fmt::try_init();

        // Initialize the project directory here from the main UI thread
        // such that background threads/tasks will be able to can access it.
        let _app_data_dir = crate::app_data_dir();
        log!("App::handle_startup(): app_data_dir: {:?}", _app_data_dir);

        // 注释掉窗口状态恢复功能 - 每次启动使用默认窗口尺寸和位置
        // if let Err(e) = persistence::load_window_state(self.ui.window(ids!(main_window)), cx) {
        //     error!("Failed to load window state: {}", e);
        // }

        self.update_login_visibility(cx);

        log!("App::Startup: starting matrix sdk loop");
        let _tokio_rt_handle = crate::sliding_sync::start_matrix_tokio().unwrap();

        #[cfg(feature = "tsp")]
        {
            log!("App::Startup: initializing TSP (Trust Spanning Protocol) module.");
            crate::tsp::tsp_init(_tokio_rt_handle).unwrap();
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let invite_confirmation_modal_inner = self
            .ui
            .confirmation_modal(ids!(invite_confirmation_modal_inner));
        if let Some(_accepted) = invite_confirmation_modal_inner.closed(actions) {
            self.ui.modal(ids!(invite_confirmation_modal)).close(cx);
        }

        for action in actions {
            if let Some(kanban_action) = action.downcast_ref::<KanbanActions>() {
                self.handle_kanban_action(cx, kanban_action.clone());
                self.ui.redraw(cx);
                continue;
            }

            if let Some(logout_modal_action) = action.downcast_ref::<LogoutConfirmModalAction>() {
                match logout_modal_action {
                    LogoutConfirmModalAction::Open => {
                        self.ui
                            .logout_confirm_modal(ids!(logout_confirm_modal_inner))
                            .reset_state(cx);
                        self.ui.modal(ids!(logout_confirm_modal)).open(cx);
                        continue;
                    }
                    LogoutConfirmModalAction::Close { was_internal, .. } => {
                        if *was_internal {
                            self.ui.modal(ids!(logout_confirm_modal)).close(cx);
                        }
                        continue;
                    }
                    _ => {}
                }
            }

            if let Some(LogoutAction::LogoutSuccess) = action.downcast_ref() {
                self.app_state.logged_in = false;
                self.ui.modal(ids!(logout_confirm_modal)).close(cx);
                self.update_login_visibility(cx);
                self.ui.redraw(cx);
                continue;
            }

            if let Some(LogoutAction::ClearAppState { on_clear_appstate }) = action.downcast_ref() {
                // Clear user profile cache, invited_rooms timeline states
                clear_all_app_state(cx);
                // Reset all app state to its default.
                self.app_state = Default::default();
                on_clear_appstate.notify_one();
                continue;
            }

            if let Some(LoginAction::LoginSuccess) = action.downcast_ref() {
                log!("Received LoginAction::LoginSuccess, hiding login view.");
                self.app_state.logged_in = true;
                self.update_login_visibility(cx);
                self.ui.redraw(cx);
                continue;
            }

            // Handle an action requesting to open the new message context menu.
            if let MessageAction::OpenMessageContextMenu { details, abs_pos } =
                action.as_widget_action().cast()
            {
                self.ui.callout_tooltip(ids!(app_tooltip)).hide(cx);
                let new_message_context_menu = self
                    .ui
                    .new_message_context_menu(ids!(new_message_context_menu));
                let expected_dimensions = new_message_context_menu.show(cx, details);
                // Ensure the context menu does not spill over the window's bounds.
                let rect = self.ui.window(ids!(main_window)).area().rect(cx);
                let pos_x = min(abs_pos.x, rect.size.x - expected_dimensions.x);
                let pos_y = min(abs_pos.y, rect.size.y - expected_dimensions.y);
                new_message_context_menu.apply_over(
                    cx,
                    live! {
                        main_content = { margin: { left: (pos_x), top: (pos_y) } }
                    },
                );
                self.ui.redraw(cx);
                continue;
            }

            if let RoomsListAction::Selected(selected_room) = action.as_widget_action().cast() {
                // A room has been selected, update the app state and navigate to the main content view.
                let display_name = selected_room.room_name().to_string();
                self.app_state.selected_room = Some(selected_room);
                // Set the Stack Navigation header to show the name of the newly-selected room.
                self.ui
                    .label(ids!(main_content_view.header.content.title_container.title))
                    .set_text(cx, &display_name);

                // Navigate to the main content view
                cx.widget_action(
                    self.ui.widget_uid(),
                    &HeapLiveIdPath::default(),
                    StackNavigationAction::Push(id!(main_content_view)),
                );
                self.ui.redraw(cx);
                continue;
            }

            // Handle actions that instruct us to update the top-level app state.
            match action.downcast_ref() {
                Some(AppStateAction::RoomFocused(selected_room)) => {
                    self.app_state.selected_room = Some(selected_room.clone());
                    continue;
                }
                Some(AppStateAction::FocusNone) => {
                    self.app_state.selected_room = None;
                    continue;
                }
                Some(AppStateAction::UpgradedInviteToJoinedRoom(room_id)) => {
                    if let Some(selected_room) = self.app_state.selected_room.as_mut() {
                        let did_upgrade = selected_room.upgrade_invite_to_joined(room_id);
                        // Updating the AppState's selected room and issuing a redraw
                        // will cause the MainMobileUI to redraw the newly-joined room.
                        if did_upgrade {
                            self.ui.redraw(cx);
                        }
                    }
                    continue;
                }
                Some(AppStateAction::RestoreAppStateFromPersistentState(app_state)) => {
                    // Ignore the `logged_in` state that was stored persistently.
                    let logged_in_actual = self.app_state.logged_in;
                    self.app_state = app_state.clone();
                    self.app_state.logged_in = logged_in_actual;
                    cx.action(MainDesktopUiAction::LoadDockFromAppState);
                    continue;
                }
                Some(AppStateAction::NavigateToRoom {
                    room_to_close,
                    destination_room,
                }) => {
                    self.navigate_to_room(cx, room_to_close.as_ref(), destination_room);
                    continue;
                }
                // If we successfully loaded a room that we were waiting on,
                // we can now navigate to it and optionally close a previous room.
                Some(AppStateAction::RoomLoadedSuccessfully { room_name_id, .. })
                    if self
                        .waiting_to_navigate_to_room
                        .as_ref()
                        .is_some_and(|(dr, _)| dr.room_id() == room_name_id.room_id()) =>
                {
                    log!("Loaded awaited room {room_name_id:?}, navigating to it now...");
                    if let Some((dest_room, room_to_close)) =
                        self.waiting_to_navigate_to_room.take()
                    {
                        self.navigate_to_room(cx, room_to_close.as_ref(), &dest_room);
                    }
                    continue;
                }
                _ => {}
            }

            // Handle actions for showing or hiding the tooltip.
            match action.as_widget_action().cast() {
                TooltipAction::HoverIn {
                    text,
                    widget_rect,
                    options,
                } => {
                    // Don't show any tooltips if the message context menu is currently shown.
                    if self
                        .ui
                        .new_message_context_menu(ids!(new_message_context_menu))
                        .is_currently_shown(cx)
                    {
                        self.ui.callout_tooltip(ids!(app_tooltip)).hide(cx);
                    } else {
                        self.ui
                            .callout_tooltip(ids!(app_tooltip))
                            .show_with_options(cx, &text, widget_rect, options);
                    }
                    continue;
                }
                TooltipAction::HoverOut => {
                    self.ui.callout_tooltip(ids!(app_tooltip)).hide(cx);
                    continue;
                }
                _ => {}
            }

            // Handle actions needed to open/close the join/leave room modal.
            match action.downcast_ref() {
                Some(JoinLeaveRoomModalAction::Open { kind, show_tip }) => {
                    self.ui
                        .join_leave_room_modal(ids!(join_leave_modal_inner))
                        .set_kind(cx, kind.clone(), *show_tip);
                    self.ui.modal(ids!(join_leave_modal)).open(cx);
                    continue;
                }
                Some(JoinLeaveRoomModalAction::Close { was_internal, .. }) => {
                    if *was_internal {
                        self.ui.modal(ids!(join_leave_modal)).close(cx);
                    }
                    continue;
                }
                _ => {}
            }

            // `VerificationAction`s come from a background thread, so they are NOT widget actions.
            // Therefore, we cannot use `as_widget_action().cast()` to match them.
            //
            // Note: other verification actions are handled by the verification modal itself.
            if let Some(VerificationAction::RequestReceived(state)) = action.downcast_ref() {
                self.ui
                    .verification_modal(ids!(verification_modal_inner))
                    .initialize_with_data(cx, state.clone());
                self.ui.modal(ids!(verification_modal)).open(cx);
                continue;
            }
            if let Some(VerificationModalAction::Close) = action.downcast_ref() {
                self.ui.modal(ids!(verification_modal)).close(cx);
                continue;
            }
            match action.downcast_ref() {
                Some(ImageViewerAction::Show(LoadState::Loading(_, _))) => {
                    self.ui.modal(ids!(image_viewer_modal)).open(cx);
                    continue;
                }
                Some(ImageViewerAction::Hide) => {
                    self.ui.modal(ids!(image_viewer_modal)).close(cx);
                    continue;
                }
                _ => {}
            }
            // Handle actions to open/close the TSP verification modal.
            #[cfg(feature = "tsp")]
            {
                use std::ops::Deref;
                use crate::tsp::{
                    tsp_verification_modal::{
                        TspVerificationModalAction, TspVerificationModalWidgetRefExt,
                    },
                    TspIdentityAction,
                };

                if let Some(TspIdentityAction::ReceivedDidAssociationRequest {
                    details,
                    wallet_db,
                }) = action.downcast_ref()
                {
                    self.ui
                        .tsp_verification_modal(ids!(tsp_verification_modal_inner))
                        .initialize_with_details(cx, details.clone(), wallet_db.deref().clone());
                    self.ui.modal(ids!(tsp_verification_modal)).open(cx);
                    continue;
                }
                if let Some(TspVerificationModalAction::Close) = action.downcast_ref() {
                    self.ui.modal(ids!(tsp_verification_modal)).close(cx);
                    continue;
                }
            }

            // Handle a request to show the invite confirmation modal.
            if let Some(InviteAction::ShowConfirmationModal(content_opt)) = action.downcast_ref() {
                if let Some(content) = content_opt.borrow_mut().take() {
                    invite_confirmation_modal_inner.show(cx, content);
                    self.ui.modal(ids!(invite_confirmation_modal)).open(cx);
                }
                continue;
            }

            // Handle card detail modal close button
            if self.ui.button(ids!(card_detail_modal.content.modal_header.close_button)).clicked(actions) {
                self.ui.modal(ids!(card_detail_modal)).close(cx);
                continue;
            }

            // Handle edit list name modal buttons
            if self.ui.button(ids!(edit_list_name_modal_inner.save_button)).clicked(actions)
                || self.ui.button(ids!(edit_list_name_modal_inner.cancel_button)).clicked(actions)
            {
                self.ui.modal(ids!(edit_list_name_modal)).close(cx);
                continue;
            }

            // // message source modal handling.
            // match action.as_widget_action().cast() {
            //     MessageAction::MessageSourceModalOpen { room_id: _, event_id: _, original_json: _ } => {
            //        // self.ui.message_source(ids!(message_source_modal_inner)).initialize_with_data(room_id, event_id, original_json);
            //        // self.ui.modal(ids!(message_source_modal)).open(cx);
            //     }
            //     MessageAction::MessageSourceModalClose => {
            //         self.ui.modal(ids!(message_source_modal)).close(cx);
            //     }
            //     _ => {}
            // }
        }
    }
}

/// Clears all thread-local UI caches (user profiles, invited rooms, and timeline states).
/// The `cx` parameter ensures that these thread-local caches are cleared on the main UI thread,
fn clear_all_app_state(cx: &mut Cx) {
    clear_user_profile_cache(cx);
    clear_all_invited_rooms(cx);
    clear_timeline_states(cx);
    clear_avatar_cache(cx);
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Shutdown = event {
            // 注释掉窗口状态保存功能 - 不记录窗口尺寸和位置
            // let window_ref = self.ui.window(ids!(main_window));
            // if let Err(e) = persistence::save_window_state(window_ref, cx) {
            //     error!("Failed to save window state. Error: {e}");
            // }
            if let Some(user_id) = current_user_id() {
                let app_state = self.app_state.clone();
                if let Err(e) = persistence::save_app_state(app_state, user_id) {
                    error!("Failed to save app state. Error: {e}");
                }
            }
            #[cfg(feature = "tsp")]
            {
                // Save the TSP wallet state, if it exists, with a 3-second timeout.
                let tsp_state = std::mem::take(&mut *crate::tsp::tsp_state_ref().lock().unwrap());
                let res = crate::sliding_sync::block_on_async_with_timeout(
                    Some(std::time::Duration::from_secs(3)),
                    async move {
                        match tsp_state.close_and_serialize().await {
                            Ok(saved_state) => {
                                match persistence::save_tsp_state_async(saved_state).await {
                                    Ok(_) => {}
                                    Err(e) => error!("Failed to save TSP wallet state. Error: {e}"),
                                }
                            }
                            Err(e) => {
                                error!("Failed to close and serialize TSP wallet state. Error: {e}")
                            }
                        }
                    },
                );
                if let Err(_e) = res {
                    error!(
                        "Failed to save TSP wallet state before app shutdown. Error: Timed Out."
                    );
                }
            }
        }

        // Forward events to the MatchEvent trait implementation.
        self.match_event(cx, event);
        let scope = &mut Scope::with_data(&mut self.app_state);
        self.ui.handle_event(cx, event, scope);

        /*
         * TODO: I'd like for this to work, but it doesn't behave as expected.
         *       The context menu fails to draw properly when a draw event is passed to it.
         *       Also, once we do get this to work, we should remove the
         *       Hit::FingerScroll event handler in the new_message_context_menu widget.
         *
        // We only forward "interactive hit" events to the underlying UI view
        // if none of the various overlay views are visible.
        // Currently, the only overlay view that captures interactive events is
        // the new message context menu.
        // We always forward "non-interactive hit" events to the inner UI view.
        // We check which overlay views are visible in the order of those views' z-ordering,
        // such that the top-most views get a chance to handle the event first.

        let new_message_context_menu = self.ui.new_message_context_menu(ids!(new_message_context_menu));
        let is_interactive_hit = utils::is_interactive_hit_event(event);
        let is_pane_shown: bool;
        if new_message_context_menu.is_currently_shown(cx) {
            is_pane_shown = true;
            new_message_context_menu.handle_event(cx, event, scope);
        }
        else {
            is_pane_shown = false;
        }

        if !is_pane_shown || !is_interactive_hit {
            // Forward the event to the inner UI view.
            self.ui.handle_event(cx, event, scope);
        }
         *
         */
    }
}

impl App {
    fn update_login_visibility(&self, cx: &mut Cx) {
        let show_login = !self.app_state.logged_in;
        if !show_login {
            self.ui
                .modal(ids!(login_screen_view.login_screen.login_status_modal))
                .close(cx);
        }
        self.ui
            .view(ids!(login_screen_view))
            .set_visible(cx, show_login);
        self.ui
            .view(ids!(home_screen_view))
            .set_visible(cx, !show_login);
    }

    /// Navigates to the given `destination_room`, optionally closing the `room_to_close`.
    fn navigate_to_room(
        &mut self,
        cx: &mut Cx,
        room_to_close: Option<&OwnedRoomId>,
        destination_room: &BasicRoomDetails,
    ) {
        // A closure that closes the given `room_to_close`, if it exists in an open tab.
        let close_room_closure_opt = room_to_close.map(|to_close| {
            let tab_id = LiveId::from_str(to_close.as_str());
            let widget_uid = self.ui.widget_uid();
            move |cx: &mut Cx| {
                cx.widget_action(
                    widget_uid,
                    &HeapLiveIdPath::default(),
                    DockAction::TabCloseWasPressed(tab_id),
                );
                enqueue_rooms_list_update(RoomsListUpdate::HideRoom {
                    room_id: to_close.clone(),
                });
            }
        });

        let destination_room_id = destination_room.room_id();
        let new_selected_room = match cx
            .get_global::<RoomsListRef>()
            .get_room_state(destination_room_id)
        {
            Some(RoomState::Joined) => SelectedRoom::JoinedRoom {
                room_name_id: destination_room.room_name_id().clone(),
            },
            Some(RoomState::Invited) => SelectedRoom::InvitedRoom {
                room_name_id: destination_room.room_name_id().clone(),
            },
            // If the destination room is not yet loaded, show a join modal.
            _ => {
                log!(
                    "Destination room {:?} not loaded, showing join modal...",
                    destination_room.room_name_id()
                );
                self.waiting_to_navigate_to_room =
                    Some((destination_room.clone(), room_to_close.cloned()));
                cx.action(JoinLeaveRoomModalAction::Open {
                    kind: JoinLeaveModalKind::JoinRoom(destination_room.clone()),
                    show_tip: false,
                });
                return;
            }
        };

        log!(
            "Navigating to destination room {:?}, closing room {:?}",
            destination_room.room_name_id(),
            room_to_close,
        );

        // Before we navigate to the room, if the AddRoom tab is currently shown,
        // then we programmatically navigate to the Home tab to show the actual room.
        if matches!(self.app_state.selected_tab, SelectedTab::AddRoom) {
            cx.action(NavigationBarAction::GoToHome);
        }
        cx.widget_action(
            self.ui.widget_uid(),
            &HeapLiveIdPath::default(),
            RoomsListAction::Selected(new_selected_room),
        );
        // Select and scroll to the destination room in the rooms list.
        enqueue_rooms_list_update(RoomsListUpdate::ScrollToRoom(destination_room_id.clone()));

        // Close a previously/currently-open room if specified.
        if let Some(closure) = close_room_closure_opt {
            closure(cx);
        }
    }

    fn handle_kanban_action(&mut self, cx: &mut Cx, action: KanbanActions) {
        let state = &mut self.app_state.kanban_state;
        match action {
            KanbanActions::LoadLists => {
                // 加载所有 kanban Space（列表）
                if get_client().is_some() {
                    submit_async_request(MatrixRequest::LoadKanbanLists);
                    state.loading = true;
                }
            }

            KanbanActions::ListLoaded(list) => {
                // 列表已加载
                log!("ListLoaded: space_id='{}', name='{}'", list.id, list.name);
                state.upsert_list(list);
                self.ui.redraw(cx);
            }

            KanbanActions::CardLoaded(card) => {
                // 卡片已加载
                log!("CardLoaded: card_id='{}', title='{}'", card.id, card.title);
                
                // 添加卡片到 state
                let space_id = card.space_id.clone();
                let card_id = card.id.clone();
                state.upsert_card(card.clone());
                
                // 添加卡片 ID 到列表的 card_ids
                if let Some(list) = state.lists.get_mut(&space_id) {
                    if !list.card_ids.contains(&card.id) {
                        list.card_ids.push(card.id);
                    }
                }
                
                // 如果当前打开的模态框是这张卡片，强制重绘模态框
                if state.selected_card_id.as_ref() == Some(&card_id) {
                    log!("🔄 Forcing modal redraw for updated card {}", card_id);
                    // 强制重绘模态框内容
                    self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                }
                
                self.ui.redraw(cx);
            }

            KanbanActions::CreateList { name } => {
                // 创建新列表（Space）
                if get_client().is_some() {
                    submit_async_request(MatrixRequest::CreateKanbanList { name });
                    state.loading = true;
                }
            }

            KanbanActions::UpdateListName { list_id, name } => {
                // 更新列表名称
                log!("UpdateListName: list_id='{}', name='{}'", list_id, name);
                
                // 使用 update_list_name 方法只更新名称，保留卡片
                state.update_list_name(&list_id, name.clone());
                log!("✅ UpdateListName: 本地状态已更新，列表名称改为 '{}'", name);
                
                // 同步到 Matrix 服务器
                if get_client().is_some() {
                    submit_async_request(MatrixRequest::UpdateKanbanListName {
                        list_id,
                        name,
                    });
                }
                
                // 强制重绘整个 UI 以更新列表名称显示
                self.ui.redraw(cx);
                cx.redraw_all();
            }
            
            KanbanActions::ShowEditListName { list_id, current_name } => {
                // 显示编辑列表名称模态框
                log!("ShowEditListName: list_id='{}', current_name='{}'", list_id, current_name);
                
                // 设置数据
                self.ui
                    .edit_list_name_modal(ids!(edit_list_name_modal_inner))
                    .set_data(cx, list_id, &current_name);
                
                // 打开模态框
                self.ui.modal(ids!(edit_list_name_modal)).open(cx);
            }

            KanbanActions::CreateCard { space_id, title } => {
                // 在列表中创建新卡片
                log!("🎯 Received CreateCard action: space_id='{}', title='{}'", space_id, title);
                if get_client().is_some() {
                    log!("🎯 Submitting CreateKanbanCard request to worker thread...");
                    submit_async_request(MatrixRequest::CreateKanbanCard { space_id, title });
                    state.loading = true;
                    log!("🎯 Request submitted, loading=true");
                } else {
                    log!("❌ Cannot create card: Matrix client not available");
                }
            }

            KanbanActions::ShowCardDetail { card_id } => {
                // 显示卡片详情
                log!("ShowCardDetail: card_id='{}'", card_id);
                
                // 重新从 Matrix 加载完整的卡片数据（包括 todos）
                // 这确保即使重启后也能看到最新的数据
                if get_client().is_some() {
                    // 获取卡片所属的 space_id
                    if let Some(card) = state.cards.get(&card_id) {
                        let space_id = card.space_id.clone();
                        log!("🔄 Reloading card {} from Matrix to get fresh data", card_id);
                        submit_async_request(MatrixRequest::LoadCard {
                            card_id: card_id.clone(),
                            space_id,
                        });
                    }
                    
                    // 加载活动记录
                    submit_async_request(MatrixRequest::LoadCardActivities {
                        card_id: card_id.clone(),
                        limit: Some(50),
                    });
                }
                
                // 存储当前要显示的卡片 ID
                state.selected_card_id = Some(card_id);
                
                // 打开卡片详情模态框
                self.ui.modal(ids!(card_detail_modal)).open(cx);
                
                self.ui.redraw(cx);
            }

            KanbanActions::MoveCard {
                card_id,
                target_space_id,
                position,
            } => {
                // 移动卡片到不同列表
                if let Some(card) = state.cards.get_mut(&card_id) {
                    let old_space_id = card.space_id.clone();
                    card.space_id = target_space_id.clone();
                    card.position = position;
                    
                    // 从旧列表移除
                    if let Some(old_list) = state.lists.get_mut(&old_space_id) {
                        old_list.card_ids.retain(|id| id != &card_id);
                    }
                    
                    // 添加到新列表
                    if let Some(new_list) = state.lists.get_mut(&target_space_id) {
                        if !new_list.card_ids.contains(&card_id) {
                            new_list.card_ids.push(card_id);
                        }
                    }
                    
                    self.ui.redraw(cx);
                }
            }

            KanbanActions::UpdateCardTitle { card_id, title } => {
                // 更新卡片标题
                log!("UpdateCardTitle: card_id='{}', title='{}'", card_id, title);
                if let Some(card) = state.cards.get_mut(&card_id) {
                    card.title = title.clone();
                    self.ui.redraw(cx);
                }
                // TODO: 同步到 Matrix 服务器
            }

            KanbanActions::UpdateCardDescription {
                card_id,
                description,
            } => {
                // 更新卡片描述
                log!("UpdateCardDescription: card_id='{}', description='{:?}'", card_id, description);
                if let Some(card) = state.cards.get_mut(&card_id) {
                    card.description = description.clone();
                    self.ui.redraw(cx);
                }
                // TODO: 同步到 Matrix 服务器
            }

            KanbanActions::DeleteCard { card_id } => {
                // 删除卡片
                log!("DeleteCard: card_id='{}'", card_id);
                if let Some(card) = state.cards.remove(&card_id) {
                    // 从列表中移除卡片 ID
                    if let Some(list) = state.lists.get_mut(&card.space_id) {
                        list.card_ids.retain(|id| id != &card_id);
                    }
                    self.ui.redraw(cx);
                }
                // TODO: 同步到 Matrix 服务器（删除 Room）
            }
            
            // ========== Phase 2: TodoList Action Handlers ==========
            
            KanbanActions::AddTodo { card_id, text } => {
                log!("📝 AddTodo: card_id='{}', text='{}'", card_id, text);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    let new_todo = crate::kanban::state::kanban_state::TodoItem::new(text.clone());
                    card.todos.push(new_todo);
                    card.touch();
                    log!("✅ Added todo in memory immediately");
                    
                    // 如果模态框打开的是这张卡片，立即重绘
                    if state.selected_card_id.as_ref() == Some(&card_id) {
                        log!("🔄 Forcing immediate modal redraw");
                        self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                    }
                    self.ui.redraw(cx);
                    
                    // 异步保存到 Matrix（传递完整的todos列表）
                    if get_client().is_some() {
                        let todos_clone = card.todos.clone();
                        submit_async_request(MatrixRequest::SaveCardTodos { 
                            card_id: card_id.clone(), 
                            todos: todos_clone 
                        });
                    }
                }
            }
            
            KanbanActions::ToggleTodo { card_id, todo_id } => {
                log!("✅ ToggleTodo: card_id='{}', todo_id='{}'", card_id, todo_id);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    if let Some(todo) = card.todos.iter_mut().find(|t| t.id == todo_id) {
                        todo.completed = !todo.completed;
                        card.touch();
                        log!("✅ Toggled todo in memory immediately");
                        
                        // 如果模态框打开的是这张卡片，立即重绘
                        if state.selected_card_id.as_ref() == Some(&card_id) {
                            log!("🔄 Forcing immediate modal redraw");
                            self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                        }
                        self.ui.redraw(cx);
                        
                        // 异步保存到 Matrix
                        if get_client().is_some() {
                            let todos_clone = card.todos.clone();
                            submit_async_request(MatrixRequest::SaveCardTodos { 
                                card_id: card_id.clone(), 
                                todos: todos_clone 
                            });
                        }
                    }
                }
            }
            
            KanbanActions::UpdateTodoText { card_id, todo_id, text } => {
                log!("✏️ UpdateTodoText: card_id='{}', todo_id='{}', text='{}'", card_id, todo_id, text);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    if let Some(todo) = card.todos.iter_mut().find(|t| t.id == todo_id) {
                        todo.text = text.clone();
                        card.touch();
                        log!("✅ Updated todo text in memory immediately");
                        
                        // 如果模态框打开的是这张卡片，立即重绘
                        if state.selected_card_id.as_ref() == Some(&card_id) {
                            log!("🔄 Forcing immediate modal redraw");
                            self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                        }
                        self.ui.redraw(cx);
                        
                        // 异步保存到 Matrix
                        if get_client().is_some() {
                            let todos_clone = card.todos.clone();
                            submit_async_request(MatrixRequest::SaveCardTodos { 
                                card_id: card_id.clone(), 
                                todos: todos_clone 
                            });
                        }
                    }
                }
            }
            
            KanbanActions::DeleteTodo { card_id, todo_id } => {
                log!("🗑️ DeleteTodo: card_id='{}', todo_id='{}'", card_id, todo_id);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    card.todos.retain(|t| t.id != todo_id);
                    card.touch();
                    log!("✅ Deleted todo in memory immediately");
                    
                    // 如果模态框打开的是这张卡片，立即重绘
                    if state.selected_card_id.as_ref() == Some(&card_id) {
                        log!("🔄 Forcing immediate modal redraw");
                        self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                    }
                    self.ui.redraw(cx);
                    
                    // 异步保存到 Matrix
                    if get_client().is_some() {
                        let todos_clone = card.todos.clone();
                        submit_async_request(MatrixRequest::SaveCardTodos { 
                            card_id: card_id.clone(), 
                            todos: todos_clone 
                        });
                    }
                }
            }
            
            // ========== Phase 3: Tags Action Handlers ==========
            
            KanbanActions::AddTag { card_id, tag } => {
                log!("🏷️ AddTag: card_id='{}', tag='{}'", card_id, tag);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    if !card.tags.contains(&tag) {
                        card.tags.push(tag.clone());
                        card.touch();
                        log!("✅ Added tag '{}' in memory immediately", tag);
                        
                        // 如果模态框打开的是这张卡片，立即重绘
                        if state.selected_card_id.as_ref() == Some(&card_id) {
                            log!("🔄 Forcing immediate modal redraw");
                            self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                        }
                        self.ui.redraw(cx);
                        
                        // 异步保存到 Matrix（传递完整的卡片数据）
                        if get_client().is_some() {
                            let card_clone = card.clone();
                            submit_async_request(MatrixRequest::SaveCardMetadata { card: card_clone });
                        }
                    }
                }
            }
            
            KanbanActions::RemoveTag { card_id, tag } => {
                log!("🗑️ RemoveTag: card_id='{}', tag='{}'", card_id, tag);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    card.tags.retain(|t| t != &tag);
                    card.touch();
                    log!("✅ Removed tag '{}' in memory immediately", tag);
                    
                    // 如果模态框打开的是这张卡片，立即重绘
                    if state.selected_card_id.as_ref() == Some(&card_id) {
                        log!("🔄 Forcing immediate modal redraw");
                        self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                    }
                    self.ui.redraw(cx);
                    
                    // 异步保存到 Matrix（传递完整的卡片数据）
                    if get_client().is_some() {
                        let card_clone = card.clone();
                        submit_async_request(MatrixRequest::SaveCardMetadata { card: card_clone });
                    }
                }
            }
            
            // ========== Phase 4: EndTime Action Handlers ==========
            
            KanbanActions::SetEndTime { card_id, end_time } => {
                log!("⏰ SetEndTime: card_id='{}', end_time={}", card_id, end_time);
                
                // 立即更新内存中的 state（乐观更新）
                if let Some(card) = state.cards.get_mut(&card_id) {
                    card.end_time = Some(end_time);
                    card.touch();
                    log!("✅ Updated end_time in memory immediately");
                    
                    // 如果模态框打开的是这张卡片，立即重绘
                    if state.selected_card_id.as_ref() == Some(&card_id) {
                        log!("🔄 Forcing immediate modal redraw");
                        self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                    }
                    self.ui.redraw(cx);
                    
                    // 异步保存到 Matrix（传递完整的卡片数据）
                    if get_client().is_some() {
                        let card_clone = card.clone();
                        submit_async_request(MatrixRequest::SaveCardMetadata { card: card_clone });
                    }
                }
            }
            
            KanbanActions::ClearEndTime { card_id } => {
                log!("🗑️ ClearEndTime: card_id='{}'", card_id);
                
                // 立即更新内存中的 state
                if let Some(card) = state.cards.get_mut(&card_id) {
                    card.end_time = None;
                    card.touch();
                    log!("✅ Cleared end_time in memory immediately");
                    
                    // 如果模态框打开的是这张卡片，立即重绘
                    if state.selected_card_id.as_ref() == Some(&card_id) {
                        log!("🔄 Forcing immediate modal redraw");
                        self.ui.view(ids!(card_detail_modal.content)).redraw(cx);
                    }
                    self.ui.redraw(cx);
                    
                    // 异步保存到 Matrix（传递完整的卡片数据）
                    if get_client().is_some() {
                        let card_clone = card.clone();
                        submit_async_request(MatrixRequest::SaveCardMetadata { card: card_clone });
                    }
                }
            }
            
            // ========== Phase 5: Activities Action Handlers ==========
            
            KanbanActions::AddComment { card_id, text } => {
                log!("💬 AddComment: card_id='{}', text='{}'", card_id, text);
                if get_client().is_some() {
                    submit_async_request(MatrixRequest::AddCardComment { card_id, text });
                }
            }
            
            KanbanActions::ActivitiesLoaded { card_id, activities } => {
                log!("📖 ActivitiesLoaded: card_id='{}', count={}", card_id, activities.len());
                state.activities.insert(card_id, activities);
            }

            KanbanActions::Loading(loading) => {
                state.loading = loading;
            }

            KanbanActions::Error(message) => {
                state.error = Some(message);
                state.loading = false;
            }
        }
    }
}

/// App-wide state that is stored persistently across multiple app runs
/// and shared/updated across various parts of the app.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct AppState {
    /// The currently-selected room, which is highlighted (selected) in the RoomsList
    /// and considered "active" in the main rooms screen.
    pub selected_room: Option<SelectedRoom>,
    /// The currently-selected navigation tab: defines which top-level view is shown.
    ///
    /// This field is only updated by the `HomeScreen` widget, which has the
    /// necessary context to be able to determine how it should be modified.
    ///
    /// This is not saved to or restored from persistent storage,
    /// so the `Home` screen and tab are always selected upon app startup.
    #[serde(skip)]
    pub selected_tab: SelectedTab,
    /// The saved "snapshot" of the dock's UI layout/state for the main "all rooms" home view.
    pub saved_dock_state_home: SavedDockState,
    /// The saved "snapshot" of the dock's UI layout/state for each space,
    /// keyed by the space ID.
    pub saved_dock_state_per_space: HashMap<OwnedRoomId, SavedDockState>,
    /// Whether a user is currently logged in to Robrix or not.
    pub logged_in: bool,
    /// Kanban module state (not persisted).
    #[serde(skip)]
    pub kanban_state: KanbanAppState,
}

/// A snapshot of the main dock: all state needed to restore the dock tabs/layout.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SavedDockState {
    /// All items contained in the dock, keyed by their room or space ID.
    pub dock_items: HashMap<LiveId, DockItem>,
    /// The rooms that are currently open, keyed by their room or space ID.
    pub open_rooms: HashMap<LiveId, SelectedRoom>,
    /// The order in which the rooms were opened, in chronological order
    /// from first opened (at the beginning) to last opened (at the end).
    pub room_order: Vec<SelectedRoom>,
    /// The selected room tab in this dock when the dock state was saved.
    pub selected_room: Option<SelectedRoom>,
}

/// Represents a room currently or previously selected by the user.
///
/// One `SelectedRoom` is considered equal to another if their `room_id`s are equal.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SelectedRoom {
    JoinedRoom { room_name_id: RoomNameId },
    InvitedRoom { room_name_id: RoomNameId },
    Space { space_name_id: RoomNameId },
}

impl SelectedRoom {
    pub fn room_id(&self) -> &OwnedRoomId {
        match self {
            SelectedRoom::JoinedRoom { room_name_id } => room_name_id.room_id(),
            SelectedRoom::InvitedRoom { room_name_id } => room_name_id.room_id(),
            SelectedRoom::Space { space_name_id } => space_name_id.room_id(),
        }
    }

    pub fn room_name(&self) -> &RoomNameId {
        match self {
            SelectedRoom::JoinedRoom { room_name_id } => room_name_id,
            SelectedRoom::InvitedRoom { room_name_id } => room_name_id,
            SelectedRoom::Space { space_name_id } => space_name_id,
        }
    }

    /// Upgrades this room from an invite to a joined room
    /// if its `room_id` matches the given `room_id`.
    ///
    /// Returns `true` if the room was an `InvitedRoom` with the same `room_id`
    /// that was successfully upgraded to a `JoinedRoom`;
    /// otherwise, returns `false`.
    pub fn upgrade_invite_to_joined(&mut self, room_id: &RoomId) -> bool {
        match self {
            SelectedRoom::InvitedRoom { room_name_id } if room_name_id.room_id() == room_id => {
                let name = room_name_id.clone();
                *self = SelectedRoom::JoinedRoom { room_name_id: name };
                true
            }
            _ => false,
        }
    }
}
impl PartialEq for SelectedRoom {
    fn eq(&self, other: &Self) -> bool {
        self.room_id() == other.room_id()
    }
}
impl Eq for SelectedRoom {}

/// Actions sent to the top-level App in order to update / restore its [`AppState`].
///
/// These are *NOT* widget actions.
#[derive(Debug)]
pub enum AppStateAction {
    /// The given room was focused (selected).
    RoomFocused(SelectedRoom),
    /// Resets the focus to none, meaning that no room is selected.
    FocusNone,
    /// The given room has successfully been upgraded from being displayed
    /// as an InviteScreen to a RoomScreen.
    UpgradedInviteToJoinedRoom(OwnedRoomId),
    /// The given app state was loaded from persistent storage
    /// and is ready to be restored.
    RestoreAppStateFromPersistentState(AppState),
    /// The given room was successfully loaded from the homeserver
    /// and is now known to our client.
    ///
    /// The RoomScreen for this room can now fully display the room's timeline.
    RoomLoadedSuccessfully {
        room_name_id: RoomNameId,
        /// `true` if this room is an invitation, `false` otherwise.
        is_invite: bool,
    },
    /// A request to navigate to a different room, optionally closing a prior/current room.
    NavigateToRoom {
        room_to_close: Option<OwnedRoomId>,
        destination_room: BasicRoomDetails,
    },
    None,
}
