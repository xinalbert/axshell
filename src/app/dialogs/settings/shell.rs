use super::*;

pub(super) fn settings_page_shell(
    view: gpui::Entity<AxShell>,
    focus_handle: &gpui::FocusHandle,
    content: impl gpui::IntoElement,
) -> impl gpui::IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .track_focus(focus_handle)
        .on_key_down({
            let view = view.clone();
            move |ev: &gpui::KeyDownEvent, window, cx| {
                view.update(cx, |this, cx| {
                    if this.recording_action.is_none()
                        && crate::app::keybinding_recorder::event_matches_action(
                            &this.config,
                            "OpenSettings",
                            ev,
                        )
                    {
                        this.request_close_settings_page(window, cx);
                        window.prevent_default();
                        cx.stop_propagation();
                        return;
                    }

                    if window.focused(cx) != Some(this.focus_handle.clone()) {
                        if this.recording_action.is_some() {
                            this.recording_action = None;
                            cx.notify();
                        }
                        return;
                    }

                    if this.recording_action.is_none() {
                        if crate::app::keybinding_recorder::event_matches_action(
                            &this.config,
                            "PrevTab",
                            ev,
                        ) {
                            this.switch_workspace_tab(-1, window, cx);
                            window.prevent_default();
                            cx.stop_propagation();
                            return;
                        }

                        if crate::app::keybinding_recorder::event_matches_action(
                            &this.config,
                            "NextTab",
                            ev,
                        ) {
                            this.switch_workspace_tab(1, window, cx);
                            window.prevent_default();
                            cx.stop_propagation();
                            return;
                        }
                    }

                    let Some(action) = this.recording_action.clone() else {
                        return;
                    };

                    window.prevent_default();
                    cx.stop_propagation();

                    if ev.keystroke.key == "escape" {
                        this.recording_action = None;
                        cx.notify();
                        return;
                    }

                    let Some(new_key) =
                        crate::app::keybinding_recorder::normalize_recorded_keystroke(ev)
                    else {
                        return;
                    };

                    if let Some((_conflict_id, conflict_label)) =
                        crate::app::keybinding_recorder::find_conflict(
                            &this.config,
                            &action,
                            &new_key,
                        )
                    {
                        let formatted = crate::app::keybinding_recorder::format_keystroke(&new_key);
                        this.recording_action = None;
                        this.keybind_error = Some((
                            action.clone(),
                            t!("keybind_conflict", key = formatted, action = conflict_label)
                                .to_string(),
                        ));
                        cx.notify();
                        return;
                    }

                    this.recording_action = None;
                    this.keybind_error = None;
                    this.config.set_key_binding(&action, &new_key);
                    if let Err(err) = this.config.save() {
                        tracing::error!(
                            component = "config",
                            operation = "save_key_binding",
                            error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                            "Failed to save key binding"
                        );
                    }
                    cx.notify();
                });
            }
        })
        .on_mouse_down_out({
            let view = view.clone();
            move |_, _window, cx| {
                view.update(cx, |this, cx| {
                    if this.recording_action.is_some() {
                        this.recording_action = None;
                        cx.notify();
                    }
                });
            }
        })
        .child(content)
}
