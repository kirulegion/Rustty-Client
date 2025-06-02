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

impl Methods {
    pub fn all() -> &'static [Methods] {
        &[
            Methods::GET,
            Methods::POST,
            Methods::PUT,
            Methods::PATCH,
            Methods::DELETE,
        ]
    }

    pub fn next(self) -> Self {
        let list = Self::all();
        let idx = list.iter().position(|&m| m == self).unwrap();
        *list.get((idx + 1) % list.len()).unwrap()
    }

    pub fn prev(self) -> Self {
        let list = Self::all();
        let idx = list.iter().position(|&m| m == self).unwrap();
        *list.get((idx + list.len() - 1) % list.len()).unwrap()
    }
}
