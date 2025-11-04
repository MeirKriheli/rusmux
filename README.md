# Rusmux - tmux automation

The main purpose of this project is to serve as a substitute for [tmuxinator](https://github.com/tmuxinator/tmuxinator),
while still utilizing its yaml files.

`tmuxinator`, which is written in Ruby, has had instances of breaking due to
updates in packages/gems, having me scrambling for a solution.

Furthermore, the process of installing it on different servers that lack a Ruby
installation proved to be tedious.

Hence, [rusmux](https://github.com/MeirKriheli/rusmux) was created. It served
as a chance for me to:

- Utilize the existing `.yml` project files from `tmuxinator`.
- Generate a standalone binary that can be easily distributed.
- Enhance my skills and comprehension of [Rust](https://www.rust-lang.org/) and
  the associated tools.

## Example

```sh
cat ~/.config/rusmux/demo.yml
```

```yaml
project_name: demo # can also use name
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
      root: ~/src/demo/code # Optional per window root overriding `project_root`
      options: # per window options
        main-pane-width: 60%
      panes:
        -  #
        - grunt serve
        - remote-log: # name is meaningless, for multi commands
            - ssh me@example.com
            - cd /var/logs
            - tail -f project.log
  - mail: python -m smtpd -n -c DebuggingServer localhost:1025
```

## Install

- Arch Linux users can install [rusmux from AUR](https://aur.archlinux.org/packages/rusmux),
  e.g. (using `paru`):

  ```sh
  paru -S rusmux
  ```

- Get a compiled binary from the
  [Releases](https://github.com/MeirKriheli/rusmux/releases) page and place it
  in your `$PATH`.

- With `cargo`:

  ```sh
  cargo install rusmux
  ```

- With shell script, for the specific version, see the
  [release page](https://github.com/MeirKriheli/rusmux/releases/latest/), e.g:

  ```sh
  curl --proto '=https' --tlsv1.2 -LsSf https://github.com/MeirKriheli/rusmux/releases/download/v0.x.y/rusmux-installer.sh | sh
  ```

## Commands

- Run a project

  ```sh
  rusmux run [project]
  rusmux start [project]
  ```

  A path to a `yaml` formatted file containing the project definition can be
  specified as well:

  ```sh
  rusmux run ~/projects/my_project/session.yaml
  ```

- List all existing projects

  ```sh
  rusmux list
  rusmux ls
  ```

- Output shell commands for a project

  ```sh
  rusmux debug [project]
  ```

- Edit an existing project (`$EDITOR` should be set)

  ```sh
  rusmux edit [project]
  ```

- Stop project's session

  ```sh
  rusmux stop [project]
  rusmux kill [project]
  ```

- Delete an existing project (after confirmation)

  ```sh
  rusmux delete [project]
  ```

- Create a new project, and open it in an editor (`$EDITOR` should be set)

  ```sh
  rusmux new [project]
  ```

  This creates the project from default template. To create one with just the
  project name:

  ```sh
  rusmux new [project] --blank
  ```

- Copy an existing project to a new one and edit it (`$EDITOR` should be set)

  ```sh
  rusmux copy [existing] [new]
  ```

- Check your environment for potential issues

  ```sh
  rusmux doctor
  ```

  Checks for `tmux` in `$PATH` and `$SHELL` & `$EDITOR` are set.

> **NOTE** In the commands above, `project`, `existing` and `new` can be:
>
> - A simple name, like `my_project`, `awesome_server`, which will be created
>   with a `yaml` extension in the config directory.
> - A path to a file (determined by an extension and/or path separator), e.g. `~/projects/my_project/session.yaml`.

## Shell completion

Under the `completions` directory you'll find the completion scripts for
`bash`, `zsh` and `fish`.

Copy them to the relevant directories for your shell and OS. For system wide
completions (Arch Linux in this example):

```sh
sudo cp completions/rusmux.bash /usr/share/bash-completion/completions/rusmux
sudo cp completions/rusmux.zsh /usr/share/zsh/site-functions/_rusmux
sudo cp completions/rusmux.fish /usr/share/fish/vendor_completions.d/rusmux.fish
```

For user directories, see your shell documentation.
