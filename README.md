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

## Features

This CLI provides a few improvements to the workflow of creating, switching and
removing worktrees:

### Switch

The `gwt switch <BRANCH>` command is designed to be as similar as possible to
a simple `git switch <BRANCH>`, making the process of switching worktrees almost
identical to switching branches.

### Create

Similar to switching, the `gwt switch -c <BRANCH>` command is designed to be as
similar as possible to `git switch -c <BRANCH>`.

This does have a few ergonomic improvements over `git worktree add <PATH> -b <BRANCH> origin/main`
though. Obviously, the command is way shorter. This is because, as explained
earlier, this CLI makes assumptions about how it's used.

For starters, you don't need to provide both a branch name and a worktree path,
because they are assumed to be the same, with the path being a sibling directory
to your repo's root, named after your branch (`/` are replaced with `__`).

Second, unless you explicitly request otherwise with the `--no-commit-ish` flag,
it always creates your new branch based on your remote default branch (as configured
in the `init` command). It even performs a `git fetch` before creating the worktree
to make sure you are up to date with your remote work.

Third, instead of just creating a branch and worktree, this command also checks
if you have a `.env` file. If so, it makes a copy of it in the new directory.
The same check is done for your `node_modules` directory, but instead of making
a copy, a symbolic link is created, this makes it so you don't have to reinstall
your packages from scratch and you also don't have a lot of wasted disk space
with copies of the same repo's dependencies.

### Remove

Removing a worktree also deletes its corresponding branch, so you don't have to
run both `git worktree remove <PATH>` and `git branch -d <BRANCH>`. Instead,
you just run `gwt remove <BRANCH>`. Furthermore, if you want to delete the worktree
you are currently on, you can just run `gwt remove`.

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

### Bash
Add the following to your `~/.bash_profile`:

`source <(gwt-bin init bash --default-branch DEFAULT_BRANCH --remote origin)`

You may change `DEFAULT_BRANCH` and `origin` to fit your needs
