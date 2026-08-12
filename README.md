# GWT

This is a small CLI program made to help me with my daily worktree process.
It is a bespoke tool created with my own personal workflow in mind, so it may or may not serve your needs.

## Requirements

GWT expects a very specific folder structure for its worktrees. More specifically,
it expects all worktrees to be sibling directories, named the same as their branches
(except with `/` replaced by `__`). It also expects a persistent branch/worktree called
`develop` (I will try to make this configurable in the future), which is the repository's
default branch.

Expected file tree:
```
my_repo/
    |__ develop/
    |__ my_feature/
    |__ hotfix/
    |__ hotfix__2/ (the branch is expected to be called `hotfix/2`)
```

## Usage

### Navigating between worktrees
`gwt switch <BRANCH_NAME>`

### Create a new branch and worktree
`gwt switch -c <BRANCH_NAME>`

### Delete a worktree
`gwt remove <BRANCH_NAME>`

### Delete the current worktree (won't work in `develop`)
`gwt remove`

### List existing worktrees
`gwt list`

## Installation
1. Run `cargo install gwt-bin`
2. Follow the instructions for the appropriate shell

### Nushell
In `config.nu`, add the following line:
`gwt-bin init nu | save -f ($nu.data-dir | path join "vendor/autoload/gwt.nu")`

### Powershell
Add the following to your `$PROFILE`
`Invoke-Expression (& { (gwt-bin init powershell | Out-String) })`
