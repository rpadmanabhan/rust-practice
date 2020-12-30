use std::convert::TryFrom;

#[derive(Debug)]
pub struct AlignResult {
    pub edit_dist: u32,
    pub cigar: String,
}

// Compare 2 AlignResults for equality
impl PartialEq for AlignResult {
    fn eq(&self, other: &Self) -> bool {
        self.edit_dist == other.edit_dist && self.cigar == other.cigar
    }
}

pub fn traceback(dp_mat: &Vec<Vec<u32>>) -> String {
    let mut alignment = String::new();

    let mut i: usize = dp_mat.len() - 1;
    let mut j: usize = dp_mat[0].len() - 1;

    // start from the bottom of the DP matrix and walk backwards
    // making lowest cost moves in the walk back
    while !(i == 0 && j == 0) {
        // Edge conditions - reached top row/col
        if i == 0 {
            alignment.push_str("I");
            j = j - 1;
        } else if j == 0 {
            alignment.push_str("D");
            i = i - 1;
        } else {
            // not at the edges - make lowest cost moves
            let mut move_choices = vec![
                ("M", dp_mat[i - 1][j - 1]),
                ("I", dp_mat[i - 1][j]),
                ("D", dp_mat[i][j - 1]),
            ];
            move_choices.sort_by_key(|k| k.1);
            alignment.push_str(move_choices[0].0);
            if move_choices[0].0 == "M" {
                i = i - 1;
                j = j - 1;
            } else if move_choices[0].0 == "I" {
                i = i - 1;
            } else if move_choices[0].0 == "D" {
                j = j - 1;
            }
        }
    }

    // Reverse the alignment string and bin adjacent positions to create a cigar string
    let mut cigar = String::new();
    let mut prev_char = alignment.chars().rev().next().unwrap();
    let mut count = 0;
    for c in alignment.chars().rev() {
        if prev_char == c {
            count = count + 1;
        } else {
            cigar.push_str(format!("{}{}", count, prev_char).as_str());
            count = 1;
        }
        prev_char = c;
    }
    cigar.push_str(format!("{}{}", count, prev_char).as_str());

    cigar
}

// Calculate editdistance
pub fn edit_dist(s1: &str, s2: &str) -> Option<AlignResult> {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    // initialize DP matrix
    let mut dp_mat = vec![vec![0u32; s2.len() + 1]; s1.len() + 1];
    dp_mat[0] = (0u32..u32::try_from(s2.len()).unwrap() + 1).collect::<Vec<u32>>();
    let mut i = 0;
    for row in &mut dp_mat {
        for col in &mut *row {
            *col = i;
            i += 1;
            break;
        }
    }

    // traverse DP matrix - row wise
    let mut i: usize = 1;
    while i < s1.len() + 1 {
        let mut j: usize = 1;

        while j < s2.len() + 1 {
            // Match / Mismatch penalty
            let mut delta = 0;
            if s1_bytes[i - 1] != s2_bytes[j - 1] {
                delta = 1;
            }

            // Calculate lowest cost move
            let mut move_choices = vec![
                (0, dp_mat[i - 1][j - 1] + delta),
                (1, dp_mat[i - 1][j] + 1),
                (2, dp_mat[i][j - 1] + 1),
            ];
            move_choices.sort_by_key(|k| k.1);

            // Update DP Matrix
            dp_mat[i][j] = move_choices[0].1;

            j += 1;
        }
        i += 1;
    }

    Some(AlignResult {
        edit_dist: dp_mat[s1.len()][s2.len()],
        cigar: traceback(&dp_mat),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_length() {
        let s1 = "ACGTA";
        let s2 = "GCGTA";
        let align_result = edit_dist(s1, s2).unwrap();
        assert_eq!(
            align_result,
            AlignResult {
                edit_dist: 1,
                cigar: String::from("5M")
            }
        );
    }

    #[test]
    fn diff_length() {
        let s1 = "ACGTAAACAC";
        let s2 = "ACGTAACAC";
        let align_result = edit_dist(s1, s2).unwrap();
        assert_eq!(
            align_result,
            AlignResult {
                edit_dist: 1,
                cigar: String::from("6M1I3M")
            }
        );
    }

    #[test]
    fn large_del() {
        let s1 = "ACGTAAAAACCCAGGGCACACGTGGGGCACACACA";
        let s2 = "ACGTCACACGTGGGGCACACACA";
        let align_result = edit_dist(s1, s2).unwrap();
        assert_eq!(
            align_result,
            AlignResult {
                edit_dist: 12,
                cigar: String::from("4M5I1M2I1M3I3M2I14M")
            }
        );
    }

    #[test]
    fn large_ins() {
        let s1 = "ACGTCACACGTGGGGCACACACAGGGGTTGTGTG";
        let s2 = "ACGTCACACGTGGGGCACACACA";
        let align_result = edit_dist(s1, s2).unwrap();
        assert_eq!(
            align_result,
            AlignResult {
                edit_dist: 11,
                cigar: String::from("23M11I")
            }
        );
    }

    #[test]
    fn ins_del_mismatch() {
        let s1 = "ACGTCACACGTGGGGCACACACAGGGGTTGTGTG";
        let s2 = "ATGTCACACGTGGGGCACACA";
        let align_result = edit_dist(s1, s2).unwrap();
        assert_eq!(
            align_result,
            AlignResult {
                edit_dist: 14,
                cigar: String::from("21M13I")
            }
        );
    }
}
