_gwt_completion() {
    local current command
    current="${COMP_WORDS[COMP_CWORD]}"
    command="${COMP_WORDS[1]}"

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
}

gwt() {
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
    if [[ "${1:-}" == "list" ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt sync ...
    if [[ "${1:-}" == "sync" ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt complete ...
    if [[ "${1:-}" == "complete" ]]; then
        gwt-bin "$@"
        return
    fi

    # gwt remove
    if [[ "${1:-}" == "remove" ]]; then
        local branch arg bare=1
        for arg in "${@:2}"; do
            [[ "$arg" == -* ]] || { bare=0; break; }
        done

        if [[ $bare -eq 1 ]]; then
            branch="$(git branch --show-current | tr -d '[:space:]')"

            root="$(git rev-parse --show-toplevel)"
            cd "$root"
            cd '../_GWT_DEFAULT_BRANCH_' || return

            gwt-bin remove "$branch" "${@:2}" &
            return
        fi
    fi

    # Execute gwt-bin, trim whitespace, and cd to the resulting path
    local path
    path="$(gwt-bin "$@" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"

    if [[ -n "$path" ]]; then
        cd "$path" || return
    fi
}

complete -F _gwt_completion gwt

