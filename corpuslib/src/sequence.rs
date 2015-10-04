use std::cmp;

pub fn sequence_ordering_n(seq1: &[usize], seq2: &[usize], n: usize) -> cmp::Ordering {
    let (n1, n2) = (seq1.len(), seq2.len());
    let mut look_n = cmp::min(n1, n2);
    if n < look_n {
        look_n = n;
    }
    // Make comparisons.
    for pos in 0..look_n {
        if seq1[pos] < seq2[pos] {
            return cmp::Ordering::Less;
        }
        else if seq2[pos] < seq1[pos] {
            return cmp::Ordering::Greater;
        }
    }
    // All assigned elements are the same, so make comparison based on length (shorter is lesser).
    if look_n == n {
        return cmp::Ordering::Equal;
    } else if n1 < n2 {
        return cmp::Ordering::Less;
    }
    else {
        return cmp::Ordering::Greater;
    }
}

pub fn sequence_ordering(seq1: &[usize], seq2: &[usize]) -> cmp::Ordering {
    let (n1, n2) = (seq1.len(), seq2.len());
    sequence_ordering_n(seq1, seq2, cmp::max(n1, n2) + 1)
}
