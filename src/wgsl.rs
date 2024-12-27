use zed_extension_api::{self as zed, Result};

struct WgslExtension;

impl zed::Extension for WgslExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("wgsl_analyzer")
            .ok_or_else(|| "wgsl_analyzer not in path.")?;
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(WgslExtension);
