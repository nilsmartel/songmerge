use std::io::Read;

fn main() {
    let width = std::env::args()
        .nth(1)
        .expect("first argument to be width of text")
        .parse::<usize>()
        .expect("width to be a positive integer");

    let cols = std::env::args()
        .nth(2)
        .map_or(Ok(2usize), |s| s.parse())
        .expect("width to be a positive integer");

    let mut content = String::new();
    std::io::stdin()
        .read_to_string(&mut content)
        .expect("read stdin");

    let linelen = width / cols;

    let mut lines = content
        .lines()
        .flat_map(|l| {
            if l.chars().count() > linelen {
                let mut v = Vec::new();
                let mut r = l;
                while !r.is_empty() {
                    let s: String = r.chars().take(linelen).collect();
                    r = &r[s.as_bytes().len()..];
                    v.push(s);
                }
                v
            } else {
                vec![l.to_string()]
            }
        })
        .collect::<Vec<_>>();

    while lines.len() % cols != 0 {
        lines.push(String::new());
    }

    let n = lines.len() / cols;

    for i in 0..n {
        for col in 0..cols {
            let index = i + col * cols;
            let f = &lines[index];
            print!("{f}");

            if col == cols - 1 {
                continue;
            }

            // fill up with whitespace
            for _ in 0..(linelen - f.chars().count()) {
                print!(" ");
            }
        }
        println!();
    }
}
