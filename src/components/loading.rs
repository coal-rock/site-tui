use iocraft::prelude::*;

use crate::colors::*;

#[component]
pub fn LoadingIndicator(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut frame = hooks.use_state(|| 0);
    hooks.use_future(async move {
        loop {
            smol::Timer::after(std::time::Duration::from_millis(100)).await;
            frame.set((frame + 1) % FRAMES.len());
        }
    });
    element! {
        View(
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        ) {
            Text(content: FRAMES[frame.get()], color: COLOR_ORANGE)
            Text(content: " Loading...", color: COLOR_FG)
        }
    }
}
