use diskpine::commands::origin_to_sim::origin_to_sim;

#[test]
fn test() {
    origin_to_sim("/root/diskpine/tests/test.csv", true).unwrap();
}