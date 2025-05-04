Rope-Based Text Editor
A lightweight, terminal-based text editor written in Rust, utilizing a rope data structure for efficient text manipulation. This editor is designed for developers who prefer a minimal, keyboard-driven interface with fast performance for handling large text files.
Features

Rope Data Structure: Efficiently handles large text files with low memory overhead and fast insertions/deletions.
Terminal Interface: Built with crossterm for a cross-platform, terminal-based UI.
Keybindings: Intuitive keybindings for common editing tasks (e.g., save, undo, redo, cursor movement).
Help Menu: Interactive help menu displaying all keybindings, accessible via Ctrl+M.
Undo/Redo: Support for undoing and redoing changes.
Custom Filename: Set custom filenames for saving files.
Cross-Platform: Runs on Windows, macOS, and Linux.

Installation
Prerequisites

Rust (stable, version 1.65 or later)
A terminal emulator (e.g., Windows Terminal, iTerm2, or any Linux terminal)
Git (optional, for cloning the repository)

Steps

Clone the Repository:
git clone https://github.com/username/rope-text-editor.git
cd rope-text-editor


Build the Project:
cargo build --release


Run the Editor:
cargo run --release

Alternatively, you can run the built binary directly from target/release/rope-text-editor.


Usage

Launch the Editor:Run the editor using cargo run or the compiled binary.

Edit Text:

Type characters to insert text.
Use arrow keys (Left/Right) to move the cursor.
Press Enter to add a new line.
Use Backspace to delete characters.
Hold Shift while typing to insert uppercase characters.


Access the Help Menu:Press Ctrl+M to open the help menu, which displays all keybindings. Press Esc to return to editing.

Save or Quit:

Press Ctrl+S to save the file.
Press Ctrl+X to set a custom filename.
Press Ctrl+A to quit the editor.


Undo/Redo:

Press Ctrl+Z to undo changes.
Press Ctrl+Y to redo changes.



Keybindings



Keybinding
Action



Ctrl+A
Quit the editor


Ctrl+S
Save the file


Ctrl+M
Open the help menu


Ctrl+X
Set filename


Ctrl+Z
Undo


Ctrl+Y
Redo


Backspace
Delete character


Left/Right
Move cursor


Enter
Insert new line


Shift+Char
Insert uppercase character


Char
Insert character


Esc (in menu)
Return to editing


Project Structure

src/main.rs: Entry point and main application logic.
src/editor.rs: Core editor logic, including rope-based text manipulation and keybinding handling.
src/ui.rs: Terminal UI rendering and help menu implementation.

Dependencies

crossterm: For terminal UI and event handling.
ropey: For the rope data structure used in text manipulation.

Contributing
Contributions are welcome! To contribute:

Fork the Repository:Fork the project on GitHub.

Create a Branch:
git checkout -b feature/your-feature


Make Changes:Implement your feature or bug fix, ensuring code follows Rust conventions.

Run Tests:
cargo test


Submit a Pull Request:Push your changes and create a pull request on GitHub.


Please ensure your code adheres to the Rust Code of Conduct and includes appropriate tests.
License
This project is licensed under the MIT License. See the LICENSE file for details.
Acknowledgments

Inspired by classic terminal-based editors like Vim and Nano.
Built with the Rust community’s amazing ecosystem of crates.


For issues or feature requests, please open an issue on the GitHub repository. Happy editing!
