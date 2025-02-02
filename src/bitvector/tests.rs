use super::*;
use crate::perf_and_test_utils::{gen_strictly_increasing_sequence, negate_vector};

#[test]
fn test_is_empty() {
    let bv = BitVector::default();
    assert!(bv.is_empty());
}

// Build a bit vector of size n with even positions set to one
// and odd ones to zero
fn build_alternate(n: usize) -> BitVector {
    let mut bv = BitVector::with_capacity(n);
    for i in 0..n {
        bv.push(i % 2 == 0);
    }
    bv
}

#[test]
fn test_get() {
    let n = 1024 + 13;
    let bv = build_alternate(n);

    for i in 0..n {
        assert_eq!(bv.get(i).unwrap(), i % 2 == 0);
    }
}

#[test]
fn test_iter() {
    let n = 1024 + 13;
    let bv = build_alternate(n);

    for (i, bit) in bv.into_iter().enumerate() {
        assert_eq!(bit, i % 2 == 0);
    }
}

#[test]
fn test_get_set_bits() {
    let n = 1024 + 13;
    let mut bv = BitVector::new();
    bv.extend_with_zeros(n);

    assert_eq!(bv.get_bits(61, 35).unwrap(), 0);
    assert_eq!(bv.get_bits(0, 42).unwrap(), 0);
    assert_eq!(bv.get_bits(n - 42 - 1, 42).unwrap(), 0);
    assert_eq!(bv.get_bits(n - 42, 42), None);
    bv.set_bits(0, 6, 42);
    assert_eq!(bv.get_bits(0, 6).unwrap(), 42);
    bv.set_bits(n - 61 - 1, 61, 42);
    assert_eq!(bv.get_bits(n - 61 - 1, 61).unwrap(), 42);
    bv.set_bits(n - 67 - 1, 33, 42);
    assert_eq!(bv.get_bits(n - 67 - 1, 33).unwrap(), 42);
}

#[test]
fn test_from_iter() {
    let n = 1024 + 13;
    let bv = build_alternate(n);

    let bv2: BitVector = (0..n).map(|x| x % 2 == 0).collect();

    assert_eq!(bv, bv2);

    /* Note: if last bits are zero, the bit vector may differ
    because we are inserting only position of ones */
    let bv2: BitVector = (0..n).filter(|x| x % 2 == 0).collect();

    assert_eq!(bv, bv2);
}

#[test]
fn test_iter_zeros() {
    let bv = BitVector::default();
    let v: Vec<usize> = bv.zeros().collect();
    assert!(v.is_empty());

    let vv: Vec<usize> = vec![0, 63, 128, 129, 254, 1026];
    let mut bv: BitVector = vv.iter().copied().collect();

    let v: Vec<usize> = bv.zeros().collect();
    assert_eq!(v, negate_vector(&vv));

    // do it again but extends with zeros
    bv.extend_with_zeros(100);

    let v: Vec<usize> = bv.zeros().collect();
    assert_eq!(vv, negate_vector(&v));
}

#[test]
fn test_iter_ones() {
    let bv = BitVector::default();
    let v: Vec<usize> = bv.ones().collect();
    assert!(v.is_empty());

    let vv: Vec<usize> = vec![0, 63, 128, 129, 254, 1026];
    let bv: BitVector = vv.iter().copied().collect();

    let v: Vec<usize> = bv.ones().collect();
    assert_eq!(v, vv);

    let vv: Vec<usize> = (0..1024).collect();
    let bv: BitVector = vv.iter().copied().collect();
    let v: Vec<usize> = bv.ones().collect();
    assert_eq!(v, vv);

    let vv = gen_strictly_increasing_sequence(1024 * 4, 1 << 20);

    let bv: BitVector = vv.iter().copied().collect();
    let v: Vec<usize> = bv.ones().collect();
    assert_eq!(v, vv);
}
