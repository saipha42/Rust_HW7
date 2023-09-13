use assert_cmd::Command;


#[test]
fn test_hw2_2_failure_test() {
    let mut cmd = Command::cargo_bin("hw2_2").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_hw2_2_multiple_value_test() {
    let mut cmd = Command::cargo_bin("hw2_2").unwrap();
    cmd.args(&["1", "2", "1", "3", "-1"]);

    let expected = "Ascending : [(1.0, 2.0), (1.0, 3.0)]\nDescending : [(1.0, 3.0), (1.0, 2.0)]\n";
    cmd.assert()
    .success().stdout(expected);
}

#[test]
fn test_hw2_2_single_value_test() {
    let mut cmd = Command::cargo_bin("hw2_2").unwrap();
    cmd.args(&["0"]);

    let expected = "Ascending : []\nDescending : []\n";
    cmd.assert()
    .success().stdout(expected);
}