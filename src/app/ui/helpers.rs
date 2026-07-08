use super::*;

impl AxShell {
    pub(super) fn bind_titlebar_drag<E>(this: E, cx: &mut Context<Self>) -> E
    where
        E: gpui::InteractiveElement + gpui::StatefulInteractiveElement,
    {
        this.on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _, _, _| {
                this.should_move_window = true;
            }),
        )
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _, _, _| {
                this.should_move_window = false;
            }),
        )
        .on_mouse_down_out(cx.listener(|this, _, _, _| {
            this.should_move_window = false;
        }))
        .on_mouse_move(cx.listener(|this, _, window, _| {
            if this.should_move_window {
                this.should_move_window = false;
                window.start_window_move();
            }
        }))
    }

    pub(super) fn collapsed_sidebar_abbrev(label: &str) -> String {
        let compact: Vec<char> = label.chars().filter(|ch| !ch.is_whitespace()).collect();
        let Some(first) = compact.first().copied() else {
            return "?".to_string();
        };
        if first > '\u{2E7F}' {
            first.to_string()
        } else {
            compact.into_iter().take(2).collect()
        }
    }

    pub(super) fn render_home_page(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_size(rems(2.333))
                    .font_weight(FontWeight::BOLD)
                    .child("AxShell"),
            )
            .child(
                div()
                    .text_size(rems(1.083))
                    .text_color(cx.theme().muted_foreground)
                    .child(t!("open_local_or_ssh")),
            )
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        Button::new("home-open-local")
                            .primary()
                            .label(t!("local_terminal").to_string())
                            .on_click(cx.listener(|this, _, _, cx| this.open_local(cx))),
                    )
                    .child(
                        Button::new("home-open-session")
                            .ghost()
                            .label(t!("open_session").to_string())
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_selector_dialog(window, cx)
                            })),
                    ),
            )
    }
}
