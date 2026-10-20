function gwt {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$GwtArgs
    )

    # Handle: gwt --help / -h
    if ($GwtArgs -contains "--help" -or $GwtArgs -contains "-h") {
        & gwt-bin @GwtArgs
        return
    }

    # Handle: gwt --version
    if ($GwtArgs -contains "--version" -or $GwtArgs -contains "-V") {
        & gwt-bin @GwtArgs
        return
    }

    if ($GwtArgs[0] -eq "list") {
        & gwt-bin @GwtArgs
        return
    }

    if ($GwtArgs[0] -eq "sync") {
        & gwt-bin @GwtArgs
        return
    }

    if ($GwtArgs[0] -eq "complete") {
        & gwt-bin @GwtArgs
        return
    }

    # Handle: gwt remove
    if ($GwtArgs[0] -eq "remove" -and @($GwtArgs | Select-Object -Skip 1 | Where-Object { $_ -notlike "-*" }).Count -eq 0) {
        $branch = (git branch --show-current).Trim()
        $flags = @($GwtArgs | Select-Object -Skip 1) -join ' '

        Set-Location -LiteralPath '../_GWT_DEFAULT_BRANCH_'

        # Run in the background
        Start-Job -ScriptBlock {
            param($branch, $flags)
            $extra = @()
            if ($flags) { $extra = $flags -split ' ' }
            & gwt-bin remove $branch @extra
        } -ArgumentList $branch, $flags | Out-Null

        return
    }

    # Run gwt-bin and change to the returned path
    $path = & gwt-bin @GwtArgs

    if ($null -ne $path) {
        Set-Location $path.Trim()
    }
}

if ($PSVersionTable.PSVersion.Major -ge 6) {
    $env:COMPLETE = "powershell"
    gwt-bin | Out-String | Invoke-Expression
    Remove-Item Env:\COMPLETE
} else {
  Register-ArgumentCompleter -Native -CommandName gwt -ScriptBlock {
      param($wordToComplete, $commandAst, $cursorPosition)

      if ($wordToComplete -eq "") {
          $elements = @($commandAst.CommandElements | ForEach-Object { $_.SafeGetValue() })

          if ($elements.Count -ge 2) {
              switch ($elements[1]) {
                  "switch" {
                      & gwt-bin complete switch |
                          ForEach-Object {
                              [System.Management.Automation.CompletionResult]::new(
                                  $_,
                                  $_,
                                  [System.Management.Automation.CompletionResultType]::ParameterValue,
                                  $_
                              )
                          }

                      return
                  }

                  "remove" {
                      & gwt-bin complete remove |
                          ForEach-Object {
                              [System.Management.Automation.CompletionResult]::new(
                                  $_,
                                  $_,
                                  [System.Management.Automation.CompletionResultType]::ParameterValue,
                                  $_
                              )
                          }

                      return
                  }
              }
          } else {
              @("switch", "remove", "list", "init", "complete", "sync") |
                  ForEach-Object {
                      [System.Management.Automation.CompletionResult]::new(
                          $_,
                          $_,
                          [System.Management.Automation.CompletionResultType]::ParameterValue,
                          $_
                      )
                  }
              return
          }
      }

      $prev = $env:COMPLETE;
      $env:COMPLETE = "powershell";

      $args = $commandAst.Extent.Text
      $args = $args.Substring(0, [math]::Min($cursorPosition, $args.Length));
      if ($wordToComplete -eq "") {
          $args += " ''";
      }

      $results = Invoke-Expression @"
& "gwt-bin" -- $args
"@;
      if ($null -eq $prev) {
          Remove-Item Env:\COMPLETE;
      } else {
          $env:COMPLETE = $prev;
      }

      $results | ForEach-Object {
          $split = $_.Split("`t");
          $cmd = $split[0];

          if ($split.Length -eq 2) {
              $help = $split[1];
          }
          else {
              $help = $split[0];
          }

          [System.Management.Automation.CompletionResult]::new($cmd, $cmd, 'ParameterValue', $help)
      }
  };
}
