// Cangjie Language Extension for Zed
// LSP server path is configured by user in Zed settings

use std::collections::HashMap;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

const LANGUAGE_SERVER_ID: &str = "cangjie-lsp";

struct CangjieExtension;

impl zed::Extension for CangjieExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err(format!(
                "Unsupported language server ID: {}",
                language_server_id.as_ref()
            )
            .into());
        }

        // Read user settings
        let config = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map_err(|e| format!("Failed to read LSP settings: {}", e))?;

        let settings = config
            .settings
            .as_ref()
            .ok_or("Cangjie LSP settings not configured. Please add 'lsp.cangjie-lsp.binary.path' in your Zed settings.json")?;

        // Get LSP binary path from user settings
        let lsp_path = settings
            .get("binary")
            .and_then(|v| v.get("path"))
            .and_then(|v| v.as_str())
            .ok_or("Missing 'lsp.cangjie-lsp.binary.path' in Zed settings. Please configure the path to cangjie-lsp executable.")?;

        Ok(zed::Command {
            command: lsp_path.to_string(),
            args: vec![],
            env: HashMap::new().into_iter().collect(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            Ok(settings) => Ok(settings.settings),
            Err(_) => Ok(None),
        }
    }
}

zed::register_extension!(CangjieExtension);
