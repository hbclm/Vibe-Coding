use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

mod commands;

struct VibeKitExtension;

impl zed::Extension for VibeKitExtension {
    fn new() -> Self {
        VibeKitExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        // Get command content from embedded commands
        if let Some(content) = commands::get_command_content(&command.name) {
            let label = format!("VibeKit: {}", commands::get_command_description(&command.name));
            
            Ok(SlashCommandOutput {
                text: content.to_string(),
                sections: vec![SlashCommandOutputSection {
                    range: (0..content.len()).into(),
                    label,
                }],
            })
        } else {
            Err(format!(
                "Unknown command: '{}'. Available commands: /api-design, /init, /review, /debug, etc.",
                command.name
            ))
        }
    }
}

zed::register_extension!(VibeKitExtension);
