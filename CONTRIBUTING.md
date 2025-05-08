# Contributing

It's a Working in Progress project introduction, which include architectures and design patterns we used or not, and their reasons.

## Rust TUI Tutorial

[Rust TUI Tutorial: Ratatui, Multithreading, and Responsiveness - Youtube](https://www.youtube.com/watch?v=awX7DUp-r14)

### Ratatui Architecture

```mermaid
flowchart TD
    Ratatui
    -- "Layout Instructions<br>Content Rendering<br>Styling Information" --> TerminalBackend["Terminal Backend (Crossterm)"]
    -- "Draw UI<br>Forward Input Events<br>Manage Terminal Modes" --> Terminal["Terminal<br>Linux/Windows/macOS"]

    Devices["Mouse<br>Keyboard"] -- "User Input" --> TerminalBackend

    Crossterm["`Crossterm works with:
    - bash
    - zsh
    - fish
    - cmd
    - powershell
    - WSL
    - etc.`"]
```

### Event Driven Architecture

```mermaid
flowchart
    mspc["mspc channel"]
    TUI.Thread["TUI Thread<br>blocks on channel, then re-renders"]
    Compute.Thread["Compute Thread"] -- "Event::Progress(f64)" --> mspc --> TUI.Thread
    Devices["Mouse<br>Keyboard"] --> Input.Thread["Input Thread<br>event::read()"] -- "Event::Input(KeyEvent)" --> mspc
```

- Advantages:
  - Re-renders only occurs when necessary
  - Better separation of concerns
- Disadvantages:
  - More complex implementation
  - Use of one additional thread

### With Async

```rust
use chrono::{DateTime, Utc};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};
use std::{process, time::Duration};
use tokio::sync::mpsc;

enum Message {
    Exit,
    Timer(DateTime<Utc>),
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let first = tx.clone();
    let second = tx.clone();

    tokio::spawn(async move {
        loop {
            let crossterm_event = crossterm::event::read().unwrap();
            match crossterm_event {
                CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => first.send(Message::Exit).await.unwrap(),
                            KeyCode::Char(char) => println!("Key {char} Pressed"),
                            _ => println!("Other KeyPress"),
                        }
                    }
                }
                _ => println!("Other CrosstermEvent"),
            }
        }
    });

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            second.send(Message::Timer(Utc::now())).await.unwrap();
        }
    });

    while let Some(message) = rx.recv().await {
        match message {
            Message::Timer(date_time) => println!("{date_time}"),
            Message::Exit => process::exit(0),
        }
    }
}
```

## Tokio

- [Tokio internals: Understanding Rust's asynchronous I/O framework from the bottom up](https://cafbit.com/post/tokio_internals)
- [【譯】Tokio 內部機制：從頭理解 Rust 非同步 I/O 框架](https://gist.github.com/ckaznable/80d1925e8ae88f1e9fd8eac70807b5d2)

```mermaid
flowchart TD
  run(["Core::run(future)"]) --> main["Spawn a task for the provided future. This is the "main task""]
  --> poll["Poll the main task and its future"] --> future.ready

  future.ready{"Did the future return Ready(_)?"} -- "yes" --> return(["Return the result"])
  future.ready -- "no" --> cal.timeout["Calculate a timeout duration base on the next timeout event, if any."] --> call.mio["Call mio::Poll:poll(). This blocks until events are available, or the timeout duration (if provided) has elapsed."] --> handle.due["Handle any timeouts that due."] --> events.process

  events.process{"Are there events to process?"} -- "yes" --> process["Process the next event."] --> type
  events.process -- "no" --> token.future

  token.future{"Was TOKEN_FUTURE received?"} -- "yes" --> poll
  token.future -- "no" --> cal.timeout

  type{"What type of event is this?"} --> type.messages & type.future & type.io & type.spawned --> events.process

  type.messages["TOKEN_MESSAGES: Clear readiness, and process the message."]
  type.future["TOKEN_FUTURE": Clear readiness, and note the event.]
  type.io["I/O Source Ready: Notify the I/O reader and/or writer task."]
  type.spawned["Spawned Task Ready: Poll the associated task."]
```

## Learning Source

- [Rust 语言圣经](https://course.rs)
- [Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn)
