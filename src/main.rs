use iocraft::prelude::*;

mod colors;
mod components;
mod pages;
mod state;

use colors::*;
use pages::{Blog, BlogPost, BlogPostContent, Home};
use state::AppState;

#[derive(Clone)]
pub enum CurrentPage {
    Home,
    Blog,
    BlogPost(BlogPostContent),
}

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);

    let current_page = hooks.use_state(|| CurrentPage::Home);

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
                CurrentPage::BlogPost(post) => {
                    element! {
                        View(
                            width,
                            height,
                            background_color: COLOR_BG,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                        ){
                            BlogPost(post: post.clone())
                        }
                    }
                }
            })
        }
    }
}

fn main() {
    smol::block_on(element!(App).fullscreen()).unwrap();
}
