# ssh_grades
## Hey guys, my name is Lukas (ldutrie@purdue.edu), let me know if this tool helped you at all. 

## Dependencies

- Rust: This project is written in Rust. You'll need the Rust toolchain, which can be installed using [rustup](https://rustup.rs/).
- `ssh2` system dependencies:
    - On Debian/Ubuntu: `sudo apt-get install -y libssl-dev pkg-config`
    - On macOS: `brew install openssl`
    - On Other Linux Distros: hopefully you can figure it out lol.

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/ssh-grades.git
   cd ssh-grades
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```
   The executable will be located at `target/release/ssh-grades`.

## Adding to PATH

For ease of use, you can add the `ssh-grades` executable to your system's PATH.

### Linux/macOS

1. **Move the binary to a common location:**
   ```bash
   sudo mv target/release/ssh-grades /usr/local/bin/
   ```

2. **Ensure `/usr/local/bin` is in your PATH.** You can check this by running `echo $PATH`. If it's not there, you can add it to your shell's configuration file (e.g., `~/.bashrc`, `~/.zshrc`):
   ```bash
   echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

### Windows

1. **Create a directory for your executables, for example, `C:\Users\YourUser\bin`.**

2. **Move the `ssh-grades.exe` file from `target\release\ssh-grades.exe` to the directory you just created.**

3. **Add the directory to your PATH:**
   - Search for "Environment Variables" in the Start Menu and select "Edit the system environment variables".
   - Click on the "Environment Variables..." button.
   - In the "System variables" section, find the `Path` variable and click "Edit...".
   - Click "New" and add the path to your executables directory (e.g., `C:\Users\YourUser\bin`).
   - Click "OK" on all windows to save the changes.

## Using ssh-grades
