use chrono::DateTime;
use iocraft::prelude::*;
use serde::Deserialize;
use smol::Timer;
use std::time::Duration;

use crate::AppState;
use crate::CurrentPage;
use crate::colors::*;
use crate::components::{LoadingIndicator, Navbar};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct BlogPostContent {
    pub title: String,
    pub id: String,
    pub description: String,
    pub content: String,
    pub timestamp: i64,
}

impl BlogPostContent {
    async fn fetch() -> Vec<BlogPostContent> {
        Timer::after(Duration::from_secs(1)).await;

        surf::get("http://localhost:8000/api/get_all_blogs")
            .recv_json()
            .await
            .unwrap()
    }
}

enum BlogState {
    Init,
    Loading,
    Loaded(Vec<BlogPostContent>),
}

#[derive(Clone, Copy)]
pub enum PageFocus {
    Navbar,
    Content,
}

#[component]
pub fn Blog(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut state = hooks.use_context_mut::<AppState>();
    let mut blog_state = hooks.use_state(|| BlogState::Init);
    let mut page_focus = hooks.use_state(|| PageFocus::Navbar);
    let mut focus_index = hooks.use_state(|| 0);
    let mut should_switch = hooks.use_state(|| false);

    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Tab | KeyCode::BackTab => match page_focus.get() {
                    PageFocus::Navbar => page_focus.set(PageFocus::Content),
                    PageFocus::Content => page_focus.set(PageFocus::Navbar),
                },
                KeyCode::Char('j')
                | KeyCode::Char('k')
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::Enter
                | KeyCode::Char(' ') => {
                    if matches!(*page_focus.read(), PageFocus::Navbar) {
                        return;
                    }

                    let posts = match &*blog_state.read() {
                        BlogState::Init => return,
                        BlogState::Loading => return,
                        BlogState::Loaded(blog_posts) => blog_posts,
                    }
                    .clone();

                    match code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            focus_index.set((focus_index + 1) % posts.len())
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            focus_index.set((focus_index + posts.len() - 1) % posts.len())
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            should_switch.set(true);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    });

    let mut load = hooks.use_async_handler(move |_: ()| async move {
        blog_state.set(BlogState::Loading);
        blog_state.set(BlogState::Loaded(BlogPostContent::fetch().await));
    });

    if matches!(*blog_state.read(), BlogState::Init) {
        load(());
    }

    if should_switch.get() {
        let posts = blog_state.read();

        let posts = match &*posts {
            BlogState::Init => None,
            BlogState::Loading => None,
            BlogState::Loaded(blog_posts) => Some(blog_posts),
        }
        .unwrap();

        let post = posts.get(focus_index.get()).unwrap();
        state.current_page.set(CurrentPage::BlogPost(post.clone()));
        should_switch.set(false);
    }

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
                active: matches!(*page_focus.read(), PageFocus::Navbar),
            )

            View(
                border_edges: Edges::Bottom,
                border_style: BorderStyle::Single,
                border_color: COLOR_GRAY,
                width: 100pct,
                height: 1,
            )

            #(
                match &*blog_state.read() {
                    BlogState::Init | BlogState::Loading => {
                        element! {
                            View(
                                width: 100pct,
                                justify_content: JustifyContent::Center,
                                ) {
                                LoadingIndicator()
                            }
                        }
                    }
                    BlogState::Loaded(posts) => {
                        element! {
                            View(width: 100pct, flex_direction: FlexDirection::Column, gap: 1) {
                                #(posts.clone().into_iter().enumerate().map(|(idx, post)| element! {
                                        View(
                                            flex_direction: FlexDirection::Column,
                                            width: 100pct,
                                            overflow: Overflow::Scroll,
                                        ) {
                                            Text(content: post.title.clone(), color: COLOR_FG, weight: Weight::Bold, decoration: if idx == focus_index.get() && matches!(*page_focus.read(), PageFocus::Content) {TextDecoration::Underline} else {TextDecoration::None})
                                            Text(content: format!("{}", DateTime::from_timestamp(post.timestamp, 0).unwrap().format("%m/%d/%y")), color: COLOR_FG4)
                                            Text(content: post.description, color: COLOR_FG3)
                                        }
                                    })
                                )
                            }
                        }
                    }
                }
            )
        }
    }
}
