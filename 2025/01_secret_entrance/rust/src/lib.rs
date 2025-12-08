use std::path::Path;

/// Parse a document that contains one instruction per line.
///
/// Each line starts with `L` (left) or `R` (right) followed by an integer.
/// `L` means a negative move, `R` a positive move.
///
/// Returns a `Vec<i32>` with the signed moves.
pub fn parse_doc(doc: &str) -> Vec<i32> {
    let mut moves = Vec::new();

    for line in doc.lines() {
        // Skip empty lines (the original Python would treat them as an error,
        // but being tolerant here is harmless.)
        if line.is_empty() {
            continue;
        }

        let dir = if line.starts_with('L') { -1 } else { 1 };
        // `line[1..]` in Python → slice from the second character onward.
        let amount: i32 = line[1..]
            .trim()
            .parse()
            .expect("failed to parse integer after L/R");
        moves.push(dir * amount);
    }

    moves
}

/// First counting method – “jump” style.
///
/// The dial has 100 positions (0‑99). We start at position 50 and add each
/// move, wrapping around with `% 100`. Every time we land exactly on `0`,
/// we increment the counter.
pub fn count_zeros(moves: &[i32]) -> usize {
    let mut pos: i32 = 50;
    let mut zero_count: usize = 0;

    for &mv in moves {
        pos = (pos + mv).rem_euclid(100); // same as Python's `% 100` for negatives
        if pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

/// Wrapper that reads a file and applies `count_zeros`.
pub fn calculate_code<P: AsRef<Path>>(file: P) -> usize {
    let text = std::fs::read_to_string(file).expect("cannot read input file");
    let moves = parse_doc(&text);
    count_zeros(&moves)
}

/// Second counting method – “step‑by‑step” style.
///
/// For each move we walk one position at a time, handling the wrap‑around
/// manually (exactly like the Python version). Every time we step onto `0`,
/// we increase the counter.
pub fn count_zeros2(moves: &[i32]) -> usize {
    let mut pos: i32 = 50;
    let mut zero_count: usize = 0;

    for &mv in moves {
        // The original Python asserts that `mv != 0`. We do the same.
        assert!(mv != 0, "move of size 0 is not allowed");

        // Walk `abs(mv)` steps, one by one.
        for _ in 0..mv.abs() {
            if mv > 0 {
                pos += 1;
                if pos == 100 {
                    pos = 0;
                }
            } else {
                pos -= 1;
                if pos == -1 {
                    pos = 99;
                }
            }

            if pos == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

/// Wrapper that reads a file and applies `count_zeros2`.
pub fn calculate_code2<P: AsRef<Path>>(file: P) -> usize {
    let text = std::fs::read_to_string(file).expect("cannot read input file");
    let moves = parse_doc(&text);
    count_zeros2(&moves)
}

/* -------------------------------------------------------------------------- */
/*                               Unit tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Helper to locate the test files relative to the repository root.
    fn test_path(rel: &str) -> PathBuf {
        // `CARGO_MANIFEST_DIR` points to the directory containing Cargo.toml.
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push(rel);
        p
    }

    #[test]
    fn test_1() {
        // Equivalent of `calculate_code(Path("../test_input1.txt"))`
        let path = test_path("../test_input1.txt");
        assert_eq!(3, calculate_code(path));
    }

    #[test]
    fn test_2() {
        let path = test_path("../test_input1.txt");
        assert_eq!(6, calculate_code2(path));
    }

    #[test]
    fn test_2_big_rot() {
        // Directly test the step‑by‑step function with huge rotations.
        assert_eq!(10, count_zeros2(&[-1000]));
        assert_eq!(10, count_zeros2(&[1000]));
    }

    // Additional sanity checks (optional, but nice to have)
    #[test]
    fn parse_doc_works() {
        let txt = "L3\nR12\nL5\n";
        let moves = parse_doc(txt);
        assert_eq!(moves, vec![-3, 12, -5]);
    }

    #[test]
    fn count_zeros_matches_python_logic() {
        // Small handcrafted example
        let moves = vec![10, -20, 30];
        // Python version:
        // start 50 -> 60 -> 40 -> 70 (never hits 0) => 0
        assert_eq!(0, count_zeros(&moves));
    }
}
