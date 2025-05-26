fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    permute(&digits, 0, &mut |perm| {
        let m = perm[0] as u32;
        let u = perm[1] as u32;
        let x = perm[2] as u32;
        let a = perm[3] as u32;
        let s = perm[4] as u32;
        let l = perm[5] as u32;
        let o = perm[6] as u32;
        let n = perm[7] as u32;

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            println!("  {}{}{}{}", m, u, x, a);
            println!("×     {}", a);
            println!("--------");
            println!(" {}{}{}{}", s, l, o, n);
            println!();
            count += 1;
        }
    });

    println!("Знайдено рішень: {}", count);
}

fn permute<F>(arr: &[u8], start: usize, f: &mut F)
where
    F: FnMut(&[u8]),
{
    let mut arr = arr.to_owned();
    permute_inner(&mut arr, start, f);
}

fn permute_inner<F>(arr: &mut [u8], start: usize, f: &mut F)
where
    F: FnMut(&[u8]),
{
    if start == arr.len() {
        f(arr);
        return;
    }
    for i in start..arr.len() {
        arr.swap(start, i);
        permute_inner(arr, start + 1, f);
        arr.swap(start, i);
    }
}