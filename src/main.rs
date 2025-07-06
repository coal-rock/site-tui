use iocraft::prelude::*;

mod colors;
use colors::*;

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);

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
        View(
            width,
            height,
            background_color: COLOR_BG,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        ) {
            View(
                width: 50,
                height: 30,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                gap: 1,
            ) {
                MixedText(align: TextAlign::Center, contents: vec![
                    MixedTextContent::new("site").color(COLOR_GRAY).weight(Weight::Bold),
                    MixedTextContent::new("@").color(COLOR_YELLOW).weight(Weight::Bold),
                    MixedTextContent::new("coal.sh").color(COLOR_RED_LIGHT).weight(Weight::Bold),
                ])

                Text(align: TextAlign::Center, content: "[ blog ] [ projects ] [ resume ]", color: COLOR_FG0)

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
}

#[derive(Default, Props, Clone)]
pub struct CardProps {
    pub title: &'static str,
    pub fields: Vec<&'static str>,
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
}
