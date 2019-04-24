# Mei Ling

CLI tool to store ideas/notes (you can use Markdown) on a Git repo.

You need to setup a Git repo (e.g. GitHub) and pass the address on the config file.

# Capabilities
  - 0.0.1 - reads a config file and print out the content - DONE
  - 0.0.2 - reads a config file key and print out the value - DONE
  - 0.0.3 - reads a config file key for git repo and makes a git clone to local
  - 0.0.4 - reads a config file key for git repo and makes a git pull
  - 0.0.5 - reads a config file key for git repo and makes a git push with local content
  - 0.0.6 - reads a config file key for git repo, creates a blank file and push it
  - 0.0.7 - reads a config file key for git repo and show error message if it does not exists

# Run
```sh
$ cargo run
```