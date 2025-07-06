use iocraft::prelude::*;

use crate::colors::*;
use crate::components::{Card, Navbar};

#[component]
pub fn Home() -> impl Into<AnyElement<'static>> {
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
                should_center: true,
                active: true,
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
