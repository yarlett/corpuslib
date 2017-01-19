use std::cmp;

pub fn sequence_compare(seq1: &[usize], seq2: &[usize]) -> cmp::Ordering {
    sequence_compare_n(seq1, seq2, cmp::max(&seq1.len(), &seq2.len()))
}

pub fn sequence_compare_n(seq1: &[usize],
                          seq2: &[usize],
                          comparison_length: &usize)
                          -> cmp::Ordering {
    let (n1, n2) = (seq1.len(), seq2.len());
    let mut n = cmp::min(n1, n2);
    if *comparison_length < n {
        n = *comparison_length;
    }
    // Make comparisons over defined extent of sequences.
    for seq_pos in 0..n {
        if seq1[seq_pos] < seq2[seq_pos] {
            return cmp::Ordering::Less;
        }
        if seq1[seq_pos] > seq2[seq_pos] {
            return cmp::Ordering::Greater;
        }
    }
    // All assigned elements are the same, so make comparison based on length (shorter is lesser).
    if n == *comparison_length {
        return cmp::Ordering::Equal;
    } else if n1 < n2 {
        return cmp::Ordering::Less;
    } else {
        return cmp::Ordering::Greater;
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::cmp;
    use super::*;

    fn random_sequence(ntypes: usize, ntokens: usize) -> Vec<usize> {
        let mut sequence: Vec<usize> = Vec::with_capacity(ntokens);
        for _ in 0..ntokens {
            let v = rand::random::<usize>() % ntypes;
            sequence.push(v);
        }
        sequence
    }

    #[test]
    fn sequences_equal_themself() {
        for _ in 0..100 {
            let seq = random_sequence(10, 100);
            assert!(sequence_compare(&seq[..], &seq[..]) == cmp::Ordering::Equal);
            assert!(sequence_compare_n(&seq[..], &seq[..], &seq.len()) == cmp::Ordering::Equal);
        }
    }

    #[test]
    fn sequences_reverse_comparison_ordering() {
        for _ in 0..100 {
            let seq1 = random_sequence(10, 100);
            let seq2 = random_sequence(10, 100);
            let cmp1 = sequence_compare(&seq1[..], &seq2[..]);
            let cmp2 = sequence_compare(&seq2[..], &seq1[..]);
            assert!((cmp1 == cmp::Ordering::Equal && cmp2 == cmp::Ordering::Equal) ||
                    (cmp1 == cmp::Ordering::Less && cmp2 == cmp::Ordering::Greater) ||
                    (cmp1 == cmp::Ordering::Greater && cmp2 == cmp::Ordering::Less));
        }
    }
}
