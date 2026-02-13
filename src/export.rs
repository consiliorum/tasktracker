use crate::models::{Priority, Task};

pub fn to_markdown(tasks: &[Task]) -> String {
    let mut out = String::from("# Tasks\n\n");

    for (label, priority) in [("High Priority", Priority::High), ("Medium Priority", Priority::Medium), ("Low Priority", Priority::Low)] {
        let group: Vec<&Task> = tasks.iter().filter(|t| t.priority == priority).collect();
        if group.is_empty() {
            continue;
        }
        out.push_str(&format!("## {}\n\n", label));
        for task in &group {
            let checkbox = if task.done { "x" } else { " " };
            let due = task
                .due_date
                .as_deref()
                .map(|d| format!(" (due: {})", d))
                .unwrap_or_default();
            out.push_str(&format!("- [{}] {}{}\n", checkbox, task.title, due));
            if let Some(desc) = &task.description {
                if !desc.is_empty() {
                    out.push_str(&format!("  - {}\n", desc));
                }
            }
        }
        out.push('\n');
    }

    out
}
