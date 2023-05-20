use std::process::Command;

pub fn all_tests_pass() -> bool {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to execute 'cargo test'");

    output.status.success()
}

pub fn highest_passing_test() -> Option<u8> {
    let mut highest_passing = 0;
    for task in 1..2 {
        match run_tests_for_task(task) {
            true => highest_passing = task,
            false => break
        }
    }
    match highest_passing {
        0 => None,
        x => Some(x)
    }
}

pub fn run_tests_for_task(task: u8) -> bool {
    let output = Command::new("cargo")
        .args(["test", &format!("task_{}", task)])
        .output()
        .expect("Failed to execute 'cargo test'");

    output.status.success()
}