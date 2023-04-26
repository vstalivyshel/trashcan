use crate::kak_json::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait GetContent {
    fn get_content(&self) -> String;
}

impl GetContent for Atoms {
    fn get_content(&self) -> String {
        let mut content = String::new();
        self.iter().for_each(|atom| content.push_str(&atom.contents));
        content
    }
}

impl GetContent for Vec<Atoms> {
    fn get_content(&self) -> String {
        let mut content = String::new();
        let _ = self.iter().flat_map(|atoms| atoms.iter()).map(|atom| content.push_str(&atom.contents));
        content
    }
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

