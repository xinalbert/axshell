use super::*;

impl AxShell {
    pub(super) fn render_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_group_index = self
            .active_group
            .as_ref()
            .and_then(|gid| self.tab_groups.iter().position(|g| g.id == *gid));
        let selected = if self.workspace_page == WorkspacePage::Settings && self.settings_page_open
        {
            self.tab_groups.len()
        } else {
            active_group_index.unwrap_or(0)
        };
        let groups_data: Vec<(String, String, Vec<String>)> = self
            .tab_groups
            .iter()
            .map(|g| {
                let pane_ids: Vec<String> = g
                    .pane_root
                    .tab_ids()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                (g.id.clone(), g.title.clone(), pane_ids)
            })
            .collect();
        let is_integrated =
            self.active_title_bar_style == crate::session::config::TitleBarStyle::Integrated;

        h_flex()
            .flex_1()
            .min_w(px(0.))
            .h_full()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .h_full()
                    .overflow_x_hidden()
                    .child({
                        TabBar::new("ax_shell-tab-bar")
                            .track_scroll(&self.tabs_scroll_handle)
                            .selected_index(selected)
                            .children(groups_data.iter().enumerate().map(
                                |(ix, (group_id, title, pane_ids))| {
                                    let gid = group_id.clone();
                                    let label = if pane_ids.len() > 1 {
                                        format!("{} ({})", title, pane_ids.len())
                                    } else {
                                        title.clone()
                                    };
                                    let close_id = if self.active_group.as_ref() == Some(&gid) {
                                        self.active_tab.clone().unwrap_or_else(|| {
                                            pane_ids.first().cloned().unwrap_or_default()
                                        })
                                    } else {
                                        pane_ids.first().cloned().unwrap_or_default()
                                    };

                                    let dot_color = pane_ids
                                        .first()
                                        .and_then(|id| self.tabs.iter().find(|t| t.id == *id))
                                        .map(|tab| {
                                            if tab.connected {
                                                cx.theme().success
                                            } else {
                                                cx.theme().danger
                                            }
                                        })
                                        .unwrap_or(cx.theme().success);
                                    Tab::new()
                                        .min_w(px(80.))
                                        .prefix(div().w(px(5.)).h(px(32.)).bg(dot_color))
                                        .child(
                                            div()
                                                .when(ix == selected, |this| {
                                                    this.font_weight(FontWeight::BOLD)
                                                        .text_color(cx.theme().primary)
                                                        .text_base()
                                                })
                                                .child(label),
                                        )
                                        .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                            window.prevent_default();
                                            cx.stop_propagation();
                                        })
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            cx.stop_propagation();
                                            this.activate_group(gid.clone(), window, cx)
                                        }))
                                        .suffix(
                                            Button::new(("tab-close", ix))
                                                .ghost()
                                                .xsmall()
                                                .icon(IconName::Close)
                                                .mr(px(5.))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    |_, window, cx| {
                                                        window.prevent_default();
                                                        cx.stop_propagation();
                                                    },
                                                )
                                                .on_click(cx.listener(
                                                    move |this, _, window, cx| {
                                                        window.prevent_default();
                                                        cx.stop_propagation();
                                                        if !close_id.is_empty() {
                                                            this.close_tab(close_id.clone(), cx)
                                                        }
                                                    },
                                                )),
                                        )
                                },
                            ))
                            .when(self.settings_page_open, |this| {
                                this.child(
                                    Tab::new()
                                        .min_w(px(120.))
                                        .prefix(div().w(px(5.)).h(px(32.)).bg(cx.theme().primary))
                                        .child(
                                            div()
                                                .when(
                                                    self.workspace_page == WorkspacePage::Settings,
                                                    |this| {
                                                        this.font_weight(FontWeight::BOLD)
                                                            .text_color(cx.theme().primary)
                                                            .text_base()
                                                    },
                                                )
                                                .child(t!("settings").to_string()),
                                        )
                                        .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                            window.prevent_default();
                                            cx.stop_propagation();
                                        })
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.open_settings_page(cx);
                                        }))
                                        .suffix(
                                            Button::new("settings-tab-close")
                                                .ghost()
                                                .xsmall()
                                                .icon(IconName::Close)
                                                .mr(px(5.))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    |_, window, cx| {
                                                        window.prevent_default();
                                                        cx.stop_propagation();
                                                    },
                                                )
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.close_settings_page(cx);
                                                })),
                                        ),
                                )
                            })
                            .when(is_integrated, |this| {
                                this.suffix(
                                    div()
                                        .id("tab-bar-drag-spacer")
                                        .w(px(56.))
                                        .h_full()
                                        .flex_shrink_0()
                                        .window_control_area(gpui::WindowControlArea::Drag),
                                )
                            })
                            .last_empty_space(div().w_3())
                            .w_full()
                            .h_full()
                    }),
            )
            .child(
                h_flex()
                    .on_mouse_down(MouseButton::Left, |_, window, cx| {
                        window.prevent_default();
                        cx.stop_propagation();
                    })
                    .flex_none()
                    .items_center()
                    .gap_1()
                    .pr(px(6.))
                    .child(
                        Button::new("open-selector")
                            .secondary()
                            .small()
                            .rounded(px(999.))
                            .icon(IconName::Plus)
                            .tooltip(t!("settings_open_session").to_string())
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_selector_dialog(window, cx)
                            })),
                    )
                    .child(
                        Button::new("split-horizontal")
                            .secondary()
                            .small()
                            .rounded(px(999.))
                            .icon(IconName::PanelBottom)
                            .tooltip(t!("settings_split_pane_down").to_string())
                            .disabled(self.workspace_page == WorkspacePage::Settings)
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.prevent_default();
                                cx.stop_propagation();
                                this.split_current_pane("down", cx);
                            })),
                    )
                    .child(
                        Button::new("split-vertical")
                            .secondary()
                            .small()
                            .rounded(px(999.))
                            .icon(IconName::PanelRight)
                            .tooltip(t!("settings_split_pane_right").to_string())
                            .disabled(self.workspace_page == WorkspacePage::Settings)
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.prevent_default();
                                cx.stop_propagation();
                                this.split_current_pane("right", cx);
                            })),
                    )
                    .when(self.workspace_page != WorkspacePage::Settings, |this| {
                        this.child(self.render_search_button(cx))
                    }),
            )
    }
}
