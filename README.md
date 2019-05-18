# Mei Ling

CLI tool to store ideas as notes (Markdown) that uses a Git repository as storage backend.

You need to setup a Git repo (e.g. GitHub) and pass the address on the config file.

# Capabilities
  - 0.0.1 - reads a config file and print out the content - DONE
  - 0.0.2 - reads a config file key and print out the value - DONE
  - 0.0.3 - reads a config file key for git repo and makes a git clone to local - DONE
  - 0.0.4 - reads a config file key for git repo and makes a git pull - DONE
  - 0.0.5 - reads a command line argument and create an empty file on the local repo - DONE
  - 0.0.6 - reads a command line argument and create an empty file on the local repo and opens vim to write to this file - DONE
  - 0.0.7 - reads a config file key for git repo and makes a git push with local content - DONE
  - 0.0.8 - reads a config file key for git repo, creates a blank file and push it - DONE
  - 0.0.9 - reads a config file key for git repo and show error message if it does not exists
  - 0.1.0 - reads a command line argument and open a file with vim - DONE
  - 0.1.1 - reads a command line argument and gives the status of the repository (just like git status) - DONE
  - 0.1.2 - reads a command line argument, adds all the current changes and pushes to the repository - DONE
  - 0.1.3 - reads a command line argument and pull the repository remote state - DONE

}

# Run
```sh
$ cargo run
```
