# metaphone3

A pure Rust implementation of the Metaphone 3 phonetic encoding algorithm.

Metaphone 3 is a more accurate version of the original Soundex algorithm, designed so that similar-sounding words in American English share the same keys. This makes it useful for fuzzy matching, searching names, and comparing words phonetically.

## Features

- **Pure Rust**: No FFI or external dependencies required (only `smartstring` for efficient string handling)
- **Primary and Secondary Encodings**: Generates both primary and alternate phonetic keys for better matching
- **Vowel Encoding**: Optional encoding of vowel sounds for finer phonetic distinction
- **Exact Mode**: Optional stricter encoding that differentiates similar sounds (e.g., hard "G" vs hard "K")
- **Reusable Encoder**: Designed to minimize allocations when encoding multiple words
- **Builder Pattern**: Fluent API for configuration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
metaphone3 = "0.1.1"
```

## Usage

### Basic Usage

```rust
use metaphone3::Metaphone3;

fn main() {
    let mut encoder = Metaphone3::new();

    let (primary, secondary) = encoder.encode("Smith");
    println!("Primary: {}, Secondary: {}", primary, secondary);
    // Output: Primary: SM0, Secondary: XMT
}
```

### With Options

```rust
use metaphone3::Metaphone3;

fn main() {
    // Enable vowel encoding
    let mut encoder = Metaphone3::new()
        .with_encode_vowels(true);

    let (primary, _) = encoder.encode("beautiful");
    println!("{}", primary);  // Output: PATAFAL

    // Enable exact encoding for stricter matching
    let mut encoder = Metaphone3::new()
        .with_encode_exact(true);

    let (primary, secondary) = encoder.encode("edge");
    println!("Primary: {}, Secondary: {}", primary, secondary);
}
```

### Reusing the Encoder

The encoder is designed to be reused across multiple encode calls to reduce memory allocations:

```rust
use metaphone3::Metaphone3;

fn main() {
    let mut encoder = Metaphone3::new();

    let words = ["Smith", "Smyth", "Smithe", "Smythe", "Schmidt"];

    for word in words {
        let (primary, secondary) = encoder.encode(word);
        println!("{}: {} / {}", word, primary, secondary);
    }
}
```

Output:
```
Smith: SM0 / XMT
Smyth: SM0 / XMT
Smithe: SM0 / XMT
Smythe: SM0 / XMT
Schmidt: XMT /
```

## API Reference

### `Metaphone3`

The main encoder struct.

#### Methods

| Method | Description |
|--------|-------------|
| `new() -> Self` | Creates a new encoder with default settings |
| `with_encode_vowels(self, bool) -> Self` | Enables/disables vowel encoding |
| `with_encode_exact(self, bool) -> Self` | Enables/disables exact encoding mode |
| `encode(&mut self, &str) -> (String, String)` | Encodes a word, returning (primary, secondary) keys |

### Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `encode_vowels` | `false` | When `true`, includes non-initial vowel sounds in the output |
| `encode_exact` | `false` | When `true`, produces stricter encodings that differentiate similar sounds |

### Output

The `encode()` method returns a tuple of two strings:
- **Primary**: The main phonetic encoding (always present for non-empty input)
- **Secondary**: An alternate encoding when the word has ambiguous pronunciation (empty string if none)

Both encodings are limited to 8 characters maximum.

## Matching Strategy

For best results when searching for phonetic matches:
1. Encode your search term and target words
2. Match where either primary or secondary keys match

```rust
use metaphone3::Metaphone3;

fn phonetic_match(encoder: &mut Metaphone3, word1: &str, word2: &str) -> bool {
    let (p1, s1) = encoder.encode(word1);
    let (p2, s2) = encoder.encode(word2);

    p1 == p2 ||
    (!s1.is_empty() && s1 == p2) ||
    (!s2.is_empty() && p1 == s2) ||
    (!s1.is_empty() && !s2.is_empty() && s1 == s2)
}
```

## Examples

| Word | Primary | Secondary |
|------|---------|-----------|
| Smith | SM0 | XMT |
| phonetics | FNTKS | |
| Xavier | SFR | |
| edge | AJ | |
| gnome | NM | |
| Thompson | TMPSN | |
| Aachen | AKN | AXN |
| Wroclaw | RKL | |

## Performance

The encoder is optimized to run allocation-free on the hot path: candidate
comparisons match directly against the input buffer (no per-comparison heap
allocation), the input buffer's capacity is reused across `encode()` calls, and
the ≤8-character output keys stay inline via `smartstring` instead of touching
the heap.

Benchmarked on the bundled US surname corpus (~88k words), single-threaded,
reusing one encoder:

| Version | Throughput | Latency |
|---------|------------|---------|
| **v0.1.1** | ~2.8 M words/s | ~357 ns/word |
| v0.1.0 | ~0.35 M words/s | ~2893 ns/word |

That's roughly an **8× speedup** with byte-identical output. Numbers are from an
Apple Silicon laptop (`--release`, LTO enabled); your results will vary with
hardware and input distribution.

Reproduce with the included benchmark:

```sh
cargo run --release --example bench
```

## Algorithm Background

Metaphone 3 was developed by Lawrence Philips as an improvement over the original Metaphone and Double Metaphone algorithms. It provides:

- More accurate encoding of English words and names
- Better handling of non-English origin names common in America
- Support for both primary and alternate pronunciations
- Improved consonant and vowel sound mappings

For more information about the Metaphone algorithm family, see the [Wikipedia article](https://en.wikipedia.org/wiki/Metaphone).

## Thread Safety

The `Metaphone3` encoder is **not** thread-safe. Each thread should use its own encoder instance. The encoder is designed to be cheap to construct, so creating one per thread is recommended.

## References

- Original Metaphone 3 implementation: [OpenRefine Metaphone3.java](https://github.com/OpenRefine/OpenRefine/blob/master/main/src/com/google/refine/clustering/binning/Metaphone3.java)
- Go implementation this port is based on: [dlclark/metaphone3](https://github.com/dlclark/metaphone3)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
