# proj-cmd

A simple project organizer written in rust using clap.

## Installation

### Step 1
install the `proj-cmd` command -:
```zsh
cargo install proj-cmd
```

### Step 2

Add the following to your shell's config file -:
```zsh
eval "$(proj-cmd init zsh)"
```

> The init command currenly supports `bash`, `zsh` & `fish`

> By default, the init command generates a function named `proj`, you can pass in your own as  `eval proj-cmd init zsh my-cmd`

