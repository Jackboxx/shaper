use shaper::rect::{draw_rect, RectStyle};

fn main() {
    println!(
        "{}",
        draw_rect(
            "hi there fellow stranger ",
            (20, 10),
            RectStyle::default()
                .set_padding((2, 5, 0, 5))
                .set_margin((3, 3, 2, 4))
        )
    );
}
