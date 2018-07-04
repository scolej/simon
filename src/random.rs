use model::{Build, BuildId, BuildStatus};
use rand::{self, Rng};
use std::time::Duration;

fn random_status() -> BuildStatus {
    static STATUSES: [BuildStatus; 3] = [
        BuildStatus::InProgress,
        BuildStatus::Failed,
        BuildStatus::Passed,
    ];
    STATUSES[rand(STATUSES.len())]
}

fn rand(max: usize) -> usize {
    rand::thread_rng().gen::<usize>() % max
}

fn random_branch() -> String {
    static BRANCHES: [&str; 6] = [
        "master",
        "develop",
        "feature/the-best-thing",
        "feature/the-biggest-thing",
        "feature/the-fastest-thing",
        "feature/the-most-stylish-thing",
    ];
    BRANCHES[rand(BRANCHES.len())].to_string()
}

fn random_commit() -> String {
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];
    let mut s = String::new();
    for _n in 0..10 {
        let i: usize = rand::thread_rng().gen::<usize>() % chars.len();
        s.push(chars[i]);
    }
    s
}

pub fn a_random_build() -> Build {
    Build {
        id: BuildId {
            branch: random_branch(),
            number: rand::thread_rng().gen(),
        },
        commit: random_commit(),
        status: random_status(),
        elapsed_time: Duration::from_secs(rand::thread_rng().gen()),
    }
}
