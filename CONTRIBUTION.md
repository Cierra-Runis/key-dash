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

    Crossterm[""`Crossterm works with:
    - bash
    - zsh
    - fish
    - cmd
    - powershell
    - WSL
    - etc.`""]
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
