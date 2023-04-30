use bitflags::bitflags;

bitflags! {
#[derive(Debug, PartialEq, Eq, Clone, )]
    pub struct BorderEdges: u8 {
        const NONE  = 0b0000;
        const TOP   = 0b1000;
        const RIGHT = 0b0100;
        const BOTTOM = 0b0010;
        const LEFT = 0b0001;
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BorderStyle {
    vert_symbol: char,
    hori_symbol: char,
    corners: (char, char, char, char),
    edges: BorderEdges,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            vert_symbol: '│',
            hori_symbol: '─',
            corners: ('┐', '┘', '└', '┌'),
            edges: BorderEdges::ALL,
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
