// Stolen from https://docs.rs/crate/kak-ui/0.2.0/source/src/lib.rs
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

type Line = Vec<Atom>;


#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
enum RawIncomingRequest {
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

impl std::fmt::Display for IncomingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IncomingRequest::*;
        match self {
            Draw { .. } => write!(f, "Draw"),
            DrawStatus { .. } => write!(f, "DrawStatus"),
            MenuShow { .. } => write!(f, "MenuShow"),
            MenuSelect { .. } => write!(f, "MenuSelect"),
            MenuHide { .. } => write!(f, "MenuHide"),
            InfoShow { .. } => write!(f, "InfoShow"),
            InfoHide { .. } => write!(f, "InfoHide"),
            SetCursor { .. } => write!(f, "SetCursor"),
            SetUiOptions { .. } => write!(f, "SetUiOptions"),
            Refresh { .. } => write!(f, "Refresh"),
        }
    }
}

impl<'de> Deserialize<'de> for IncomingRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(<JsonRpc<RawIncomingRequest>>::deserialize(deserializer)?
            .inner
            .into())
    }
}

impl From<RawIncomingRequest> for IncomingRequest {
    fn from(raw_method: RawIncomingRequest) -> Self {
        type Raw = RawIncomingRequest;
        type NotRaw = IncomingRequest;
        match raw_method {
            Raw::Draw(a, b, c) => NotRaw::Draw {
                lines: a,
                default_face: b,
                padding_face: c,
            },
            Raw::DrawStatus(a, b, c) => NotRaw::DrawStatus {
                status_line: a,
                mode_line: b,
                default_face: c,
            },
            Raw::MenuShow(a, b, c, d, e) => NotRaw::MenuShow {
                items: a,
                anchor: b,
                selected_item_face: c,
                menu_face: d,
                style: e,
            },
            Raw::MenuSelect((a,)) => NotRaw::MenuSelect { selected: a },
            Raw::MenuHide(_) => NotRaw::MenuHide,
            Raw::InfoShow(a, b, c, d, e) => NotRaw::InfoShow {
                title: a,
                content: b,
                anchor: c,
                face: d,
                style: e,
            },
            Raw::InfoHide(_) => NotRaw::InfoHide,
            Raw::SetCursor(a, b) => NotRaw::SetCursor { mode: a, coord: b },
            Raw::SetUiOptions((a,)) => NotRaw::SetUiOptions { options: a },
            Raw::Refresh((a,)) => NotRaw::Refresh { force: a },
        }
    }
}

/// A outgoing request. Input this to kakoune via stdin.
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
enum RawOutgoingRequest {
    Keys(Vec<String>),
    Resize(u32, u32),
    Scroll((u32,)),
    MouseMove(u32, u32),
    MousePress(String, u32, u32),
    MouseRelease(String, u32, u32),
    MenuSelect((u32,)),
}

impl Serialize for OutgoingRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonRpc::new(RawOutgoingRequest::from(self.clone())).serialize(serializer)
    }
}

impl From<OutgoingRequest> for RawOutgoingRequest {
    fn from(request: OutgoingRequest) -> Self {
        type Raw = RawOutgoingRequest;
        type NotRaw = OutgoingRequest;
        match request {
            NotRaw::Keys(vec) => Raw::Keys(vec),
            NotRaw::Resize {
                rows: a,
                columns: b,
            } => Raw::Resize(a, b),
            NotRaw::Scroll { amount: a } => Raw::Scroll((a,)),
            NotRaw::MouseMove { line: a, column: b } => Raw::MouseMove(a, b),
            NotRaw::MousePress {
                button: a,
                line: b,
                column: c,
            } => Raw::MousePress(a, b, c),
            NotRaw::MouseRelease {
                button: a,
                line: b,
                column: c,
            } => Raw::MouseRelease(a, b, c),
            NotRaw::MenuSelect { index: a } => Raw::MenuSelect((a,)),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct JsonRpc<T> {
    jsonrpc: String,
    #[serde(flatten)]
    inner: T,
}

impl<T> JsonRpc<T> {
    fn new(inner: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            inner,
        }
    }
}
