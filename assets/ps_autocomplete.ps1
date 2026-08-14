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
            @("switch", "remove", "list", "init", "complete") |
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
& "c:\\users\\administrador\\.cargo\\bin\\gwt-bin.exe" -- $args
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

