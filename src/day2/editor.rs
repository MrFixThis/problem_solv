use std::{io, ops::Range};

enum UndoTask {
    Write(Range<usize>),
    Delete(String),
}

pub fn txt_ed() -> io::Result<()> {
    let mut s = String::new();
    let mut uq: Vec<UndoTask> = Vec::new();

    let r#in = io::read_to_string(io::stdin())?;
    let r#in = r#in.trim();

    r#in.lines()
        .skip(1)
        .map(|v| {
            let mut sp = v.split(' ');
            // SAFETY: it's guaranted that always exist an operation and it is valid
            (sp.next().unwrap(), sp.next())
        })
        .for_each(|ins| match ins {
            ("1", Some(w)) => {
                uq.push(UndoTask::Write(s.len()..w.len()));
                s.push_str(w);
            }
            (op, Some(k)) => {
                let k: usize = k.parse().unwrap();
                if op == "3" {
                    return println!("{}", s.chars().nth(k - 1).unwrap());
                }

                uq.push(UndoTask::Delete(s[s.len() - k..].to_owned()));
                s.replace_range(s.len() - k.., "")
            }
            _ => match uq.pop() {
                Some(qt) => match qt {
                    UndoTask::Write(rng) => s.replace_range(rng, ""),
                    UndoTask::Delete(txt) => s.push_str(&txt),
                },
                _ => (),
            },
        });

    Ok(())
}
