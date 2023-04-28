use std::fmt::Display;

pub fn draw_rect<D: Display>(
    content: D,
    size: (u16, u16),
    padding: (u16, u16, u16, u16),
) -> String {
    let (x, y) = size;
    let (t_pad, r_pad, b_pad, l_pad) = padding;

    let w = (x - (l_pad + r_pad + 2)).into();
    let h = y - (t_pad + b_pad + 2);

    let pad_line = format!("│{}│\n", vec![" "; (x - 2).into()].join(""));
    let top = format!("┌{}┐\n", vec!["─"; (x - 2).into()].join(""));
    let bottom = format!("└{}┘", vec!["─"; (x - 2).into()].join(""));

    let content_padding = vec![pad_line.clone(); h.into()];

    let content = content
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .chunks(w)
        .map(|c| {
            format!(
                "│{l}{c:<w$}{r}│\n",
                l = vec![" "; l_pad.into()].join(""),
                c = c.iter().collect::<String>(),
                r = vec![" "; r_pad.into()].join(""),
            )
        })
        .chain(content_padding)
        .take((h).into())
        .collect::<Vec<String>>()
        .join("");

    top + &vec![pad_line.clone(); t_pad.into()].join("")
        + &content
        + &vec![pad_line; b_pad.into()].join("")
        + &bottom
}
