use iocraft::prelude::*;

use crate::colors::*;

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
