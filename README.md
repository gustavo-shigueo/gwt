# GWT

This is a small CLI program made to help me with my daily worktree process.
It is a bespoke tool created with my own personal workflow in mind, so it may or may not serve your needs.

## Requirements

GWT expects a very specific folder structure for its worktrees. More specifically,
it expects all worktrees to be sibling directories, named the same as their branches
(except with `/` replaced by `__`). It also expects a persistent branch/worktree. This
branch will be refered to in this README as `DEFAULT_BRANCH`.

Expected file tree:
```
my_repo/
    |__ DEFAULT_BRANCH/
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

## Windows specific warning:
This binary creates a symlink to the DEFAULT_BRANCH's node_modules folder (if one
exists), which requires developer mode to be enabled in the Windows systems settings

### Nushell
In `config.nu`, add the following line:

`gwt-bin init nu --default-branch DEFAULT_BRANCH --remote origin | save -f ($nu.data-dir | path join "vendor/autoload/gwt.nu")`

You may change `DEFAULT_BRANCH` and `origin` to fit your needs

### Powershell
Add the following to your `$PROFILE`:

`Invoke-Expression (& { (gwt-bin init powershell --default-branch DEFAULT_BRANCH --remote origin | Out-String) })`

You may change `DEFAULT_BRANCH` and `origin` to fit your needs
