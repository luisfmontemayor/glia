use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (count, c) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_with_commas_variables() {
        let cases = vec![
            (0, "0"),
            (999, "999"),
            (1000, "1,000"),
            (1_234_567, "1,234,567"),
            (12_345_678_901_234, "12,345,678,901,234"),
        ];

        for (input, expected) in cases {
            assert_eq!(format_with_commas(input), expected);
        }
    }

    #[test]
    fn test_centered_rect_variables() {
        let area = Rect::new(0, 0, 100, 100);
        
        let percent_x = 50;
        let percent_y = 50;
        let result = centered_rect(percent_x, percent_y, area);
        
        assert_eq!(result.x, 25);
        assert_eq!(result.y, 25);
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 50);

        let area2 = Rect::new(10, 10, 200, 50);
        let px2 = 10;
        let py2 = 80;
        let res2 = centered_rect(px2, py2, area2);
        
        assert_eq!(res2.x, 10 + 90);
        assert_eq!(res2.y, 10 + 5);
        assert_eq!(res2.width, 20);
        assert_eq!(res2.height, 40);
    }
}
