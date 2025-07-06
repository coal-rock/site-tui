use chrono::DateTime;
use iocraft::prelude::*;
use minimad;
use minimad::Compound;

use crate::BlogPostContent;
use crate::colors::*;
use crate::components::Navbar;

#[derive(Props, Default)]
pub struct BlogPostProps {
    pub post: BlogPostContent,
}

#[component]
pub fn BlogPost(mut hooks: Hooks, props: &BlogPostProps) -> impl Into<AnyElement<'static>> {
    let text = minimad::parse_text(&props.post.content, minimad::Options::default());
    eprintln!("{:#?}", text.lines);

    element! {
        View(
            width: 60,
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
                width: 60,
                padding_bottom: 1,
            ) {
                Text(content: props.post.title.clone(), color: COLOR_FG, weight: Weight::Bold)
                Text(content: format!("{}", DateTime::from_timestamp(props.post.timestamp, 0).unwrap().format("%m/%d/%y")), color: COLOR_FG4)
                Text(content: props.post.clone().description, color: COLOR_FG3)
            }

            #(
                text.lines.into_iter().map(|l|
                    match l {
                        minimad::Line::Normal(composite) => match composite.style {
                            minimad::CompositeStyle::Header(_) => element! {
                                View(width: 60) {
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
                                View(width: 60, flex_direction: FlexDirection::Column) {
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
                                View(width: 60, flex_direction: FlexDirection::Column, background_color: COLOR_BG1) {
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
                                View(width: 60, flex_direction: FlexDirection::Column) {
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
                        _ => element! {
                            View() {
                                Text(content: "hi")
                            }
                        },
                    }
                )
            )
            // Text(content: format!("{}", text.lines))
        }
    }
}
