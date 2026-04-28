use ratatui::widgets::{BarChart, Bar, BarGroup}; fn main() { let _b = BarChart::default().data(BarGroup::default().bars(&[Bar::default().value(5)])); }
