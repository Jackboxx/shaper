use shaper::rect::draw_rect;

fn main() {
    println!(
        "{}",
        draw_rect("hi there fellow stranger", (20, 10), (2, 1, 0, 2))
    );
}
