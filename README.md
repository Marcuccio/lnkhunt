# lnkhunt

`lnkhunt` is a Rust-based command-line tool designed for searching, listing, and overwriting Windows `.lnk` (shortcut) files in large directory structures. Originally inspired by a Capture the Flag (CTF) challenge, where `.lnk` files were exploited to escalate privileges, `lnkhunt` automates the process of manipulating `.lnk` files to target specific executables or paths. This tool is especially useful for security researchers and penetration testers who need to efficiently identify and modify shortcut files during an engagement.

## Features

- **Efficient Scanning**: Recursively scans directories for `.lnk` files.
- **Writable Check**: Automatically checks whether `.lnk` files are writable before attempting to modify them.
- **Dry Run Mode**: Lists `.lnk` files without modifying them, perfect for gathering information during reconnaissance.
- **Overwrite Mode**: Replaces existing `.lnk` files with new shortcuts pointing to a specified target, useful for testing privilege escalation scenarios.

## Installation

### Prerequisites
Ensure you have Rust installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Building the Project

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/your-username/lnkhunt.git
cd lnkhunt
cargo build --release
```

### Usage

```powershell
# dryrun for recon phase
lnkhunt.exe -d C:\Windows\system32\notepad.exe
# recursevely tamper lnk files from "C:\Common Files\"
lnkhunt.exe C:\Windows\system32\payload.exe --search-dir "C:\Common Files\"
```


# More
```powershell
# create an lnk
$shortcutPath = "$HOME\whiterabbit.lnk";$targetPath = "C:\Windows\system32\calc.exe";$WshShell = New-Object -ComObject WScript.Shell;$Shortcut = $WshShell.CreateShortcut($shortcutPath);$Shortcut.TargetPath = $targetPath;$Shortcut.IconLocation = "C:\Windows\system32\calc.exe,0";$Shortcut.Save()
```

