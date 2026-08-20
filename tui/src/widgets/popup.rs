use ratatui::{layout::{Constraint, Layout, Rect}, widgets::Widget};

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Min(height),
        Constraint::Fill(1),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .split(vertical[1])[1]
}

pub struct Dimensions {
    width: u16,
    height: u16,
}

impl Dimensions{
    pub fn new(width: u16, height:u16) -> Self{
        Self { width, height }
    }
}

pub struct Popup<W, F>
where
    W: Widget,
    F: Fn()
{
    child: W,
    dimension: Dimensions,
    on_submit: F
}

impl<W, F> Popup<W, F>
where 
    W: Widget,
    F: Fn()
{
    pub fn new(child: W, dimension: Dimensions, on_submit: F) -> Self{
        Self { child, dimension, on_submit }
    }
}

impl<W, F> Widget for Popup<W, F>
where 
    W: Widget,
    F: Fn()
{
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    {
        let popup_area = centered_rect(area, self.dimension.width, self.dimension.height);
        self.child.render(popup_area, buf);
    }
}
