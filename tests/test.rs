#[test]
fn check_file_name() {
    assert_eq!(file2png::file_name("../../test.png"), "test.png");
    assert_eq!(file2png::file_name("C:\\Random\\random.input"), "random.input");
}

#[test]
fn check_conversion() {
    assert_eq!(file2png::str_to_vec("abc"), vec!['a' as u8, 'b' as u8, 'c' as u8]);
    assert_eq!(file2png::str_to_vec(""), vec![]);
}

#[test]
fn check_hashsum() {
    assert_eq!(file2png::hash_vec(&vec![49, 52, 51]), "d6f0c71ef0c88e45e4b3a2118fcb83b0def392d759c901e9d755d0e879028727".to_string());
}