use crate::widget::Widget;

#[derive(Clone)]
pub struct Window {
    widgets: Vec<Widget>,
    selected_widget: usize,
}
