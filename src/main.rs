use untitled11::Metaphone3;

// --- Main function for demonstration ---
fn main() {
    let mut encoder = Metaphone3::new();

    let tests = vec![
        "Smith",
        "phonetics",
        "Xavier",
        "edge",
        "gnome",
        "Thompson",
        "Aachen",
        "Wroclaw",
    ];

    println!("{:<15} | {:<15} | {:<15}", "Word", "Primary", "Secondary");
    println!("{:-<15}-+-{:-<15}-+-{:-<15}", "", "", "");

    for word in tests {
        let (primary, secondary) = encoder.encode(word);
        println!("{:<15} | {:<15} | {:<15}", word, primary, secondary);
    }

    println!("\n--- With Vowel Encoding ---");
    let mut vowel_encoder = Metaphone3::new().with_encode_vowels(true);
    let (p, s) = vowel_encoder.encode("beautiful");
    println!("{:<15} | {:<15} | {:<15}", "beautiful", p, s);
}
