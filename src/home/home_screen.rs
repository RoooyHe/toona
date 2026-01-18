use makepad_widgets::*;

use crate::{
    app::AppState,
    home::navigation_tab_bar::{NavigationBarAction, SelectedTab},
    kanban::KanbanActions,
    settings::settings_screen::SettingsScreenWidgetRefExt,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::home::main_mobile_ui::MainMobileUI;
    use crate::home::rooms_sidebar::RoomsSideBar;
    use crate::home::navigation_tab_bar::NavigationTabBar;
    use crate::home::search_messages::*;
    use crate::home::spaces_bar::*;
    use crate::home::add_room::*;
    use crate::kanban::kanban_app::KanbanApp;
    use crate::shared::styles::*;
    use crate::shared::room_filter_input_bar::RoomFilterInputBar;
    use crate::home::main_desktop_ui::MainDesktopUI;
    use crate::settings::settings_screen::SettingsScreen;


    StackNavigationWrapper = {{StackNavigationWrapper}} {
        view_stack = <StackNavigation> {}
    }

    SpacesBarWrapper = {{SpacesBarWrapper}}<RoundedShadowView> {
        width: Fill,
        height: (NAVIGATION_TAB_BAR_SIZE)
        margin: {left: 4, right: 4}
        show_bg: true
        draw_bg: {
            color: (COLOR_PRIMARY_DARKER),
            border_radius: 4.0,
            border_size: 0.0
            shadow_color: #0005
            shadow_radius: 15.0
            shadow_offset: vec2(1.0, 0.0)
        }

        <CachedWidget> {
            root_spaces_bar = <SpacesBar> {}
        }

        animator: {
            spaces_bar_animator = {
                default: hide,
                show = {
                    from: { all: Forward { duration: (SPACES_BAR_ANIMATION_DURATION_SECS) } }
                    apply: { height: (NAVIGATION_TAB_BAR_SIZE),  draw_bg: { shadow_color: #x00000055 } }
                }
                hide = {
                    from: { all: Forward { duration: (SPACES_BAR_ANIMATION_DURATION_SECS) } }
                    apply: { height: 0,  draw_bg: { shadow_color: #x00000000 } }
                }
            }
        }
    }

    pub HomeScreen = {{HomeScreen}} {
        <AdaptiveView> {
            Desktop = {
                show_bg: true
                draw_bg: {
                    color: (COLOR_SECONDARY),
                }
                width: Fill, height: Fill
                flow: Right
                align: {x: 0.0, y: 0.0}
                padding: 0,
                margin: 0,

                <CachedWidget> {
                    navigation_tab_bar = <NavigationTabBar> {}
                }

                home_screen_page_flip = <PageFlip> {
                    width: Fill, height: Fill

                    lazy_init: true,
                    active_page: home_page

                    home_page = <View> {
                        width: Fill, height: Fill
                        flow: Down
                    }

                    settings_page = <View> {
                        width: Fill, height: Fill
                        show_bg: true,
                        draw_bg: {
                            color: (COLOR_PRIMARY)
                        }

                        <CachedWidget> {
                            settings_screen = <SettingsScreen> {}
                        }
                    }

                    kanban_page = <View> {
                        width: Fill, height: Fill
                        show_bg: true,
                        draw_bg: {
                            color: #F4F5F7
                        },

                        flow: Down,

                        kanban_app = <KanbanApp> {
                            width: Fill, height: Fill
                        }
                    }

                    add_room_page = <View> {
                        width: Fill, height: Fill
                        show_bg: true,
                        draw_bg: {
                            color: (COLOR_PRIMARY)
                        }

                        <CachedWidget> {
                            add_room_screen = <AddRoomScreen> {}
                        }
                    }
                }
            }

            Mobile = {
                width: Fill, height: Fill
                flow: Down

                show_bg: true
                draw_bg: {
                    color: (COLOR_PRIMARY)
                }

                <StackNavigationWrapper> {
                    view_stack = <StackNavigation> {

                        root_view = {
                            padding: {top: 40.}
                            flow: Down
                            width: Fill, height: Fill

                            home_screen_page_flip = <PageFlip> {
                                width: Fill, height: Fill

                                lazy_init: true,
                                active_page: home_page

                                home_page = <View> {
                                    width: Fill, height: Fill
                                    flow: Down

                                    <RoomsSideBar> {}
                                }

                                settings_page = <View> {
                                    width: Fill, height: Fill

                                    <CachedWidget> {
                                        settings_screen = <SettingsScreen> {}
                                    }
                                }

                                add_room_page = <View> {
                                    width: Fill, height: Fill

                                    <CachedWidget> {
                                        add_room_screen = <AddRoomScreen> {}
                                    }
                                }
                            }

                            <CachedWidget> {
                                spaces_bar_wrapper = <SpacesBarWrapper> {}
                            }

                            <CachedWidget> {
                                navigation_tab_bar = <NavigationTabBar> {}
                            }
                        }

                        main_content_view = <StackNavigationView> {
                            width: Fill, height: Fill
                            header = {
                                content: {
                                    button_container = {
                                        padding: {left: 14}
                                    }
                                    title_container = {
                                        title = {
                                            draw_text: {
                                                color: (ROOM_NAME_TEXT_COLOR)
                                            }
                                        }
                                    }
                                }
                            }
                            body: {
                                main_content = <MainMobileUI> {}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A simple wrapper around the SpacesBar that allows us to animate showing or hiding it.
#[derive(Live, LiveHook, Widget)]
pub struct SpacesBarWrapper {
    #[deref]
    view: View,
    #[animator]
    animator: Animator,
}

impl Widget for SpacesBarWrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // TODO: i want to uncomment this, but adding it back in will break
        //       the animation of showing the SpacesBarWrapper.
        //       I'm not sure why the SpacesBar is getting redrawn constantly though.
        // if walk.height.to_fixed().is_some_and(|h| h < 0.01) {
        //     return DrawStep::done();
        // }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SpacesBarWrapperRef {
    /// Shows or hides the spaces bar by animating it in or out.
    fn show_or_hide(&self, cx: &mut Cx, show: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        if show {
            inner.animator_play(cx, ids!(spaces_bar_animator.show));
        } else {
            inner.animator_play(cx, ids!(spaces_bar_animator.hide));
        }
        inner.redraw(cx);
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HomeScreen {
    #[deref]
    view: View,

    /// The previously-selected navigation tab, used to determine which tab
    /// and top-level view we return to after closing the settings screen.
    ///
    /// Note that the current selected tap is stored in `AppState` so that
    /// other widgets can easily access it.
    #[rust]
    previous_selection: SelectedTab,
    #[rust]
    is_spaces_bar_shown: bool,
}

impl Widget for HomeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::Actions(actions) = event {
            let app_state = scope.data.get_mut::<AppState>().unwrap();
            for action in actions {
                match action.downcast_ref() {
                    Some(NavigationBarAction::GoToHome) => {
                        if !matches!(app_state.selected_tab, SelectedTab::Home) {
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::Home;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::GoToAddRoom) => {
                        if !matches!(app_state.selected_tab, SelectedTab::AddRoom) {
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::AddRoom;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::GoToSpace { space_name_id }) => {
                        let new_space_selection = SelectedTab::Space {
                            space_name_id: space_name_id.clone(),
                        };
                        if app_state.selected_tab != new_space_selection {
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = new_space_selection;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    // Only open the settings screen if it is not currently open.
                    Some(NavigationBarAction::OpenSettings) => {
                        if !matches!(app_state.selected_tab, SelectedTab::Settings) {
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::Settings;
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            if let Some(settings_page) =
                                self.update_active_page_from_selection(cx, app_state)
                            {
                                settings_page
                                    .settings_screen(ids!(settings_screen))
                                    .populate(cx, None);
                                self.view.redraw(cx);
                            } else {
                                error!("BUG: failed to set active page to show settings screen.");
                            }
                        }
                    }
                    Some(NavigationBarAction::CloseSettings) => {
                        if matches!(app_state.selected_tab, SelectedTab::Settings) {
                            app_state.selected_tab = self.previous_selection.clone();
                            cx.action(NavigationBarAction::TabSelected(
                                app_state.selected_tab.clone(),
                            ));
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::GoToKanban) => {
                        if !matches!(app_state.selected_tab, SelectedTab::Kanban) {
                            self.previous_selection = app_state.selected_tab.clone();
                            app_state.selected_tab = SelectedTab::Kanban;
                            cx.action(NavigationBarAction::TabSelected(SelectedTab::Kanban));
                            cx.action(KanbanActions::LoadBoards);
                            self.update_active_page_from_selection(cx, app_state);
                            self.view.redraw(cx);
                        }
                    }
                    Some(NavigationBarAction::ToggleSpacesBar) => {
                        self.is_spaces_bar_shown = !self.is_spaces_bar_shown;
                        self.view
                            .spaces_bar_wrapper(ids!(spaces_bar_wrapper))
                            .show_or_hide(cx, self.is_spaces_bar_shown);
                    }
                    // We're the ones who emitted this action, so we don't need to handle it again.
                    Some(NavigationBarAction::TabSelected(_)) | None => {}
                }
            }
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let app_state = scope.data.get_mut::<AppState>().unwrap();
        // Note: We need to update the active page before drawing,
        // because if we switched between Desktop and Mobile views,
        // the PageFlip widget will have been reset to its default,
        // so we must re-set it to the correct page based on `app_state.selected_tab`.
        self.update_active_page_from_selection(cx, app_state);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl HomeScreen {
    fn update_active_page_from_selection(
        &mut self,
        cx: &mut Cx,
        app_state: &mut AppState,
    ) -> Option<WidgetRef> {
        self.view
            .page_flip(ids!(home_screen_page_flip))
            .set_active_page(
                cx,
                match app_state.selected_tab {
                    SelectedTab::Space { .. } | SelectedTab::Home => id!(home_page),
                    SelectedTab::Settings => id!(settings_page),
                    SelectedTab::AddRoom => id!(add_room_page),
                    SelectedTab::Kanban => id!(kanban_page),
                },
            )
    }
}

/// A wrapper around the StackNavigation widget
/// that simply forwards stack view actions to it.
#[derive(Live, LiveHook, Widget)]
pub struct StackNavigationWrapper {
    #[deref]
    view: View,
}

impl Widget for StackNavigationWrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for StackNavigationWrapper {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.stack_navigation(ids!(view_stack))
            .handle_stack_view_actions(cx, actions);
    }
}
