# Fish completion script for rusmux

# Function to get project list from rusmux list
function __rusmux_projects
    rusmux list 2>/dev/null | string trim
end

# Main command completion
complete -c rusmux -f # Disable file completion by default

# Subcommands
complete -c rusmux -n __fish_use_subcommand -a run -d "Run the project's commands"
complete -c rusmux -n __fish_use_subcommand -a start -d "Alias for run"
complete -c rusmux -n __fish_use_subcommand -a stop -d "Stop the project's session"
complete -c rusmux -n __fish_use_subcommand -a kill -d "Alias for stop"
complete -c rusmux -n __fish_use_subcommand -a debug -d "Output shell commands for a project"
complete -c rusmux -n __fish_use_subcommand -a edit -d "Edit an existing project"
complete -c rusmux -n __fish_use_subcommand -a delete -d "Delete an existing project"
complete -c rusmux -n __fish_use_subcommand -a new -d "Create a new project"
complete -c rusmux -n __fish_use_subcommand -a list -d "List all projects in the config directory"
complete -c rusmux -n __fish_use_subcommand -a ls -d "Alias for list"
complete -c rusmux -n __fish_use_subcommand -a copy -d "Copy an existing project to a new one and edit it"
complete -c rusmux -n __fish_use_subcommand -a doctor -d "Check your environment's configuration"

# Command-specific completions using rusmux list output
# run/start - requires project argument from rusmux list
complete -c rusmux -n "__fish_seen_subcommand_from run start" -r -a "(__rusmux_projects)"

# stop/kill - requires project argument from rusmux list
complete -c rusmux -n "__fish_seen_subcommand_from stop kill" -r -a "(__rusmux_projects)"

# debug - requires project argument from rusmux list
complete -c rusmux -n "__fish_seen_subcommand_from debug" -r -a "(__rusmux_projects)"

# edit - requires project argument from rusmux list
complete -c rusmux -n "__fish_seen_subcommand_from edit" -r -a "(__rusmux_projects)"

# delete - requires project argument from rusmux list
complete -c rusmux -n "__fish_seen_subcommand_from delete" -r -a "(__rusmux_projects)"

# new - requires a new project name (no specific completions), has --blank flag
complete -c rusmux -n "__fish_seen_subcommand_from new" -r -d "New project name"
complete -c rusmux -n "__fish_seen_subcommand_from new" -l blank -d "Don't use a template for the file"

# copy - requires existing project from rusmux list and new project name
complete -c rusmux -n "__fish_seen_subcommand_from copy" -r -a "(__rusmux_projects)" -d "Existing project name from rusmux list"
complete -c rusmux -n "__fish_seen_subcommand_from copy; and __fish_prev_arg_in (__rusmux_projects)" -r -d "New project name"

# Version and help
complete -c rusmux -s v -l version -d "Show version information"
complete -c rusmux -s h -l help -d "Show help information"
