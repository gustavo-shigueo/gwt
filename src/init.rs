use crate::command::{InitCommand, Shell};

pub fn init(InitCommand { shell }: InitCommand) {
    let function = match shell {
        Shell::Nu => {
            r#"
def --env --wrapped gwt [...args] {
    if (($args | first) == "list") and (($args | length) == 1) {
        gwt-bin list
        return
    }

    if (($args | first) == "remove") and (($args | length) == 1) {
        let branch = (git branch --show-current | str trim)
        cd ../develop

        job spawn {
            gwt-bin remove $branch
        }

        return
    }

    if ("--help" in $args) or ("-h" in $args) {
        ^gwt-bin ...$args
        return
    }

    let path = (gwt-bin ...$args | str trim)
    cd $path
}
            "#
        }
        Shell::Powershell => {
            r#"
function gwt {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Args
    )

    # Handle: gwt remove
    if ($Args.Count -eq 1 -and $Args[0] -eq "remove") {
        $branch = (git branch --show-current).Trim()

        Set-Location ../develop

        # Run in the background
        Start-Job -ScriptBlock {
            param($branch)
            & gwt-bin remove $branch
        } -ArgumentList $branch | Out-Null

        return
    }

    if ($Args.Count -eq 1 -and $Args[0] -eq "list") {
        & gwt-bin list
        return
    }

    # Handle: gwt --help / -h
    if ($Args -contains "--help" -or $Args -contains "-h") {
        & gwt-bin @Args
        return
    }

    # Run gwt-bin and change to the returned path
    $path = (& gwt-bin @Args).Trim()
    Set-Location $path
}
            "#
        }
    };

    println!("{}", function.trim());
}
