pub fn draw_tree(levels: usize) {
    if levels == 0 {
        return;
    }
    let max_width = 2 * levels + 1;

    for level in 1..=levels {
        for row in 1..=level {
            let star_count = 2 * row - 1;
            let stars = "*".repeat(star_count);
            let spaces = if star_count > max_width {
                0
            } else {
                (max_width - star_count) / 2
            };

            println!("{}{}", " ".repeat(spaces), stars);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_tree() {
        draw_tree(5);
    }
}