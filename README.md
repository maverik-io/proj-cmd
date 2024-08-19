# proj-cmd

A simple project organizer written in rust using clap.

## Installation

### Step 1
install the `proj-cmd` command -:

```zsh
cargo install proj-cmd
```

### Step 2


<details>
<summary>zsh</summary>
Add the following to your shell's config file -:

```zsh
eval "$(proj-cmd init zsh)"
```
</details>

<details>
<summary>bash</summary>
Add the following to your shell's config file -:

```
eval "$(proj-cmd init bash)"
```
</details>

<details>
<summary>fish</summary>
Add the following to your shell's config file -:

```
eval "$(proj-cmd init fish)"
```
</details>

<details>
<summary>nu</summary>

run this command `proj-cmd init nu | save -f ~/.proj.nu`
Add the following to your shell's config file -:
```
source ~/.proj.nu
```
</details>

> The init command currenly supports `bash`, `zsh`, `nu` & `fish`

> By default, the init command generates a function named `proj`, you can pass in your own as  `eval proj-cmd init zsh my-cmd`

