use ratatui::widgets::{BarChart, Bar, BarGroup};
fn main() {
    let bar1 = Bar::default().value(10);
    let group = BarGroup::default().bars(&[bar1]);
    let _chart = BarChart::default().data(group);
}
