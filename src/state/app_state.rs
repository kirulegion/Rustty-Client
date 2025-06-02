#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focused {
    Normal,
    Url,
    Workspace,
    Feature,
    Method,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Methods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

pub struct App_state {
    pub is_running: bool,
    pub url_input: String,
    pub method: Methods,
    pub mehtod_id: i8,
    pub focused: Focused,
}

impl App_state {
    pub fn new() -> Self {
        Self {
            url_input: String::new(),
            is_running: true,
            method: Methods::GET,
            focused: Focused::Normal,
            mehtod_id: 0,
        }
    }
}
