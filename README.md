# Raven

Program to watch over a file or folder and execute an action when the file gets changed.

## Usage

The program expects a config file in the path

```sh
~/.config/raven/config.toml
```

Config file structure is as follows

```toml
# File expects an array of `targets` to watch over
# Each target has two entities, path and sync
# `path` is the file/folder to watch for
# `sync` is the action to be taken when a file change occurs
[[targets]]
path = "/Users/user/sample.txt"
sync = { script = "./sync.sh" }
```

Sample sync file

```sh
#!/usr/bin/env sh

git add .
git commit -m "sync: ${date}"
git push
```

Currently sync action is dependent on script file. More modules are planned.

### Note!

This program has been created for exploring rust, it's still in early stages and lot of work is pending before a proper release. There are quite few edge case scenarios pending to be handled. They will be recorded in the README soon.

```sh
▒▒▒▒░░░░░░░░▒▒▒▒░░░░▒▒▒▒░░▒▒░░▒▒░░░░░░░░▒▒▒▒░░░░░░▒▒░░▒▒▒▒▓▓▓▓██▓▓
▓▓▓▓▒▒▒▒░░  ░░░░▒▒░░▒▒▒▒░░  ░░  ░░        ░░▒▒      ░░░░▒▒▒▒▓▓░░▓▓
▓▓▓▓▓▓▓▓▒▒    ░░▒▒▒▒░░░░░░░░                  ░░░░        ▒▒▒▒▓▓▒▒
▓▓▓▓▓▓██▒▒░░░░░░▒▒░░▒▒▒▒░░░░                    ░░░░      ░░  ░░▓▓
▒▒▓▓▒▒▓▓▓▓░░░░▒▒░░▒▒▒▒██▒▒▒▒          ░░░░░░        ▒▒      ░░  ▒▒
▒▒▒▒▓▓░░▒▒▓▓▒▒░░▓▓▒▒▒▒▒▒░░░░▒▒▒▒▓▓▒▒▓▓▓▓▒▒░░████      ░░░░    ░░░░
▒▒▒▒▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░▒▒▒▒▒▒▓▓▒▒▓▓▓▓▒▒▓▓██▒▒    ░░▒▒    ▒▒
▓▓▒▒░░▓▓▒▒░░▒▒▒▒░░░░▒▒▒▒▒▒▒▒▓▓▓▓▒▒▓▓██▒▒██▒▒▓▓▓▓▓▓██▒▒      ▒▒░░░░
▓▓▒▒░░▒▒▓▓▒▒░░▓▓▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒██▒▒▒▒▓▓▓▓▓▓████░░      ▒▒▒▒
▒▒▓▓░░░░▓▓▒▒▒▒▒▒▒▒▒▒░░░░░░▒▒  ▒▒██▓▓▒▒▒▒▓▓▓▓▒▒▒▒▓▓██████        ░░
▓▓▒▒▓▓▓▓░░▓▓▓▓░░▒▒▒▒░░▒▒▒▒▒▒▒▒    ██▓▓▓▓▒▒▒▒▒▒▒▒▒▒▓▓██▓▓░░      ░░
▓▓▒▒▒▒▓▓▓▓▒▒▓▓▒▒▒▒░░▒▒░░░░░░░░▒▒  ██▒▒████▓▓▒▒▓▓██▓▓██▓▓▒▒      ░░
▒▒▓▓▒▒░░▒▒▒▒▒▒░░▒▒░░░░░░      ▒▒▓▓██▒▒▒▒▓▓▓▓▓▓▓▓████████▓▓      ░░
▒▒▒▒▓▓░░▒▒▒▒░░▒▒  ░░  ░░░░  ▓▓▓▓▓▓▒▒▓▓▒▒▒▒░░▒▒▓▓▓▓██▓▓██        ░░
▒▒░░░░▒▒▒▒▒▒░░  ▒▒░░░░  ░░▓▓▓▓▓▓░░░░░░▓▓▒▒▒▒░░██████████        ░░
▒▒░░▒▒░░░░  ░░    ░░    ▓▓▓▓░░  ░░░░▒▒▓▓░░▒▒▒▒▓▓██████▓▓      ░░░░
▒▒▒▒░░░░            ░░▓▓░░▒▒░░░░▒▒░░░░░░░░░░▒▒██████████        ▒▒
▒▒░░░░░░▒▒░░        ░░▒▒▒▒▓▓▒▒░░░░▒▒░░░░▒▒▓▓████████████        ░░
░░  ░░░░  ░░░░      ▓▓▒▒░░░░    ░░▓▓██▒▒▓▓▓▓████████████        ░░
░░            ░░  ░░▓▓▒▒▒▒▒▒▒▒░░▓▓▓▓▓▓████▓▓████████████░░      ░░
░░              ░░▓▓▒▒▓▓▓▓▒▒  ░░▒▒██▓▓████████████████▒▒░░  ░░
░░              ▒▒▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓██████████████████    ░░░░░░░░
░░              ▒▒▒▒██▒▒▓▓▓▓██▓▓▒▒████████████████████        ▒▒▒▒
░░░░          ░░██▓▓▒▒██████▓▓▒▒▒▒██████████████████▓▓          ░░
░░          ░░██▒▒▓▓▒▒▒▒▓▓████▒▒██████████████████████          ░░
░░          ██▓▓▓▓▓▓▒▒██▒▒▒▒▒▒▓▓████████████████████▒▒  ▒▒░░    ░░
░░        ██▓▓▓▓▓▓░░▓▓▒▒▓▓▓▓██▓▓██████████████████▒▒░░    ▒▒▒▒▒▒░░
░░          ▒▒▓▓▓▓▒▒▓▓██▓▓██▒▒▓▓██████████████████▓▓░░      ▒▒▒▒▒▒
░░          ░░▒▒▓▓▒▒████▒▒▓▓▓▓▓▓██████████████████▒▒          ▒▒▒▒
░░          ▒▒▓▓▓▓██████▓▓████████████████████████░░            ░░
░░          ▒▒▓▓████▒▒████▒▒██████████████████▒▒▒▒▒▒            ░░
░░          ▒▒▓▓██▓▓░░▓▓▒▒▓▓██████████████▓▓░░  ▒▒              ░░
░░        ▒▒▒▒▓▓████▒▒▒▒▓▓▓▓██████████████░░░░  ░░    ░░░░  ░░
░░      ░░▓▓██▓▓▓▓██▒▒▓▓██████████▓▓████▓▓              ░░▒▒░░░░░░
░░      ▒▒▒▒██▓▓██▓▓▓▓██████████▓▓░░  ▒▒██                ░░▒▒▒▒░░
░░    ▒▒░░▓▓▓▓██▓▓▒▒██▓▓▒▒██▒▒▓▓░░      ▒▒▒▒                  ▓▓▒▒
░░  ▒▒░░  ▓▓▒▒▓▓▒▒██▓▓▒▒██    ▓▓░░  ░░██████▓▓▒▒▒▒░░░░          ▓▓
░░  ░░  ░░▓▓▒▒▒▒██▓▓▓▓██░░░░▒▒░░▓▓▒▒██████████▓▓▓▓▓▓░░          ░░
░░      ▓▓▓▓▒▒████▒▒▓▓▒▒    ▒▒████▓▓▓▓██▓▓▓▓██▓▓██▓▓▒▒          ░░
░░      ░░▓▓▒▒██▒▒▓▓██      ▓▓▒▒██████████▓▓▓▓▒▒▓▓░░▓▓          ░░
░░      ░░  ▒▒░░▒▒▓▓        ▓▓▒▒▒▒▓▓▓▓██▓▓▓▓▓▓▒▒▒▒▒▒▒▒          ░░
░░      ░░  ▒▒██▒▒          ▓▓░░▓▓▓▓▒▒▒▒▒▒▒▒▒▒░░▒▒▒▒▓▓
░░        ░░▓▓▒▒░░          ▓▓  ▒▒░░▓▓▒▒▒▒░░░░░░░░░░▒▒          ░░
▒▒                          ▓▓░░░░  ░░  ▒▒░░▓▓░░░░░░▒▒          ░░
                            ▓▓░░    ░░▒▒░░▒▒▓▓░░▒▒░░▒▒
░░                          ▒▒░░      ░░░░░░▓▓░░▒▒░░▒▒          ░░
░░                          ▓▓▓▓        ░░  ▓▓░░░░░░▓▓    ░░▒▒  ░░
▒▒░░░░                      ▓▓▒▒░░░░        ▒▒░░▒▒▒▒▒▒    ░░▒▒  ░░
▒▒░░░░░░                    ▓▓▒▒░░  ░░░░  ▒▒░░▓▓░░░░██          ░░
▓▓▓▓░░░░░░  ░░              ▓▓▒▒░░░░          ▓▓▒▒▒▒██      ░░░░░░
▓▓████▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▒▒▒▒▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒

```
