use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BorderStyle {
    vert_symbol: char,
    hori_symbol: char,
    corners: (char, char, char, char),
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            vert_symbol: '│',
            hori_symbol: '─',
            corners: ('┐', '┘', '└', '┌'),
        }
    }
}

impl BorderStyle {
    pub(super) fn get_vert_symbol(&self) -> char {
        self.vert_symbol
    }

    pub(super) fn get_hori_symbol(&self) -> char {
        self.hori_symbol
    }

    pub(super) fn get_corners(&self) -> (char, char, char, char) {
        self.corners
    }
}
