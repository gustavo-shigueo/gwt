use crate::{
    command::{InitCommand, Shell},
    config::Config,
};
use color_eyre::Result;

#[allow(clippy::too_many_lines)]
pub fn init(
    InitCommand {
        shell,
        default_branch,
        remote,
    }: InitCommand,
) -> Result<()> {
    let function = match shell {
        Shell::Nu => init_nu(&default_branch),
        Shell::Powershell => init_powershell(&default_branch),
    };

    let mut config_path = std::env::current_exe()?;
    config_path.pop();
    config_path.push("gwt.toml");

    let config = Config {
        default_branch,
        remote,
    };

    let buffer = toml::to_string(&config)?;
    std::fs::write(config_path, buffer)?;

    println!("{}", function.trim());
    Ok(())
}

fn init_nu(default_branch: &str) -> String {
    format!(
        r#"
def "nu-complete gwt" [spans: list<string>] {{
    let args = ($spans | skip 1)
    let current = ($args | last | default "")
    let command = ($args | first)

    match $command {{
        "switch" => {{
            ^gwt-bin complete switch $current | lines
        }}

        "remove" => {{
            ^gwt-bin complete remove $current | lines
        }}

        _ => (["switch" "list" "remove"] | where (str starts-with $current))
    }}
}}

@complete 'nu-complete gwt'
def --env --wrapped gwt [...args] {{
    if (($args | first) == "list") and (($args | length) == 1) {{
        gwt-bin list
        return
    }}

    if (($args | first) == "sync") {{
        gwt-bin ...$args

        return
    }}

    if (($args | first) == "complete") {{
        gwt-bin ...$args

        return
    }}

    if (($args | first) == "remove") and (($args | length) == 1) {{
        let branch = (git branch --show-current | str trim)
        cd ../{}

        job spawn {{
            gwt-bin remove $branch
        }}

        return
    }}

    if ("--help" in $args) or ("-h" in $args) {{
        ^gwt-bin ...$args
        return
    }}

    if ("--version" in $args) or ("-V" in $args) {{
        ^gwt-bin ...$args
        return
    }}

    let path = (gwt-bin ...$args | str trim)
    cd $path
}}
            "#,
        default_branch.replace('/', "__")
    )
}

fn init_powershell(default_branch: &str) -> String {
    const POWERSHELL_5_COMPLETE: &str = include_str!("../assets/ps_autocomplete.ps1");

    format!(
        r#"
function gwt {{
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$GwtArgs
    )

    # Handle: gwt remove
    if ($GwtArgs.Count -eq 1 -and $GwtArgs[0] -eq "remove") {{
        $branch = (git branch --show-current).Trim()

        Set-Location ../{}

        # Run in the background
        Start-Job -ScriptBlock {{
            param($branch)
            & gwt-bin remove $branch
        }} -ArgumentList $branch | Out-Null

        return
    }}

    if ($GwtArgs.Count -eq 1 -and $GwtArgs[0] -eq "list") {{
        & gwt-bin list
        return
    }}

    if ($GwtArgs[0] -eq "sync") {{
        & gwt-bin @GwtArgs
        return
    }}

    if ($GwtArgs[0] -eq "complete") {{
        & gwt-bin @GwtArgs
        return
    }}

    # Handle: gwt --help / -h
    if ($GwtArgs -contains "--help" -or $GwtArgs -contains "-h") {{
        & gwt-bin @GwtArgs
        return
    }}

    # Handle: gwt --version
    if ($GwtArgs -contains "--version" -or $GwtArgs -contains "-V") {{
        & gwt-bin @GwtArgs
        return
    }}

    # Run gwt-bin and change to the returned path
    $path = (& gwt-bin @GwtArgs).Trim()
    Set-Location $path
}}

$env:COMPLETE = "powershell"

if ($PSVersionTable.PSVersion.Major -ge 6) {{
    gwt-bin | Out-String | Invoke-Expression
}} else {{
    {}
}}

Remove-Item Env:\COMPLETE
            "#,
        default_branch.replace('/', "__"),
        POWERSHELL_5_COMPLETE
    )
}
