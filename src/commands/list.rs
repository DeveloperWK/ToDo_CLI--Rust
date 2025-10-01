use crate::storage::Storage;
use anyhow::Result;

pub fn run(storage: &Storage, show_all: bool, show_completed: bool) -> Result<()> {
    let todos = storage.load()?;
    let iter = todos.iter().filter(|t| {
        if show_all {
            return true;
        }
        if show_completed {
            return t.completed;
        }
        !t.completed
    });
    // for t in iter {
    //     println!(
    //         "{} [{}] - {}",
    //         t.id,
    //         if t.completed { "x" } else { " " },
    //         t.title
    //     );
    //     if let Some(d) = &t.due {
    //         println!("   due: {}", d);
    //     }
    // }

    for t in iter {
        let mut line = format!(
            "{} [{}] - {}",
            t.id,
            if t.completed { "x" } else { " " },
            t.title
        );

        if let Some(d) = &t.due {
            line.push_str(&format!(" (due: {})", d));
        }

        println!("{}", line);
    }

    Ok(())
}
