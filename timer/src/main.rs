use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};

mod elements;
mod models;
mod views;
use views::timer::TimerView;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, app_cx| app_cx.new(|view_cx| TimerView::new(view_cx)),
        )
        .unwrap();
    });
}
