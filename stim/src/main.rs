use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};

mod model;
mod view;

use view::TimerView;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(300.), px(300.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|view_cx| TimerView::new(view_cx)),
        )
        .expect("Failed to open main window");
    });
}
