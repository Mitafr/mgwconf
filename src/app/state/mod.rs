pub mod configuration;

pub trait State {
    fn next(&mut self);
    fn back(&mut self);

    fn update_pan_len(&mut self);

    fn current_pan(&self) -> usize;
    fn current_tab(&self) -> usize;
    fn current_selected(&self) -> usize;
    fn select_current(&mut self);
    fn unselect_current(&mut self);

    fn is_tab_selected(&self) -> bool;

    fn waiting_for_load(&self) -> bool;
    fn wait_for_load(&mut self);
    fn is_loaded(&self) -> bool;
    fn set_loaded(&mut self);
    fn unset_loaded(&mut self);
}
