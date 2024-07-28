use std::fs;

use zed::{LanguageServerId, Result};
use zed_extension_api as zed;

struct NesFabExtension {
    cached_binary_path: Option<String>,
}

impl NesFabExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("nesfab-language-server") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "hnd2/nesfab-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "nesfab-language-server-{version}-{os}-{arch}.{extension}",
            version = release.version,
            os = match platform {
                zed::Os::Mac => "darwin",
                _ => return Err(format!("unsupported platform {platform:?}")),
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "x64",
                _ => return Err(format!("unsupported architecture {arch:?}")),
            },
            extension = match platform {
                zed::Os::Mac => "tar.gz",
                _ => return Err(format!("unsupported platform {platform:?}")),
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("nesfab-language-server-{}", release.version);
        let binary_path = format!(
            "{version_dir}/bin/nesfab-language-server{extension}",
            extension = match platform {
                zed::Os::Mac => "",
                _ => return Err(format!("unsupported platform {platform:?}")),
            }
        );

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac => zed::DownloadedFileType::GzipTar,
                    _ => return Err(format!("unsupported platform {platform:?}")),
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for NesFabExtension {
    fn new() -> Self {
        NesFabExtension {
            cached_binary_path: None,
        }
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(_language_server_id, _worktree)?,
            args: Default::default(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(NesFabExtension);
