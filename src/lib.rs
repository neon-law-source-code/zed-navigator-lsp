//! Zed extension shim for `navigator-lsp`.
//!
//! Carries no Navigator source — it only resolves and launches a
//! platform-matching `navigator-lsp` binary from
//! `neon-law-source-code/navigator`'s own GitHub Releases (the
//! `navigator-lsp-<tag>-<platform>` archives `.github/workflows/deploy.yml`
//! attaches there), and points Zed's built-in Markdown language at it.

use zed_extension_api::{self as zed, LanguageServerId, Result};

/// The repository whose GitHub Releases carry the `navigator-lsp` archives.
const NAVIGATOR_REPO: &str = "neon-law-source-code/navigator";

struct NavigatorLspExtension {
    cached_binary_path: Option<String>,
}

impl NavigatorLspExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // A binary already on `$PATH` (a developer's own build, or one
        // fetched with `navigator lsp`) always wins over a downloaded copy.
        if let Some(path) = worktree.which("navigator-lsp") {
            return Ok(path);
        }
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).is_ok() {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            NAVIGATOR_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();
        // Only the three platforms `.github/workflows/deploy.yml` actually
        // builds an archive for — an Intel Mac or ARM Linux navigator-lsp
        // has no published binary to resolve, and this refuses cleanly
        // rather than guessing a near-miss asset name.
        let archive_suffix = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "macos.tar.gz",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux.tar.gz",
            (zed::Os::Windows, zed::Architecture::X8664) => "windows.zip",
            _ => {
                return Err(format!(
                    "navigator-lsp has no published release archive for this platform ({os:?}/{arch:?})"
                ))
            }
        };
        let asset_name = format!("navigator-lsp-{}-{archive_suffix}", release.version);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "navigator-lsp release {} has no asset named `{asset_name}`",
                    release.version
                )
            })?;

        let version_dir = format!("navigator-lsp-{}", release.version);
        let binary_name = if matches!(os, zed::Os::Windows) {
            "navigator-lsp.exe"
        } else {
            "navigator-lsp"
        };
        let binary_path = format!("{version_dir}/{binary_name}");

        if !std::fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let file_type = if archive_suffix.ends_with(".zip") {
                zed::DownloadedFileType::Zip
            } else {
                zed::DownloadedFileType::GzipTar
            };
            zed::download_file(&asset.download_url, &version_dir, file_type)?;
            zed::make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for NavigatorLspExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = self.language_server_binary_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(NavigatorLspExtension);
