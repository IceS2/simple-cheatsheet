# Simple Cheatsheet

**Obs.: This CLI Tool is in a really early stage.**

Small, Straightforward CLI Tool to interact with your own Cheatsheets o/

# How to Install

## Using Cargo

```bash
cargo install simple-cheatsheet
```

# Settings

You are able to configure three different settings by using environment variables:

- **SCHEAT_CHEATSHEETS_PATH** -> Path where you will store the Cheatsheets
- **SCHEAT_WIDTH**            -> Total Width to be used when displaying in terminal
- **SCHEAT_EDITOR**           -> Editor to use when editing Cheatsheets

## Defaults

- Cheatsheet Path -> `$HOME/.cheatsheets`
- Width           -> `80`
- Editor          -> `$EDITOR`

# How to Use

## 1. Define a Cheatsheet

A **Cheatsheet** contains **Sections**.
A **Section** contains **Cheats**.

To define a Cheatsheet, you need to specify a `yaml` file with the following format:

`cheatsheet.yaml`
```yaml
<Section Name>:
  - description: <Command Description>
    command: <Command>
```

Example:

`nvim.yaml`
```yaml
Tree:
  - description: Toggle Tree
    command: <leader>nt
  - description: Add File
    command: a
```

## 2. Use the CLI

### List all Cheatsheets

```bash
scheat list
```

### Show a Cheatsheet

```bash
scheat show <CHEATSHEET>
```

```bash
scheat show <CHEATSHEET> -s <SECTION>
```

### Edit a Cheatsheet

```bash
scheat edit <CHEATSHEET>
```

# Roadmap

 [X] Define own Cheatsheets in Yaml
 [X] List Cheatsheets
 [X] Display a Cheatsheet
 [X] Display a Section of a Cheatsheet
 [X] Edit a Cheatsheet
 [] Proper Testing :sweaty_smile:
 [] Decent Documentation :sweaty_smile:
 [] Create a new Cheatsheet
 [] Define own Cheatsheets in a less verbose format
