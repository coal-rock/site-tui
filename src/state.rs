use iocraft::prelude::*;

use crate::CurrentPage;

pub struct AppState {
    pub current_page: State<CurrentPage>,
}
