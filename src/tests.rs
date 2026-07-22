use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Metaphone3;

#[test]
fn test_basic_words() {
    let test_cases = vec![
        ("A", "A", ""),
        ("ack", "AK", ""),
        ("eek", "AK", ""),
        ("ache", "AK", "AX"),
    ];

    let mut encoder = Metaphone3::new();

    for (input, expected_primary, expected_secondary) in test_cases {
        let (primary, secondary) = encoder.encode(input);
        assert_eq!(
            primary, expected_primary,
            "Primary mismatch for '{input}': expected '{expected_primary}', got '{primary}'"
        );
        assert_eq!(
            secondary, expected_secondary,
            "Secondary mismatch for '{input}': expected '{expected_secondary}', got '{secondary}'"
        );
    }
}

#[test]
fn test_harness() {
    let mut encoder = Metaphone3::new().with_encode_vowels(true);
    let (primary, _) = encoder.encode("supernode");
    assert_eq!(primary, "SAPARNAT", "Expected 'SAPARNAT', got '{primary}'");
}

#[test]
fn test_aaberg() {
    let mut encoder = Metaphone3::new();
    let a = encoder.encode("Aaberg");
    assert_eq!(a, ("APRK".into(), "".into()));
}

#[test]
fn test_name_files() -> Result<(), Box<dyn std::error::Error>> {
    let testdata_dir = Path::new("testdata");
    if !testdata_dir.exists() {
        eprintln!("Skipping test_name_files: testdata/ directory not found");
        return Ok(());
    }

    for entry in std::fs::read_dir(testdata_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() != Some("test".as_ref()) {
            continue;
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_reader(reader);

        let mut encoder = Metaphone3::new();
        let mut encoder_v = Metaphone3::new().with_encode_vowels(true);
        let mut encoder_e = Metaphone3::new().with_encode_exact(true);
        let mut encoder_ev = Metaphone3::new()
            .with_encode_vowels(true)
            .with_encode_exact(true);

        let mut cnt = 0;
        let mut enc_err = 0;
        let mut enc_v_err = 0;
        let mut enc_e_err = 0;
        let mut enc_ev_err = 0;

        for result in csv_reader.records() {
            let record = result?;
            if record.len() < 9 {
                continue; // skip malformed lines
            }

            let input = &record[0];
            let main_ve = &record[1]; // !v!e
            let alt_ve = &record[2];
            let main_v_ev = &record[3]; // v+e
            let alt_v_ev = &record[4];
            let main_e = &record[5]; // !v+e
            let alt_e = &record[6];
            let main_v = &record[7]; // v!e
            let alt_v = &record[8];

            cnt += 1;

            check_encoding("Enc", &mut encoder, input, main_ve, alt_ve, &mut enc_err);
            check_encoding(
                "EncEV",
                &mut encoder_ev,
                input,
                main_v_ev,
                alt_v_ev,
                &mut enc_ev_err,
            );
            check_encoding("EncE", &mut encoder_e, input, main_e, alt_e, &mut enc_e_err);
            check_encoding("EncV", &mut encoder_v, input, main_v, alt_v, &mut enc_v_err);
        }

        let print_stat = |name: &str, err: i32, total: i32| {
            let percent = (f64::from(err) / f64::from(total)) * 100.0;
            println!("Encoder {name}, error percent: {percent:.2}%");
        };

        print_stat("Enc", enc_err, cnt);
        print_stat("EncEV", enc_ev_err, cnt);
        print_stat("EncE", enc_e_err, cnt);
        print_stat("EncV", enc_v_err, cnt);

        assert!(
            enc_err + enc_ev_err + enc_e_err + enc_v_err == 0,
            "Errors when processing {path:?}: Enc={enc_err} EncEV={enc_ev_err} EncE={enc_e_err} EncV={enc_v_err}"
        );
    }

    Ok(())
}

fn check_encoding(
    name: &str,
    encoder: &mut Metaphone3,
    input: &str,
    expected_primary: &str,
    expected_secondary: &str,
    error_count: &mut i32,
) {
    let (primary, secondary) = encoder.encode(input);

    let mut had_error = false;
    if primary != expected_primary {
        eprintln!(
            "Error Encoding '{input}' with {name}: Primary want '{expected_primary}', got '{primary}'"
        );
        had_error = true;
    }
    if secondary != expected_secondary {
        eprintln!(
            "Error Encoding '{input}' with {name}: Secondary want '{expected_secondary}', got '{secondary}'"
        );
        had_error = true;
    }

    if had_error {
        *error_count += 1;
    }
}
