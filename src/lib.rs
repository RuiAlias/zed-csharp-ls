use zed_extension_api::{self as zed, Result};

struct CsharpExtension {}

impl zed::Extension for CsharpExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("csharp-ls").expect("csharp-ls not found");

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CsharpExtension);
