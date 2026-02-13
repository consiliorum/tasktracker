# tasktracker

A command-line task manager built in Rust with SQLite persistence, priority sorting, colored table output, and Markdown export.

## Installation

```bash
cargo install --path .
```

## Usage

### Add a task

```bash
tasktracker add "Buy groceries" -p high --due 2026-03-01
tasktracker add "Read book" -d "Finish chapter 5" -p low
```

Options:
- `-p, --priority` — low, medium, or high (default: medium)
- `-d, --desc` — task description
- `--due` — due date in YYYY-MM-DD format

### List tasks

```bash
tasktracker list                  # pending tasks, sorted by priority
tasktracker list --all            # include completed tasks
tasktracker list --sort due       # sort by due date
tasktracker list --done           # show only completed tasks
```

### Mark a task as done

```bash
tasktracker done 1
```

### Edit a task

```bash
tasktracker edit 1 --title "New title" --priority low --due 2026-04-01
```

### Remove a task

```bash
tasktracker remove 1
```

### Export to Markdown

```bash
tasktracker export                  # print to stdout
tasktracker export --file tasks.md  # save to file
```

## Storage

Tasks are stored in a SQLite database at `~/.tasktracker.db`, created automatically on first run.
