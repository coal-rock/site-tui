use iocraft::prelude::*;
use std::{process::exit, time::Duration};

mod colors;
use colors::*;

#[component]
fn Navbar(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut state = hooks.use_context_mut::<AppState>();
    let mut focus = hooks.use_state(|| 0);
    let mut should_switch = hooks.use_state(|| false);

    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Enter | KeyCode::Char(' ') => should_switch.set(true),
                KeyCode::Right | KeyCode::Tab | KeyCode::Char('l') => focus.set((focus + 1) % 4),
                KeyCode::Left | KeyCode::BackTab | KeyCode::Char('h') => focus.set((focus + 3) % 4),
                _ => {}
            }
        }
        _ => {}
    });

    if should_switch.get() {
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
        ) {
            MixedText(align: TextAlign::Center, contents: vec![
                MixedTextContent::new("site").color(COLOR_GRAY).weight(Weight::Bold).decoration(if focus == 0 { TextDecoration::Underline } else { TextDecoration::None }),
                MixedTextContent::new("@").color(COLOR_YELLOW).weight(Weight::Bold).decoration(if focus == 0 { TextDecoration::Underline } else { TextDecoration::None }),
                MixedTextContent::new("coal.sh").color(COLOR_RED_LIGHT).weight(Weight::Bold).decoration(if focus == 0 { TextDecoration::Underline } else { TextDecoration::None }),
            ])

            View(
            ) {
                Text(content: "[ ", color: COLOR_FG0)
                Button() {
                    Text(content: "blog", color: COLOR_FG0, decoration: if focus == 1 {TextDecoration::Underline } else { TextDecoration::None } )
                }
                Text(content: " ] ", color: COLOR_FG0)

                Text(content: "[ ", color: COLOR_FG0)
                Button() {
                    Text(content: "projects", color: COLOR_FG0, decoration: if focus == 2 {TextDecoration::Underline } else { TextDecoration::None } )
                }
                Text(content: " ] ", color: COLOR_FG0)

                Text(content: "[ ", color: COLOR_FG0)
                Button() {
                    Text(content: "resume", color: COLOR_FG0, decoration: if focus == 3 {TextDecoration::Underline } else { TextDecoration::None } )
                }
                Text(content: " ]", color: COLOR_FG0)
            }
        }
    }
}

#[derive(Copy, Clone)]
enum CurrentPage {
    Home,
    Blog,
}

struct AppState {
    current_page: State<CurrentPage>,
}

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);

    let current_page = hooks.use_state(|| CurrentPage::Home);
    let (stdout, stderr) = hooks.use_output();

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('q') => should_exit.set(true),
                    _ => {}
                }
            }
            _ => {}
        }
    });

    if should_exit.get() {
        system.exit();
    }

    element! {
        ContextProvider(
            value: Context::owned(AppState { current_page })
        ) {
            #(match &*current_page.read() {
                CurrentPage::Home => {
                    element! {
                        View(
                            width,
                            height,
                            background_color: COLOR_BG,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                        ){
                            Home()
                        }
                    }
                }
                CurrentPage::Blog => {
                    element! {
                        View(
                            width,
                            height,
                            background_color: COLOR_BG,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                        ){
                            Blog()
                        }
                    }
                }
            })
        }
    }
}

#[component]
fn Home(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            width: 50,
            height: 30,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            gap: 1,
        ) {
            Navbar(
            ){}

            MixedText(contents: vec![
                MixedTextContent::new("hey, i'm ").color(COLOR_FG),
                MixedTextContent::new("coal").color(COLOR_RED_LIGHT),
                MixedTextContent::new(", a hobbyist programmer from upstate ny. my interests include reverse engineering, compiler design, experimental filesystems, and penetration testing.").color(COLOR_FG),
            ])

            Card(
                title: "about me",
                fields: vec![
                    "rust, lua, python, c",
                    "linux (archbtw)",
                    "neovim evangelist",
                    "avid llm hater",
                    "foss enthusiast",
                    "gruvbox lover"
                ],
            )

            Card(
                title: "where to find me",
                fields: vec![
                    "discord: @coalrock",
                    "github: @coal-rock",
                    "email: coal320@proton.me",
                ],
            )
        }
    }
}

#[derive(Default, Props, Clone)]
pub struct CardProps {
    pub title: &'static str,
    pub fields: Vec<&'static str>,
}

#[component]
pub fn Blog() -> impl Into<AnyElement<'static>> {
    element! {
        View()
    }
}

#[component]
pub fn Card(props: &CardProps) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            width: 50,
            border_style: BorderStyle::Single,
            border_color: COLOR_FG,
            padding: 0,
            margin: 0,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
        ) {
            View(
                width: 48,
                border_edges: Edges::Bottom,
                border_style: BorderStyle::Single,
                border_color: COLOR_FG,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            )
            {
                Text(align: TextAlign::Center, content: format!("{}", props.title), color: COLOR_FG, italic: true)
            }

            #(props.fields.clone().into_iter().map(|field| element! {
                MixedText(align: TextAlign::Center, contents: vec![
                    MixedTextContent::new("-> ").color(COLOR_GREEN).weight(Weight::Bold),
                    MixedTextContent::new(field).color(COLOR_ORANGE),
                    MixedTextContent::new(" <-").color(COLOR_GREEN).weight(Weight::Bold),
                ])
            }))
        }
    }
}

fn main() {
    smol::block_on(element!(App).fullscreen()).unwrap();
    println!("hi")
}
