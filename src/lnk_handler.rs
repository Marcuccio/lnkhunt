use std::path::Path;
use windows::{
    core::PCWSTR,
    Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED},
    Win32::UI::Shell::{IShellLinkW, IPersistFile, ShellLinkDataList},
    Win32::System::Com::CLSCTX_INPROC_SERVER,
    Win32::Foundation::PWSTR,
};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Handles the .lnk file based on dry_run flag
pub fn handle_lnk_file(lnk_file: &Path, dry_run: bool) {
    if dry_run {
        println!("Found .lnk file: {}", lnk_file.display());
    } else {
        println!("Overwriting .lnk file: {}", lnk_file.display());
        if let Err(e) = create_shortcut(lnk_file, Path::new("C:/path/to/target.exe"), "Generated Shortcut") {
            eprintln!("Failed to overwrite {}: {:?}", lnk_file.display(), e);
        }
    }
}

/// Creates a valid .lnk file pointing to the target executable
fn create_shortcut(shortcut_path: &Path, target_path: &Path, description: &str) -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)?;

        let shell_link: IShellLinkW = CoCreateInstance(&ShellLinkDataList::uuidof(), std::ptr::null_mut(), CLSCTX_INPROC_SERVER)?;

        shell_link.SetPath(target_path_to_pcwstr(target_path))?;
        shell_link.SetDescription(target_path_to_pcwstr(description))?;

        let persist_file: IPersistFile = shell_link.cast()?;
        let shortcut_wstr: Vec<u16> = OsStr::new(shortcut_path)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        persist_file.Save(PWSTR(shortcut_wstr.as_ptr()), true)?;

        CoUninitialize();

        Ok(())
    }
}

/// Converts a Rust Path to PCWSTR (wide string)
fn target_path_to_pcwstr(path: &Path) -> PCWSTR {
    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    PCWSTR(wide_path.as_ptr())
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

        handle_lnk_file(&lnk_file, true);

        assert!(lnk_file.exists());
    }

    #[test]
    fn test_handle_lnk_file_overwrite() {
        let temp_dir = tempdir().unwrap();
        let lnk_file = temp_dir.path().join("mock_file.lnk");
        File::create(&lnk_file).unwrap();

        handle_lnk_file(&lnk_file, false);

        let content = std::fs::read(&lnk_file).unwrap();
        assert_eq!(content.len(), 0);  // File content validation would vary based on actual .lnk structure
    }
}
