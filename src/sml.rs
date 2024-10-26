use zed_extension_api::serde_json;
use zed_extension_api::{self as zed, Result};

struct SMLExtension {}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct MilletInitOptions {
    token_hover: bool,
    fs_watcher: bool,
    format: String, // "smlfmt", "naive", "none"
    diagnostics: Diagnostics,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Diagnostics {
    on_change: bool,
    more_info_hint: bool,
    ignore: String, // "none", "after-syntax", "all"
}

impl zed::Extension for SMLExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
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

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let initialization_options = MilletInitOptions {
            token_hover: true,
            fs_watcher: true,
            format: "none".to_owned(),
            diagnostics: Diagnostics {
                on_change: true,
                more_info_hint: true,
                ignore: "none".to_owned(),
            },
        };

        Ok(Some(serde_json::json!(initialization_options)))
    }
}

zed::register_extension!(SMLExtension);
