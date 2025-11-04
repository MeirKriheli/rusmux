#compdef rusmux

_rusmux() {
  local -a commands
  commands=(
    'run:Run the project’s commands (alias: start)'
    'stop:Stop the project’s session (alias: kill)'
    'debug:Output shell commands for a project'
    'edit:Edit an existing project'
    'delete:Delete an existing project'
    'new:Create a new project'
    'list:List all projects in the config directory (alias: ls)'
    'copy:Copy an existing project to a new one and edit it'
    'doctor:Check your environment’s configuration'
  )

  _arguments \
    '1:command:_rusmux_commands' \
    '*::args:_rusmux_args'
}

_rusmux_commands() {
  _describe -t commands 'command' commands
}

_rusmux_args() {
  local -a projects
  projects=("${(@f)$(rusmux list)}")

  case $words[1] in
    run|start|stop|kill|debug|edit|delete)
      _arguments '1:project:(${projects})'
      ;;
    new)
      _arguments \
        '1:project:' \
        '--blank[Don’t use a template for the file]'
      ;;
    copy)
      _arguments '1:existing project:(${projects})' '2:new project:'
      ;;
  esac
}

compdef _rusmux rusmux

