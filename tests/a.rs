use test_template_lib_crate::utils;

#[test]
fn feature() {
    let sum = utils::add(1, 1);
    assert_eq!(sum, 2)
}
