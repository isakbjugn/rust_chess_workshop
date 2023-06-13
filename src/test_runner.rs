use std::process::Command;

pub fn all_tests_pass() -> bool {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to execute 'cargo test'");

    output.status.success()
}

pub fn highest_passing_test() -> Option<u8> {
    let mut highest_passing: Option<u8> = None;
    for task in 0.. {
        match run_tests_for_task(task) {
            true => highest_passing = Some(task),
            _ => break
        }
    };
    highest_passing
}

pub fn run_tests_for_task(task: u8) -> bool {
    let output = Command::new("cargo")
        .args(["test", &format!("task_{}", task)])
        .output()
        .expect("Failed to execute 'cargo test'");

    output.status.success()
}