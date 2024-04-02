# Rusmux - tmux automation

Built primarily to replace 
[tmuxinator](https://github.com/tmuxinator/tmuxinator) while using it's
yaml files.


`tmuxinator` is written in Ruby, and various packages/gem upgrades broke
it occasionally, having me scrambling for a fix.

On top of that, installing it on various servers missing Ruby installation was
a chore.

Thus [rusmux](https://github.com/MeirKriheli/rusmux) was born. I've used it
as an opportunity to:

* Work with the existing `.yml` project files of `tmuxinator`.
* Provide a single binary I can copy around.
* Further my knowledge and understanding of [Rust](https://www.rust-lang.org/)
  and the tooling around it.

## Example

```sh
cat ~/.config/rusmux/demo.yml
```

```yaml
project_name: demo  # can also use name
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
        - remote-log: # name is meaningless, for multi commands
          - ssh me@example.com
          - cd /var/logs
          - tail -f project.log 
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


> **NOTE** In the commands above, `project`, `existing` and `new` can be:
>
> * A simple name, like `my_project`, `awesome_server`, which will be created with a `yaml` extension in the config directory.
> * A path to a file (determined by an extension and/or path separator), e.g. `~/projects/my_project/session.yaml`.
