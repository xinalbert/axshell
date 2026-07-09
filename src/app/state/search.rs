use gpui::{Bounds, Entity, Pixels};
use gpui_component::input::InputState;

pub(crate) struct SearchState {
    pub(crate) input: Entity<InputState>,
    pub(crate) active: bool,
    pub(crate) query: String,
    pub(crate) matches: Vec<(i32, i32)>,
    pub(crate) current: usize,
    pub(crate) target_tab: Option<String>,
    pub(crate) bar_bounds: Option<Bounds<Pixels>>,
}
