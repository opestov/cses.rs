use std::io::{BufRead, BufWriter, Write};

fn line(reader: &mut dyn BufRead) -> String {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let i0 = std::io::stdin();
    let ii = &mut i0.lock();

    let o0 = std::io::stdout();
    let oo = &mut BufWriter::new(o0.lock());

    let mut t: u32 = line(ii).parse().unwrap();
    while t > 0 {
        let v: Vec<i64> = line(ii).split(' ').map(|x| x.parse().unwrap()).collect();
        let (x1, y1, x2, y2, x3, y3) = (v[0], v[1], v[2], v[3], v[4], v[5]);

        let s = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
        let m = if s > 0 {
            "LEFT"
        } else if s < 0 {
            "RIGHT"
        } else {
            "TOUCH"
        };
        writeln!(oo, "{}", m).unwrap();

        t -= 1;
    }
}
