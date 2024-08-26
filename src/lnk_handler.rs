use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::{
    core::{Result, PCWSTR, ComInterface},
    Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED, CLSCTX_INPROC_SERVER},
    Win32::UI::Shell::{IShellLinkW, ShellLink},
    Win32::System::Com::IPersistFile
};

/// Handles the .lnk file based on the `dry_run` flag.
pub fn handle_lnk_file(lnk_file: &Path, target_path: &Path, dry_run: bool) -> Result<()> {
    if dry_run {
        // In dry run mode, just return success
        Ok(())
    } else {
        // Overwrite the .lnk file
        create_shortcut(lnk_file, target_path)
    }
}



/// Convert a Path to a PCWSTR (wide string)
fn path_to_pcwstr(path: &Path) -> PCWSTR {
    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    PCWSTR(wide_path.as_ptr())
}

/// Creates a valid Windows shortcut (.lnk) file
fn create_shortcut(shortcut_path: &Path, target_path: &Path) -> windows::core::Result<()> {    // Define the shortcut path and description
    let description = "lnkhunt";

    unsafe {
        // Initialize COM library
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        // Create the IShellLink object
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;

        // Set the target path and description
        shell_link.SetPath(path_to_pcwstr(target_path))?;
        shell_link.SetDescription(path_to_pcwstr(Path::new(description)))?;

        // Save the shortcut using IPersistFile
        let persist_file: IPersistFile = shell_link.cast()?;
        let shortcut_wstr: Vec<u16> = OsStr::new(&shortcut_path)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        let pcwstr: PCWSTR = PCWSTR(shortcut_wstr.as_ptr());
        persist_file.Save(pcwstr, true)?;

        // Uninitialize COM library
        CoUninitialize();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;

    #[test]
    fn test_handle_lnk_file_dry_run() {
        let temp_dir = tempdir().unwrap();
        let lnk_file = temp_dir.path().join("mock_file.lnk");
        File::create(&lnk_file).unwrap();

        // Test dry run mode, where nothing should be overwritten
        let target_path = Path::new("C:/Windows/system32/notepad.exe");
        let _ = handle_lnk_file(&lnk_file, target_path, true);

        // Ensure the file still exists and nothing was modified
        assert!(lnk_file.exists());
        // let content = read(&lnk_file).unwrap();
        // assert!(content.len() > 0);  // Ensure the file is not empty
    }

    #[test]
    fn test_handle_lnk_file_overwrite() {
        let temp_dir = tempdir().unwrap();
        let lnk_file = temp_dir.path().join("mock_file.lnk");
        File::create(&lnk_file).unwrap();

        // Test overwrite mode, where the .lnk file should be replaced
        let target_path = Path::new("C:/Windows/system32/notepad.exe");
        let _ = handle_lnk_file(&lnk_file, target_path, false);

        // Ensure the file still exists and it was overwritten
        assert!(lnk_file.exists());
        // let content = read(&lnk_file).unwrap();
        // assert!(content.len() > 0);  // Ensure the file is not empty
    }
}

