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
        Shell::Bash => init_bash(&default_branch),
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

        _ => (["switch" "list" "remove" "sync"] | where (str starts-with $current))
    }}
}}

@complete 'nu-complete gwt'
def --env --wrapped gwt [...args] {{
    if ("--help" in $args) or ("-h" in $args) {{
        ^gwt-bin ...$args
        return
    }}

    if ("--version" in $args) or ("-V" in $args) {{
        ^gwt-bin ...$args
        return
    }}

    if ($args | first) == "list" {{
        ^gwt-bin ...$args
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

    if (($args | first) == "remove") and ((($args | skip 1) | all {{|arg| $arg | str starts-with "-"}})) {{
        let branch = (git branch --show-current | str trim)
        let flags = ($args | skip 1)
        cd ../{}

        job spawn {{
            gwt-bin remove $branch ...$flags
        }}

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

    if ($GwtArgs[0] -eq "list") {{
        & gwt-bin @GwtArgs
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

    # Handle: gwt remove
    if ($GwtArgs[0] -eq "remove" -and @($GwtArgs | Select-Object -Skip 1 | Where-Object {{ $_ -notlike "-*" }}).Count -eq 0) {{
        $branch = (git branch --show-current).Trim()
        $flags = @($GwtArgs | Select-Object -Skip 1) -join ' '

        Set-Location ../{}

        # Run in the background
        Start-Job -ScriptBlock {{
            param($branch, $flags)
            $extra = @()
            if ($flags) {{ $extra = $flags -split ' ' }}
            & gwt-bin remove $branch @extra
        }} -ArgumentList $branch, $flags | Out-Null

        return
    }}

    # Run gwt-bin and change to the returned path
    $path = & gwt-bin @GwtArgs

    if ($null -ne $path) {{
        Set-Location $path.Trim()
    }}
}}

if ($PSVersionTable.PSVersion.Major -ge 6) {{
    $env:COMPLETE = "powershell"
    gwt-bin | Out-String | Invoke-Expression
    Remove-Item Env:\COMPLETE
}} else {{
    {}
}}
            "#,
        default_branch.replace('/', "__"),
        POWERSHELL_5_COMPLETE
    )
}

fn init_bash(default_branch: &str) -> String {
    format!(
        r#"
_gwt_completion() {{
    local current command
    current="${{COMP_WORDS[COMP_CWORD]}}"
    command="${{COMP_WORDS[1]}}"

    case "$command" in
        switch)
            COMPREPLY=(
                $(gwt-bin complete switch "$current" 2>/dev/null)
            )
            ;;
        remove)
            COMPREPLY=(
                $(gwt-bin complete remove "$current" 2>/dev/null)
            )
            ;;
        *)
            local commands="switch list remove sync"
            COMPREPLY=(
                $(compgen -W "$commands" -- "$current")
            )
            ;;
    esac
}}

gwt() {{
    # Pass --help/-h directly to gwt-bin
    if [[ " $* " == *" --help "* || " $* " == *" -h "* ]]; then
        gwt-bin "$@"
        return
    fi

    # Pass --version/-V directly to gwt-bin
    if [[ " $* " == *" --version "* || " $* " == *" -V "* ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt list
    if [[ "${{1:-}}" == "list" ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt sync ...
    if [[ "${{1:-}}" == "sync" ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt complete ...
    if [[ "${{1:-}}" == "complete" ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt remove
    if [[ "${{1:-}}" == "remove" ]]; then
        local branch arg bare=1
        for arg in "${{@:2}}"; do
            [[ "$arg" == -* ]] || {{ bare=0; break; }}
        done

        if [[ $bare -eq 1 ]]; then
            branch="$(git branch --show-current | tr -d '[:space:]')"

            cd ../{} || return

            gwt-bin remove "$branch" "${{@:2}}" &
            return
        fi
    fi

    # Execute gwt-bin, trim whitespace, and cd to the resulting path
    local path
    path="$(gwt-bin "$@" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"

    if [[ -n "$path" ]]; then
        cd "$path" || return
    fi
}}

complete -F _gwt_completion gwt
        "#,
        default_branch.replace('/', "__")
    )
}
