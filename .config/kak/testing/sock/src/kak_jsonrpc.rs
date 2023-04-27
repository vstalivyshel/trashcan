use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct Face {
    pub attributes: Vec<String>,
    pub bg: String,
    pub fg: String,
    pub underline: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Atom {
    pub contents: String,
    pub face: Face,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Coord {
    pub column: u64,
    pub line: u64,
}

pub type Line = Vec<Atom>;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum RawIncomingRequest {
    Draw(Vec<Line>, Face, Face),
    DrawStatus(Line, Line, Face),
    MenuShow(Vec<Line>, Coord, Face, Face, String),
    MenuSelect((u32,)),
    MenuHide([(); 0]),
    InfoShow(Line, Vec<Line>, Coord, Face, String),
    InfoHide([(); 0]),
    SetCursor(String, Coord),
    SetUiOptions((HashMap<String, String>,)),
    Refresh((bool,)),
}

#[derive(Clone, Debug)]
pub enum IncomingRequest {
    Draw {
        lines: Vec<Line>,
        default_face: Face,
        padding_face: Face,
    },
    DrawStatus {
        status_line: Line,
        mode_line: Line,
        default_face: Face,
    },
    MenuShow {
        items: Vec<Line>,
        anchor: Coord,
        selected_item_face: Face,
        menu_face: Face,
        style: String,
    },
    MenuSelect {
        selected: u32,
    },
    MenuHide,
    InfoShow {
        title: Line,
        content: Vec<Line>,
        anchor: Coord,
        face: Face,
        style: String,
    },
    InfoHide,
    SetCursor {
        mode: String,
        coord: Coord,
    },
    SetUiOptions {
        options: HashMap<String, String>,
    },
    Refresh {
        force: bool,
    },
}

#[derive(Debug, Clone)]
pub enum OutgoingRequest {
    Keys(Vec<String>),
    Resize {
        rows: u32,
        columns: u32,
    },
    Scroll {
        amount: u32,
    },
    MouseMove {
        line: u32,
        column: u32,
    },
    MousePress {
        button: String,
        line: u32,
        column: u32,
    },
    MouseRelease {
        button: String,
        line: u32,
        column: u32,
    },
    MenuSelect {
        index: u32,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum RawOutgoingRequest {
    Keys(Vec<String>),
    Resize(u32, u32),
    Scroll((u32,)),
    MouseMove(u32, u32),
    MousePress(String, u32, u32),
    MouseRelease(String, u32, u32),
    MenuSelect((u32,)),
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpc<T> {
    pub jsonrpc: String,
    #[serde(flatten)]
    pub inner: T,
}

impl<T> JsonRpc<T> {
    pub fn new(inner: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            inner,
        }
    }
}
