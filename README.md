# Rusmux - tmux automation

Built primarily to replace 
[tmuxinator](https://github.com/tmuxinator/tmuxinator) while using it's
yaml files.

## Example

```sh
cat ~/.config/rusmux/demo.yml
```

```yaml
project_name: demo
project_root: ~/src/demo
on_project_start:
  - sudo systemctl start postgresql
  - sudo systemctl start mysqld
# on_project_stop: 
pre_window:
  - workon demo
  - cd demo
windows:
  - editor: vim
  - shells:
      layout: main-vertical
      panes:
        - #
        - grunt serve
  - mail: python -m smtpd -n -c DebuggingServer localhost:1025
```

## Install

* Arch Linux users can install [rusmux from AUR](https://aur.archlinux.org/packages/rusmux),
  e.g. (using `paru`):

        paru -S rusmux

* Get a compiled binary from the
  [Releases](https://github.com/MeirKriheli/rusmux/releases) page and place it
  in your `$PATH`.

* With `cargo`:

        cargo install rusmux


## Commands

* Run a project

        rusmux [project]

  or 

        rusmux run [project]

* List all existing projects

        rusmux list
  
* Output shell commands for a project 

        rusmux debug [project]

* Edit an existing project (`$EDITOR` should be set)

        rusmux edit [project]

* Stop project's session

        rusmux stop [project]

* Delete an existing project (after confirmation)

        rusmux delete [project]

* Create a new project, and open it in an editor (`$EDITOR` should be set)

        rusmux new [project]

  This creates the project from default template. To create one with just the
  project name:

        rusmux new [project] --blank

* Copy an existing project to a new one and edit it (`$EDITOR` should be set)

        rusmux copy [existing] [new]

* Check your environment for potential issues

        rusmux doctor

  Checks for `tmux` in `$PATH` and `$SHELL` & `$EDITOR` are set.
