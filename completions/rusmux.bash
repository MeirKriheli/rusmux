_rusmux() {
  local cur prev commands aliases projects

  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD - 1]}"

  commands="run stop debug edit delete new list copy doctor"
  aliases="start kill ls"

  if [[ "$prev" == "rusmux" ]]; then
    COMPREPLY=($(compgen -W "$commands $aliases" -- "$cur"))
    return 0
  fi

  projects=$(rusmux list 2>/dev/null)

  case "$prev" in
  run | start | stop | kill | debug | edit | delete)
    COMPREPLY=($(compgen -W "$projects" -- "$cur"))
    return 0
    ;;
  new)
    if [[ "$cur" == --* ]]; then
      COMPREPLY=($(compgen -W "--blank" -- "$cur"))
    else
      COMPREPLY=($(compgen -W "$projects" -- "$cur"))
    fi
    return 0
    ;;
  copy)
    if [[ ${COMP_CWORD} -eq 2 ]]; then
      COMPREPLY=($(compgen -W "$projects" -- "$cur"))
    fi
    return 0
    ;;
  esac
}

complete -F _rusmux rusmux
