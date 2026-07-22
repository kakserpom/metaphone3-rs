use metaphone3::Metaphone3;
use std::time::Instant;

fn main() {
    let text = std::fs::read_to_string("testdata/surnames-us.txt").unwrap();
    let words: Vec<&str> = text.lines().map(|l| l.split(',').next().unwrap()).collect();
    println!("words: {}", words.len());

    let mut encoder = Metaphone3::new();
    // warmup
    let mut sink = 0usize;
    for w in &words {
        let (p, s) = encoder.encode(w);
        sink = sink.wrapping_add(p.len() + s.len());
    }

    let reps = 20;
    let start = Instant::now();
    for _ in 0..reps {
        for w in &words {
            let (p, s) = encoder.encode(w);
            sink = sink.wrapping_add(p.len() + s.len());
        }
    }
    let elapsed = start.elapsed();
    let total = words.len() * reps;
    println!("sink={}", sink);
    println!(
        "encoded {} words in {:?} ({:.0} ns/word, {:.2} M words/s)",
        total,
        elapsed,
        elapsed.as_nanos() as f64 / total as f64,
        total as f64 / elapsed.as_secs_f64() / 1e6
    );
}
