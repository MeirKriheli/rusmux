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

## TODO

- [ ] Check if session exists and generate different commands
- [X] Add example to README
- [ ] Implement more events
- [ ] Implement more `tmuxinator` commands
