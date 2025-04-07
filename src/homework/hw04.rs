fn main() {
    const HEIGHT: usize = 5;

    for i in 1..=HEIGHT {
        let zirochki = 2 * i - 1;
        let progalini = HEIGHT - i;
        print!("{}{}", " ".repeat(progalini), "*".repeat(zirochki));
        println!();
    }

    for i in (1..HEIGHT).rev() {
        let zirochki = 2 * i - 1;
        let progalini = HEIGHT - i;
        print!("{}{}", " ".repeat(progalini), "*".repeat(zirochki));
        println!();
    }
}