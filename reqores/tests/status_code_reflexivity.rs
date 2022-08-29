use reqores::HttpStatusCode;

#[test]
fn test_reflexivity() {
    for i in 100..=999 {
        let status_code = match HttpStatusCode::try_from(i) {
            Ok(status_code) => status_code,
            Err(_) => continue,
        };
        let status_code_u16 = u16::from(status_code);
        assert_eq!(i, status_code_u16);
    }
}
