use assert_cmd::Command;

const DEFAULT_NUMBER_OF_PASSWORDS: usize = 1;
const DEFAULT_PASSWORD_LENGTH: usize = 10;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn help_check() -> TestResult {
    Command::cargo_bin("password_generator")?
        .args(["-h"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage"))
        .stdout(predicates::str::contains("Options"));
    Ok(())
}

#[test]
fn no_arg() -> TestResult {
    let output = Command::cargo_bin("password_generator")?
        .output()?;
    assert!(output.status.success());

    let stdout: &str = std::str::from_utf8(&output.stdout)?;
    let len: usize = stdout.trim().len();
    assert_eq!(DEFAULT_PASSWORD_LENGTH, len);

    Ok(())
}

fn run(args: &[&str], number_passwords: usize, password_length: usize) -> TestResult {
    let output = Command::cargo_bin("password_generator")?
        .args(args)
        .output()?;
    assert!(output.status.success());

    let stdout = std::str::from_utf8(&output.stdout)?;
    let mut passwords: Vec<&str> = stdout.split("\n").collect();
    if passwords.last().expect("Cannot get the last element").len() == 0 {
        passwords.pop();
    }
    assert_eq!(number_passwords, passwords.len());

    for password in passwords {
        assert_eq!(password_length, password.len());
    }

    Ok(())
}

#[test]
fn generate_2_password() -> TestResult {
    run(&["-n", "2"], 2, DEFAULT_PASSWORD_LENGTH)
}

#[test]
fn generate_15_length_password() -> TestResult {
    run(&["-l", "15"], DEFAULT_NUMBER_OF_PASSWORDS, 15)
}

#[test]
fn generate_5_password_20_length() -> TestResult {
    run(&["--number-of-passwords", "5", "--length", "20"], 5, 20)
}

#[test]
fn invalid_length1() -> TestResult {
    Command::cargo_bin("password_generator")?
        .args(["-l", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Length of the password must be greater than 2 to protect secure."));
    Ok(())
}

#[test]
fn invalid_length2() -> TestResult {
    Command::cargo_bin("password_generator")?
        .args(["-l", "test"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("invalid digit found in string"));
    Ok(())
}

#[test]
fn invalid_number_of_passwords1() -> TestResult {
    Command::cargo_bin("password_generator")?
        .args(["-n", "0"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Number of passwords must be greater than or equal to 1."));

    Ok(())
}

#[test]
fn invalid_number_of_passwords2() -> TestResult {
    Command::cargo_bin("password_generator")?
        .args(["-n", "invalid"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("invalid digit found in string"));

    Ok(())
}
