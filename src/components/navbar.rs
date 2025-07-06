use iocraft::prelude::*;
use std::process::exit;

use crate::{AppState, CurrentPage, colors::*};

#[derive(Default, Clone, Props)]
pub struct NavbarProps {
    pub should_center: bool,
    pub active: bool,
}

#[component]
pub fn Navbar(mut hooks: Hooks, props: &NavbarProps) -> impl Into<AnyElement<'static>> {
    let mut state = hooks.use_context_mut::<AppState>();
    let mut focus = hooks.use_state(|| 0);
    let mut should_switch = hooks.use_state(|| false);

    let active = props.active.clone();
    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Enter | KeyCode::Char(' ') => should_switch.set(true),
                KeyCode::Right | KeyCode::Char('l') => {
                    if active {
                        focus.set((focus + 1) % 4)
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if active {
                        focus.set((focus + 3) % 4)
                    }
                }
                _ => {}
            }
        }
        _ => {}
    });

    if should_switch.get() && props.active {
        match focus.get() {
            0 => state.current_page.set(CurrentPage::Home),
            1 => state.current_page.set(CurrentPage::Blog),
            2 => {
                println!("\x1bc\x1b[2J\x1b[H");
                println!("$ xdg-open https://github.com/coal-rock");
                exit(0);
            }
            3 => {
                println!("\x1bc\x1b[2J\x1b[H");
                println!("$ curl https://coal.sh/assets/CV.pdf");
                exit(0);
            }
            _ => {}
        }

        should_switch.set(false);
    }

    element! {
        View(
            flex_direction: FlexDirection::Column,
            align_items: if props.should_center { AlignItems::Center } else { AlignItems::Start }
        ) {
            MixedText(align: TextAlign::Center, contents: vec![
                MixedTextContent::new("site").color(COLOR_GRAY).weight(Weight::Bold).decoration(if focus == 0 && props.active { TextDecoration::Underline } else { TextDecoration::None }),
                MixedTextContent::new("@").color(COLOR_YELLOW).weight(Weight::Bold).decoration(if focus == 0 && props.active { TextDecoration::Underline } else { TextDecoration::None }),
                MixedTextContent::new("coal.sh").color(COLOR_RED_LIGHT).weight(Weight::Bold).decoration(if focus == 0 && props.active { TextDecoration::Underline } else { TextDecoration::None }),
            ])

            View(
            ) {
                Text(content: "[ ", color: COLOR_FG0)
                Button() {
                    Text(content: "blog", color: COLOR_FG0, decoration: if focus == 1 && props.active {TextDecoration::Underline } else { TextDecoration::None } )
                }
                Text(content: " ] ", color: COLOR_FG0)

                Text(content: "[ ", color: COLOR_FG0)
                Button() {
                    Text(content: "projects", color: COLOR_FG0, decoration: if focus == 2 && props.active {TextDecoration::Underline } else { TextDecoration::None } )
                }
                Text(content: " ] ", color: COLOR_FG0)

                Text(content: "[ ", color: COLOR_FG0)
                Button() {
                    Text(content: "resume", color: COLOR_FG0, decoration: if focus == 3 && props.active {TextDecoration::Underline } else { TextDecoration::None } )
                }
                Text(content: " ]", color: COLOR_FG0)
            }
        }
    }
}
