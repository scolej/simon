use model::*;
use rand::{self, Rng};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::*;

/// Send random builds periodically.
pub fn findBuilds(sink: Sender<Build>) {
    loop {
        sink.send(a_random_build());
        println!("sent!");
        thread::sleep(Duration::from_millis(1000));
    }
}

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

fn a_random_build() -> Build {
    let num: i32 = rand::thread_rng().gen();
    Build {
        name: "The Big Project".to_string(),
        identifier: num.to_string(),
        status: random_status(),
        branch: Some(random_branch()),
        commit: Some(random_commit()),
        // elapsed_time: Duration::from_secs(rand::thread_rng().gen()),
    }
}
