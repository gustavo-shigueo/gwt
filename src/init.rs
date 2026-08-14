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
        Shell::Nu => {
            format!(
                r#"
def "nu-complete gwt" [spans: list<string>] {{
    let args = ($spans | skip 1)
    let current = ($args | last | default "")

    if ($args | is-empty) {{
        return []
    }}

    let command = ($args | first)

    match $command {{
        "switch" => {{
            let positional = (
                $args
                | skip 1
                | where {{ |arg| not ($arg | str starts-with "-") }}
            )

            # `switch` has exactly one positional argument: name.
            # If there isn't one yet, or we're currently typing it,
            # complete branches.
            if ($positional | is-empty) or (
                ($positional | last) == $current
            ) {{
                ^gwt-bin complete branch $current | lines
            }} else {{
                []
            }}
        }}

        "remove" => {{
            if ($args | length) <= 2 {{
                ^gwt-bin complete branch $current | lines
            }} else {{
                []
            }}
        }}

        _ => []
    }}
}}

@complete 'nu-complete gwt'
def --env --wrapped gwt [...args] {{
    if (($args | first) == "list") and (($args | length) == 1) {{
        gwt-bin list
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

    let path = (gwt-bin ...$args | str trim)
    cd $path
}}
            "#,
                default_branch.replace('/', "__")
            )
        }
        Shell::Powershell => {
            format!(
                r#"
function gwt {{
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Args
    )

    # Handle: gwt remove
    if ($Args.Count -eq 1 -and $Args[0] -eq "remove") {{
        $branch = (git branch --show-current).Trim()

        Set-Location ../{}

        # Run in the background
        Start-Job -ScriptBlock {{
            param($branch)
            & gwt-bin remove $branch
        }} -ArgumentList $branch | Out-Null

        return
    }}

    if ($Args.Count -eq 1 -and $Args[0] -eq "list") {{
        & gwt-bin list
        return
    }}

    if ($Args[0] -eq "complete") {{
        & gwt-bin @Args
        return
    }}

    # Handle: gwt --help / -h
    if ($Args -contains "--help" -or $Args -contains "-h") {{
        & gwt-bin @Args
        return
    }}

    # Run gwt-bin and change to the returned path
    $path = (& gwt-bin @Args).Trim()
    Set-Location $path
}}

$env:COMPLETE = "powershell"
gwt-bin | Out-String | Invoke-Expression
Remove-Item Env:\COMPLETE
            "#,
                default_branch.replace('/', "__")
            )
        }
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
