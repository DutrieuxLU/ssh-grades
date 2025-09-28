# ssh_grades

## Hey guys, my name is Lukas (ldutrie@purdue.edu), let me know if this tool helped you at all. 

⭐ Star the repository on GitHub. This is the easiest way to show your appreciation and helps others discover the project.
Dependencies

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
## Using WSL Might make this easier BTW


1. **Create a directory for your executables, for example, `C:\Users\YourUser\bin`.**


2. **Move the `ssh-grades.exe` file from `target\release\ssh-grades.exe` to the directory you just created.**


3. **Add the directory to your PATH:**

   - Search for "Environment Variables" in the Start Menu and select "Edit the system environment variables".

   - Click on the "Environment Variables..." button.

   - In the "System variables" section, find the `Path` variable and click "Edit...".

   - Click "New" and add the path to your executables directory (e.g., `C:\Users\YourUser\bin`).

   - Click "OK" on all windows to save the changes.


## Using ssh-grades


ssh-grades is designed to be very easy to use for people who are taking CS354 with Dr. Comer in Fall 2025. Assuming that you have everything installed, you will have a few steps on first startup.


1. You will be prompted for your Purdue username and password. These should be the exact same as what you use to login to Data/Xinu. **DO NOT ADD THE ',PUSH' TO YOUR PASSWORD**.

2. If you got your username/password right, you should be prompted by **DUO** on your phone. Accept this confirmation, and you should be up and running! If not, submit an issue or email me.

3. If you think you entered your password incorrectly, doing `ssh-grades reset` will restart your login process.


## Contributing

ssh-grades is a pet project, and has a lot of stuff that can be worked on. If you are interested in Rust or SSH, I would highly encourage you to take a look at one of the issues, and see what you got! 
