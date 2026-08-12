use color_eyre::Result;

pub fn list() -> Result<()> {
    let output = std::process::Command::new("git")
        .arg("worktree")
        .arg("list")
        .output()?;

    println!("{}", std::str::from_utf8(&output.stdout)?);

    Ok(())
}
