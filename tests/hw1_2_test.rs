use assert_cmd::Command;


#[test]
fn test_hw1_2_failure_test() {
    let mut cmd = Command::cargo_bin("hw1_2").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_hw1_2_multiple_value_test() {
    let mut cmd = Command::cargo_bin("hw1_2").unwrap();
    cmd.args(&["-5","0", "1", "2", "-10","-1"]);

    let expected = "Ascending : [-10, -5, -1, 0, 1, 2]\nDescending : [2, 1, 0, -1, -5, -10]\n";
   
    cmd.assert()
    .success().stdout(expected);
}

#[test]
fn test_hw1_2_single_value_test() {
    let mut cmd = Command::cargo_bin("hw1_2").unwrap();
    cmd.args(&["0"]);

    let expected = "Ascending : [0]\nDescending : [0]\n";
    cmd.assert()
    .success().stdout(expected);
}