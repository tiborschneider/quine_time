use std::{
    fs::*,
    io::{self, *},
    thread::*,
    time::*,
};

fn p(d: &Vec<Vec<u8>>, x: &mut usize, y: &mut usize, c: char) {
    let z = "\x1b[";
    let (a, b) = match d[*y][*x] {
        2 => (1, 92),
        1 => (1, 32),
        _ => (21, 90),
    };
    print!("{}{}m{}{}m{}", z, a, z, b, c);
    *x += 1;
    if c == '\n' {
        *x = 0;
        *y += 1;
    }
}

fn main() -> io::Result<()> {
    let mut rng = File::open("/dev/urandom")?;
    let d = vec![0u8; 256];
    let mut d: Vec<_> = std::iter::repeat(&d).take(20).cloned().collect();
    let mut b = [0];

    loop {
        let mut row = d[0].clone();
        for x in row.iter_mut() {
            if *x > 0 {
                *x -= 1;
            }
        }

        for _ in 0..10 {
            rng.read_exact(&mut b)?;
            row[b[0] as usize] = 2u8;
        }

        d.pop();
        d.insert(0, row);

        print!("\x1b[2J\x1b[1;1H");

        let mut x = 0;
        let mut y = 0;
        for c in S.chars() {
            if Some(c) == char::from_u32(126) {
                for c in S.chars() {
                    if c == '"' || c == '\\' {
                        p(&d, &mut x, &mut y, '\\')
                    }
                    p(&d, &mut x, &mut y, c)
                }
            } else {
                p(&d, &mut x, &mut y, c)
            }
        }
        println!("\x1b[0m");
        sleep(Duration::from_millis(99))
    }

    Ok(())
}

const S: &str = "~";
