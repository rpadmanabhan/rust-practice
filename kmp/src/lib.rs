// To Do:
// 1.) Using .as_bytes() to get index access to strings, explore for alternatives in Rust
//     Related: Use generics to accept &str or Vec<u8> (bytes)
// 2.) Test and handle weird input, e.g. empty strings
//     Related: Extend code to any UTF-8 encoded string. Focus right now on Biological Strings


// Return a vector containing the KMP failure function values
// Value at index i corresponds to the value for the prefix of length (i + 1)
// i.e. i = 0 corresponds to needle[0] (is always 0) and i = len(needle) - 1 corresponds to needle
pub fn return_failure_function_table(needle: &str) -> Vec<usize> {

    // init jump table - idx represents jump for prefix of length (idx + 1)
    let mut jump_table: Vec<usize> = vec![0; needle.len()];

    // init needle as bytes for index access
    let needle_bytes = needle.as_bytes();
    let mut i:usize = 1;

    // loop over needle and compute jumps for each prefix size
    while i < needle_bytes.len() {
        let mut j = i;
        while j > 0 {
            if needle_bytes[i] == needle_bytes[jump_table[j - 1]] {
                jump_table[i] = 1 + jump_table[i - 1];
                break;
            }
            else {
                j = jump_table[jump_table[j - 1]];
            }
        }
        i += 1;
    }

    jump_table
}

pub fn kmp_wrapper(needle: &str, haystack: &str) -> Option<usize> {

    // Index needle - i.e. init jumps for each prefix size of needle
    let jump_table = return_failure_function_table(&needle);

    // KMP search
    kmp(needle, haystack, &jump_table)
}

// Return idx in haystack where the first occurence of needle occurs
pub fn kmp(needle: &str, haystack: &str, jump_table: &Vec<usize>) -> Option<usize> {
    // Search for needle in haystack using jump_table to skip unwanted comparisons
    // idx in haystack
    let mut i = 0;
    // idx in needle
    let mut j = 0;
    // start idx for match in needle
    let mut i0 = 0;

    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    while haystack_bytes.len() - i0 >= needle_bytes.len(){
        if j == needle_bytes.len() {
            return Some(i0);
        }

        if haystack_bytes[i] == needle_bytes[j] {
            j += 1;
            i += 1;
        }
        else {
            if j == 0 {
                j = 0;
                i += 1;
            }
            else {
                j = jump_table[j - 1];
            }

            i0 = i - j;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failure_function_table() {
        let needle = "AAAGAAA";
        assert_eq!(vec![0, 1, 2, 0, 1, 2, 3],
                   return_failure_function_table(&needle));
    }

    #[test]
    fn failure_function_table_kmp77() {
        // test example in the KMP 1977 paper
        let needle = "abcabcacab";
        assert_eq!(vec![0, 0, 0, 1, 2, 3, 4, 0, 1, 2],
                   return_failure_function_table(&needle));
    }

    #[test]
    fn failure_function_repetitive() {
        let needle = "AAAAAAA";
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6],
                   return_failure_function_table(&needle));
    }

    #[test]
    fn kmp_search_match() {
        assert_eq!(std::option::Option::Some(7),
                   kmp_wrapper("AAAGAAA", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_match_start() {
        assert_eq!(std::option::Option::Some(0),
                   kmp_wrapper("AGCATT", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_match_first_char() {
        assert_eq!(std::option::Option::Some(0),
                   kmp_wrapper("A", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_match_first_occurence() {
        assert_eq!(std::option::Option::Some(7),
                   kmp_wrapper("AAA", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_match_end() {
        assert_eq!(std::option::Option::Some(14),
                   kmp_wrapper("TTT", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_match_full() {
        assert_eq!(std::option::Option::Some(0),
                   kmp_wrapper("AGCATTCAAAGAAATTT", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_match_repetitive1() {
        assert_eq!(std::option::Option::Some(0),
                   kmp_wrapper("AAAAA", "AAAAAAAAAAAAAAAAA"));
    }

    #[test]
    fn kmp_search_match_repetitive2() {
        assert_eq!(std::option::Option::Some(5),
                   kmp_wrapper("AAAAA", "CCCCCAAAAAAAAAAAAAAAAA"));
    }

    #[test]
    fn kmp_search_match_repetitive3() {
        assert_eq!(std::option::Option::Some(1),
                   kmp_wrapper("CACACACA", "ACACACACAAAAAAAAAAAAA"));
    }

    #[test]
    fn kmp_search_match_single_chars() {
        assert_eq!(std::option::Option::Some(0),
                   kmp_wrapper("C", "C"));
    }

    #[test]
    fn kmp_search_match_edge_case() {
        assert_eq!(std::option::Option::Some(8),
                   kmp_wrapper("AAAAAAAAAAA", "ACACACACAAAAAAAAAAAAA"));
    }

    #[test]
    fn kmp_search_match_edge_case2() {
        assert_eq!(std::option::Option::Some(24),
                   kmp_wrapper("AAAAAAAAAAAAAAAAAAAAAAAAATCAAAAAAACAAAACACAAAACTC",
                               "TGGCTCTAAAATGCTCTGTTCTCAAAAAAAAAAAAAAAAAAAAAAAAAATCAAAAAAACAAAACACAAAACTCTTTAGAGAATCACCCCCCCTTACATTCTTG"));
    }

    #[test]
    fn kmp_search_no_match() {
        assert_eq!(std::option::Option::None,
                   kmp_wrapper("AAAGAAC", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_no_match_edge_case1() {
        assert_eq!(std::option::Option::None,
                   kmp_wrapper("GAAATTTC", "AGCATTCAAAGAAATTT"));
    }

    #[test]
    fn kmp_search_no_match_needle_longer() {
        assert_eq!(std::option::Option::None,
                   kmp_wrapper("AGCATTCAAAGAAATTTCC", "AGCATTCAAAGAAATTT"));
    }


}
