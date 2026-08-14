use clap_complete::CompletionCandidate;
use std::ffi::OsStr;

fn git_branches() -> Vec<String> {
    let Ok(output) = std::process::Command::new("git")
        .arg("for-each-ref")
        .arg("--format=%(refname:short)")
        .arg("refs/heads/")
        .output()
    else {
        return vec![];
    };

    std::str::from_utf8(&output.stdout)
        .unwrap_or_default()
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

pub fn complete_branches(current: Option<&str>) -> Vec<String> {
    current.map_or_else(git_branches, |current| {
        git_branches()
            .into_iter()
            .filter(|x| x.starts_with(current))
            .collect()
    })
}

pub fn complete_branch_candidates(current: &OsStr) -> Vec<CompletionCandidate> {
    let Some(current) = current.to_str() else {
        return vec![];
    };

    git_branches()
        .into_iter()
        .filter(|x| x.starts_with(current))
        .map(CompletionCandidate::new)
        .collect()
}

fn worktree_branches() -> Vec<String> {
    let Ok(output) = std::process::Command::new("git")
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .output()
    else {
        return vec![];
    };

    std::str::from_utf8(&output.stdout)
        .unwrap_or_default()
        .lines()
        .filter_map(|x| x.strip_prefix("branch refs/heads/"))
        .map(ToOwned::to_owned)
        .collect()
}

pub fn complete_worktree_branches(current: Option<&str>) -> Vec<String> {
    current.map_or_else(worktree_branches, |current| {
        worktree_branches()
            .into_iter()
            .filter(|x| x.starts_with(current))
            .collect()
    })
}

pub fn complete_worktree_branch_candidates(current: &OsStr) -> Vec<CompletionCandidate> {
    let Some(current) = current.to_str() else {
        return vec![];
    };

    worktree_branches()
        .into_iter()
        .filter(|x| x.starts_with(current))
        .map(CompletionCandidate::new)
        .collect()
}
