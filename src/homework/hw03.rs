fn main() {
    const WIDTH: usize = 28;
    const HEIGHT: usize = 10;

    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                output.push('*');
            }
            else if (x as f32 - (y as f32 * (WIDTH as f32 - 1.0) / (HEIGHT as f32 - 1.0))).abs() < 0.5 {
                output.push('*');
            }
            else if (x as f32 - ((HEIGHT as f32 - 1.0 - y as f32) * (WIDTH as f32 - 1.0) / (HEIGHT as f32 - 1.0))).abs() < 0.5 {
                output.push('*');
            }
            else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    print!("{}", output);
}