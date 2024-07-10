use zed_extension_api as zed;

struct NesFabExtension;

impl zed::Extension for NesFabExtension {
    fn new() -> Self {
        NesFabExtension {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed::Result<zed::Command> {
        Err("Not implemented".into())
    }
}

zed::register_extension!(NesFabExtension);
