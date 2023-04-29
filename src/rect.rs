use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::border::BorderStyle;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RectStyle {
    padding: (u16, u16, u16, u16),
    margin: (u16, u16, u16, u16),
}

impl RectStyle {
    pub fn set_padding(mut self, padding: (u16, u16, u16, u16)) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_margin(mut self, margin: (u16, u16, u16, u16)) -> Self {
        self.margin = margin;
        self
    }
}

pub fn surround_with_spacing<D: Display>(content: D, spacing: (u16, u16, u16, u16)) -> String {
    let (t, r, b, l) = spacing;
    let top = "\n".repeat(t.into());
    let bottom = "\n".repeat(b.into());

    let content = content
        .to_string()
        .split("\n")
        .map(|line| {
            format!(
                "{lp}{l}{rp}",
                l = line.trim(),
                lp = " ".repeat(l.into()),
                rp = " ".repeat(r.into())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    top + &content + &bottom
}

pub fn surrond_with_border<D: Display>(content: D, style: BorderStyle) -> String {
    let len = content
        .to_string()
        .split("\n")
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap_or("")
        .len();

    let vert_symbol = style.get_vert_symbol();
    let hori_symbol = style.get_hori_symbol();
    let corners = style.get_corners();

    let top = format!(
        "{l}{m}{r}",
        l = corners.3,
        m = hori_symbol.to_string().repeat(len),
        r = corners.0
    );

    let bottom = format!(
        "{l}{m}{r}",
        l = corners.2,
        m = hori_symbol.to_string().repeat(len),
        r = corners.1
    );

    let content = content
        .to_string()
        .split("\n")
        .map(|line| format!("{vb}{line:len$}{vb}", vb = vert_symbol))
        .collect::<Vec<String>>()
        .join("\n");

    top + "\n" + &content + "\n" + &bottom
}

pub fn draw_rect<D: Display>(content: D, size: (u16, u16), style: RectStyle) -> String {
    let (x, y) = size;
    let (t_pad, r_pad, b_pad, l_pad) = style.padding;

    let w = (x - (l_pad + r_pad + 2)).into();
    let h = y - (t_pad + b_pad + 2);

    let pad_line = format!("{}\n", " ".repeat((x - 2).into()));
    let content_padding = vec![pad_line.clone(); h.into()];

    let content = content
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .chunks(w)
        .map(|c| format!("{c:<w$}\n", c = c.iter().collect::<String>(),))
        .chain(content_padding)
        .take(h.into())
        .collect::<Vec<String>>()
        .join("");

    let rect_content = surround_with_spacing(content, style.padding);
    let rect = surrond_with_border(rect_content, BorderStyle::default());

    surround_with_spacing(rect, style.margin)
}
