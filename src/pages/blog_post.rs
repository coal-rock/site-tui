use chrono::DateTime;
use iocraft::prelude::*;
use minimad;

use crate::BlogPostContent;
use crate::colors::*;
use crate::components::Navbar;

#[derive(Props, Default)]
pub struct BlogPostProps {
    pub post: BlogPostContent,
}

#[component]
pub fn BlogPost(mut hooks: Hooks, props: &BlogPostProps) -> impl Into<AnyElement<'static>> {
    let mut scroll_offset = hooks.use_state(|| 0i32);

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        scroll_offset.set((scroll_offset.get() - 1).max(0))
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        scroll_offset.set(scroll_offset.get() + 1)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });

    let text = minimad::parse_text(&props.post.content, minimad::Options::default());

    element! {
        View(
            width: 75,
            height: 100pct,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
        ) {
            Navbar(
                should_center: false,
                active: true,
            )

            View(
                border_edges: Edges::Bottom,
                border_style: BorderStyle::Single,
                border_color: COLOR_GRAY,
                width: 100pct,
                height: 1,
            )

            View(
                flex_direction: FlexDirection::Column,
                width: 75,
            ) {
                Text(content: props.post.title.clone(), color: COLOR_FG, weight: Weight::Bold)
                Text(content: format!("{}", DateTime::from_timestamp(props.post.timestamp, 0).unwrap().format("%m/%d/%y")), color: COLOR_FG4)
                Text(content: props.post.clone().description, color: COLOR_FG3)
            }

            View(
                border_edges: Edges::Bottom,
                border_style: BorderStyle::Single,
                border_color: COLOR_GRAY,
                width: 100pct,
                height: 1,
            )

            View(
                height: 90pct,
                width: 100pct,
                overflow: Overflow::Hidden,
            ){
                View(
                    top: -scroll_offset.get(),
                    flex_direction: FlexDirection::Column,
                    position: Position::Absolute,
                )
                {
                    #(
                        text.lines.into_iter().map(|l|
                            match l {
                                minimad::Line::Normal(composite) => match composite.style {
                                    minimad::CompositeStyle::Header(_) => element! {
                                        View(width: 75) {
                                            #(
                                                composite.compounds.into_iter().map(|compound| {
                                                    element! {
                                                        Text(
                                                            content: compound.src,
                                                            weight: Weight::Bold,
                                                            color: COLOR_FG,
                                                        )
                                                    }
                                                })
                                            )
                                        }
                                    },
                                    minimad::CompositeStyle::Paragraph => element! {
                                        View(width: 75, flex_direction: FlexDirection::Column) {
                                            MixedText(contents:
                                                composite.compounds.into_iter().map(|compound| {
                                                    MixedTextContent::new(compound.src)
                                                        .color(COLOR_FG)
                                                        .weight(if compound.bold { Weight::Bold } else { Weight::Normal })
                                                }).collect::<Vec<MixedTextContent>>()
                                            )
                                        }
                                    },
                                    minimad::CompositeStyle::Code => element! {
                                        View(width: 75, flex_direction: FlexDirection::Column, background_color: COLOR_BG1) {
                                            MixedText(contents:
                                                composite.compounds.into_iter().map(|compound| {
                                                    MixedTextContent::new(compound.src)
                                                        .color(COLOR_FG)
                                                        .weight(if compound.bold { Weight::Bold } else { Weight::Normal })
                                                }).collect::<Vec<MixedTextContent>>()
                                            )
                                        }
                                    },
                                    minimad::CompositeStyle::Quote => element! {
                                        View(width: 75, flex_direction: FlexDirection::Column) {
                                            MixedText(contents:
                                                composite.compounds.into_iter().map(|compound| {
                                                    MixedTextContent::new(compound.src)
                                                        .color(COLOR_FG)
                                                        .weight(if compound.bold { Weight::Bold } else { Weight::Normal })
                                                }).collect::<Vec<MixedTextContent>>()
                                            )
                                        }
                                    },
                                    minimad::CompositeStyle::ListItem(_) => todo!(),
                                },
                                minimad::Line::HorizontalRule => {
                                    element! {
                                        View(
                                            border_edges: Edges::Bottom,
                                            border_style: BorderStyle::Single,
                                            border_color: COLOR_GRAY,
                                            width: 100pct,
                                            height: 1,
                                            padding_bottom: 0
                                        )
                                    }
                                }
                                _ => element! {
                                    View() {
                                        Text(content: "hi")
                                    }
                                },
                            }
                        )
                    )
                }

            }
        }
    }
}
