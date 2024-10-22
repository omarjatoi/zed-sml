use zed_extension_api::{self as zed, Result};

struct SMLExtension;

impl zed::Extension for SMLExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let millet_cmd = worktree.which("millet-ls");
        let path = millet_cmd.ok_or_else(|| "millet-ls must be in your path".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SMLExtension);
