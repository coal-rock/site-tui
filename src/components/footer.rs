use iocraft::prelude::*;

use crate::colors::*;

#[component]
pub fn Footer() -> impl Into<AnyElement<'static>> {
    element! {
        View(
            margin_top: Margin::Auto,
            width: 100pct,
            height: 20,
            border_edges: Edges::Top,
            border_style: BorderStyle::Single,
            border_color: COLOR_GRAY,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
        )
        {
            Text(content: "navigation", color: COLOR_FG, weight: Weight::Bold)
            Text(content: "<- arrow keys or vim binds for moving around ->", color: COLOR_FG)
            Text(content: "<- tab for switching focus from navbar to body ->", color: COLOR_FG)
            Text(content: "<- space or enter to select ->", color: COLOR_FG)
            // View(margin_right: Margin::Auto) {
            //     MixedText(align: TextAlign::Center, contents: vec![
            //         MixedTextContent::new("").color(COLOR_GREEN).weight(Weight::Bold),
            //         MixedTextContent::new("hi").color(COLOR_ORANGE),
            //         MixedTextContent::new(" <-").color(COLOR_GREEN).weight(Weight::Bold),
            //     ])
            // }
        }
    }
}
