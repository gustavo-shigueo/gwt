def "nu-complete gwt" [spans: list<string>] {
    let args = ($spans | skip 1)
    let current = ($args | last | default "")
    let command = ($args | first)

    match $command {
        "switch" => {
            ^gwt-bin complete switch $current | lines
        }

        "remove" => {
            ^gwt-bin complete remove $current | lines
        }

        _ => (["switch" "list" "remove" "sync"] | where (str starts-with $current))
    }
}

@complete 'nu-complete gwt'
def --env --wrapped gwt [...args] {
    if ("--help" in $args) or ("-h" in $args) {
        ^gwt-bin ...$args
        return
    }

    if ("--version" in $args) or ("-V" in $args) {
        ^gwt-bin ...$args
        return
    }

    if ($args | first) == "list" {
        ^gwt-bin ...$args
        return
    }

    if (($args | first) == "sync") {
        gwt-bin ...$args

        return
    }

    if (($args | first) == "complete") {
        gwt-bin ...$args

        return
    }

    if (($args | first) == "remove") and ((($args | skip 1) | all {|arg| $arg | str starts-with "-"})) {
        let branch = (git branch --show-current | str trim)
        let flags = ($args | skip 1)
        cd '../_GWT_DEFAULT_BRANCH_'

        job spawn {
            gwt-bin remove $branch ...$flags
        }

        return
    }

    let path = (gwt-bin ...$args | str trim)
    cd $path
}

