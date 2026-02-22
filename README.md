# tt

A command-line task manager built in Rust with SQLite persistence, priority sorting, colored table output, and Markdown export.

## Installation

```bash
cargo install --path .
```

## Usage

### Add a task

```bash
tt add "Buy groceries" -p high --due 2026-03-01
tt add "Read book" -d "Finish chapter 5" -p low
```

Options:
- `-p, --priority` — low, medium, or high (default: medium)
- `-d, --desc` — task description
- `--due` — due date in YYYY-MM-DD format

### List tasks

```bash
tt list                  # pending tasks, sorted by priority
tt list --all            # include completed tasks
tt list --sort due       # sort by due date
tt list --done           # show only completed tasks
```

### Mark a task as done

```bash
tt done 1
```

### Edit a task

```bash
tt edit 1 --title "New title" --priority low --due 2026-04-01
```

### Remove a task

```bash
tt remove 1
```

### Export to Markdown

```bash
tt export                  # print to stdout
tt export --file tasks.md  # save to file
```

## Storage

Tasks are stored in a SQLite database at `~/.tasktracker.db`, created automatically on first run.
