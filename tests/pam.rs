extern crate crispyr;

use crispyr::common::encode_dna;
use crispyr::pam::{Position, PAM};

#[test]
fn test_pam_lengths() {
    assert_eq!(PAM::head(b"").len(), 0);
    assert_eq!(PAM::tail(b"").len(), 0);
    assert_eq!(PAM::head(b"YTTN").len(), 4);
    assert_eq!(PAM::tail(b"NGG").len(), 3);
}

#[test]
fn test_pam_position() {
    assert_eq!(PAM::head(b"YTTN").position(), Position::Head);
    assert_eq!(PAM::tail(b"NGG").position(), Position::Tail);
}

#[test]
fn test_empty_head_pam_matches() {
    let pam = PAM::head(b"");

    assert!(pam.matches(b""));
    assert!(pam.matches(b"A"));
}

#[test]
fn test_empty_head_pam_kmer_slice() {
    let pam = PAM::head(b"");
    let kmer = encode_dna(b"ACTGAGTCAGATA").unwrap();

    assert_eq!(pam.kmer(b""), None);
    assert_eq!(pam.kmer(b"A"), None);
    assert_eq!(pam.kmer(b"ACTGAGTCAGATA"), Some((0, kmer)));
    assert_eq!(pam.kmer(b"ACTGAGTCAGATAT"), Some((0, kmer)));
}

#[test]
fn test_empty_tail_pam_matches() {
    let pam = PAM::tail(b"");

    assert!(pam.matches(b""));
    assert!(pam.matches(b"A"));
}

#[test]
fn test_empty_tail_pam_kmer_slice() {
    let pam = PAM::tail(b"");
    let kmer = encode_dna(b"ACTGAGTCAGATA").unwrap();

    assert_eq!(pam.kmer(b""), None);
    assert_eq!(pam.kmer(b"A"), None);
    assert_eq!(pam.kmer(b"ACTGAGTCAGATA"), Some((13, kmer)));
    assert_eq!(pam.kmer(b"TACTGAGTCAGATA"), Some((14, kmer)));
}

#[test]
fn test_head_pam_matches() {
    let pam = PAM::head(b"YTTN");

    assert!(!pam.matches(b"TTT"));
    assert!(pam.matches(b"TTTA"));
    assert!(pam.matches(b"CTTT"));
    assert!(pam.matches(b"TTTG"));
    assert!(pam.matches(b"CTTA"));
    assert!(!pam.matches(b"ATTA"));
    assert!(!pam.matches(b"GTTTA"));
}

#[test]
fn test_head_kmer() {
    let pam = PAM::head(b"YTTN");
    let kmer = encode_dna(b"ACTGAGTCAGATA").unwrap();

    assert_eq!(pam.kmer(b"CTTGACTGAGTCAGAT"), None); // Too short
    assert_eq!(pam.kmer(b"CTTGACTGAGTCAGATA"), Some((0, kmer)));
    assert_eq!(pam.kmer(b"CTTGACTGAGTCAGATAT"), Some((0, kmer)));
    assert_eq!(pam.kmer(b"ACTTGACTGAGTCAGATA"), None); // Inside
    assert_eq!(pam.kmer(b"ACTGAGTCAGATACTTG"), None); // At tail
}

#[test]
fn test_tail_pam_matches() {
    let pam = PAM::tail(b"NGG");

    assert!(!pam.matches(b"GG"));
    assert!(pam.matches(b"AGG"));
    assert!(pam.matches(b"TCGG"));
    assert!(!pam.matches(b"AGN"));
    assert!(!pam.matches(b"AGT"));
    assert!(!pam.matches(b"AGGA"));
}

#[test]
fn test_tail_kmer() {
    let pam = PAM::tail(b"NGG");
    let kmer = encode_dna(b"ACTGAGTCAGATA").unwrap();

    assert_eq!(pam.kmer(b"CTGAGTCAGATATGG"), None); // Too short
    assert_eq!(pam.kmer(b"ACTGAGTCAGATATGG"), Some((13, kmer)));
    assert_eq!(pam.kmer(b"AACTGAGTCAGATATGG"), Some((14, kmer)));
    assert_eq!(pam.kmer(b"ACTGAGTCAGATATGGA"), None); // Inside
    assert_eq!(pam.kmer(b"TGGACTGAGTCAGATA"), None); // At head
}

#[test]
fn test_to_string() {
    let pam = PAM::head(b"YTTN");

    assert_eq!(pam.to_string(), "YTTN");
}

#[test]
fn test_enzyme_kmer_cas9() {
    let pam = PAM::tail(b"NGG");

    assert_eq!(pam.kmer_slice(b""), b"");
    assert_eq!(pam.kmer_slice(b"aGg"), b"");
    assert_eq!(pam.kmer_slice(b"aTaCaGg"), b"aTaC");
    assert_eq!(pam.kmer_slice(b"actaTccaCaTaCaGg"), b"actaTccaCaTaC");
    assert_eq!(pam.kmer_slice(b"TatAcTTactaTccaCaTaCaGg"), b"actaTccaCaTaC");
}

#[test]
fn test_enzyme_kmer_mut_cas9() {
    let pam = PAM::tail(b"NGG");

    assert_eq!(pam.kmer_slice_mut(&mut b"".to_owned()), b"");
    assert_eq!(pam.kmer_slice_mut(&mut b"aGg".to_owned()), b"");
    assert_eq!(pam.kmer_slice_mut(&mut b"aTaCaGg".to_owned()), b"aTaC");
    assert_eq!(
        pam.kmer_slice_mut(&mut b"actaTccaCaTaCaGg".to_owned()),
        b"actaTccaCaTaC"
    );
    assert_eq!(
        pam.kmer_slice_mut(&mut b"TatAcTTactaTccaCaTaCaGg".to_owned()),
        b"actaTccaCaTaC"
    );
}

#[test]
fn test_enzyme_kmer_mad7() {
    let pam = PAM::head(b"YTTN");

    assert_eq!(pam.kmer_slice(b""), b"");
    assert_eq!(pam.kmer_slice(b"TatA"), b"");
    assert_eq!(pam.kmer_slice(b"TatAcTTa"), b"cTTa");
    assert_eq!(pam.kmer_slice(b"TatAcTTactaTccaCa"), b"cTTactaTccaCa");
    assert_eq!(pam.kmer_slice(b"TatAcTTactaTccaCaTaCaGg"), b"cTTactaTccaCa");
}

#[test]
fn test_enzyme_kmer_mut_mad7() {
    let pam = PAM::head(b"YTTN");

    assert_eq!(pam.kmer_slice_mut(&mut b"".to_owned()), b"");
    assert_eq!(pam.kmer_slice_mut(&mut b"TatA".to_owned()), b"");
    assert_eq!(pam.kmer_slice_mut(&mut b"TatAcTTa".to_owned()), b"cTTa");
    assert_eq!(
        pam.kmer_slice_mut(&mut b"TatAcTTactaTccaCa".to_owned()),
        b"cTTactaTccaCa"
    );
    assert_eq!(
        pam.kmer_slice_mut(&mut b"TatAcTTactaTccaCaTaCaGg".to_owned()),
        b"cTTactaTccaCa"
    );
}
