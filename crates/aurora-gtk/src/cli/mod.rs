//! Aurora CLI - Project Setup and Development Tools
//!
//! Command-line interface for creating Aurora projects, scaffolding components, and managing design tokens.

use std::collections::HashMap;

/// CLI command type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    New,
    Add,
    Generate,
    Theme,
    Export,
    Init,
}

impl CommandType {
    pub fn name(&self) -> &str {
        match self {
            CommandType::New => "new",
            CommandType::Add => "add",
            CommandType::Generate => "generate",
            CommandType::Theme => "theme",
            CommandType::Export => "export",
            CommandType::Init => "init",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            CommandType::New => "Create a new Aurora project",
            CommandType::Add => "Add a component to your project",
            CommandType::Generate => "Generate icons, themes, documentation",
            CommandType::Theme => "Customize Aurora theme colors",
            CommandType::Export => "Export icons as SVG, fonts, images",
            CommandType::Init => "Initialize Aurora in existing project",
        }
    }
}

/// CLI argument
#[derive(Debug, Clone)]
pub struct Argument {
    name: String,
    description: String,
    required: bool,
    default_value: std::option::Option<String>,
}

impl Argument {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required: false,
            default_value: std::option::Option::None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn with_default(mut self, value: &str) -> Self {
        self.default_value = std::option::Option::Some(value.to_string());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn is_required(&self) -> bool {
        self.required
    }
    pub fn default_value(&self) -> std::option::Option<&str> {
        self.default_value.as_deref()
    }
}

/// CLI flag/option
#[derive(Debug, Clone)]
pub struct Flag {
    short_flag: std::option::Option<char>,
    long_flag: String,
    description: String,
    has_value: bool,
}

impl Flag {
    pub fn new(long: &str, description: &str) -> Self {
        Self {
            short_flag: std::option::Option::None,
            long_flag: long.to_string(),
            description: description.to_string(),
            has_value: false,
        }
    }

    pub fn with_short(mut self, c: char) -> Self {
        self.short_flag = std::option::Option::Some(c);
        self
    }

    pub fn with_value(mut self) -> Self {
        self.has_value = true;
        self
    }

    pub fn short_flag(&self) -> std::option::Option<char> {
        self.short_flag
    }
    pub fn long_flag(&self) -> &str {
        &self.long_flag
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn has_value(&self) -> bool {
        self.has_value
    }
}

/// CLI command definition
#[derive(Debug, Clone)]
pub struct Command {
    command_type: CommandType,
    arguments: Vec<Argument>,
    flags: Vec<Flag>,
    aliases: Vec<String>,
}

impl Command {
    pub fn new(command_type: CommandType) -> Self {
        Self {
            command_type,
            arguments: Vec::new(),
            flags: Vec::new(),
            aliases: Vec::new(),
        }
    }

    pub fn add_argument(mut self, argument: Argument) -> Self {
        self.arguments.push(argument);
        self
    }

    pub fn add_flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    pub fn add_alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_string());
        self
    }

    pub fn command_type(&self) -> CommandType {
        self.command_type
    }
    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }
    pub fn flags(&self) -> &[Flag] {
        &self.flags
    }
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    pub fn help(&self) -> String {
        let mut help = String::new();
        help.push_str(&format!("aurora {}\n", self.command_type.name()));
        help.push_str(&format!("{}\n\n", self.command_type.description()));

        if !self.arguments.is_empty() {
            help.push_str("ARGUMENTS:\n");
            for arg in &self.arguments {
                let req_str = if arg.is_required() { " (required)" } else { "" };
                help.push_str(&format!("  {}  {}{}\n", arg.name, arg.description, req_str));
            }
            help.push('\n');
        }

        if !self.flags.is_empty() {
            help.push_str("FLAGS:\n");
            for flag in &self.flags {
                if let Some(c) = flag.short_flag() {
                    help.push_str(&format!(
                        "  -{}, --{}  {}\n",
                        c,
                        flag.long_flag(),
                        flag.description()
                    ));
                } else {
                    help.push_str(&format!(
                        "  --{}  {}\n",
                        flag.long_flag(),
                        flag.description()
                    ));
                }
            }
        }

        help
    }
}

/// Aurora CLI
pub struct AuroraCli {
    commands: HashMap<String, Command>,
    version: String,
}

impl AuroraCli {
    pub fn new() -> Self {
        let mut cli = Self {
            commands: HashMap::new(),
            version: "1.1.0".to_string(),
        };
        cli.register_default_commands();
        cli
    }

    fn register_default_commands(&mut self) {
        let new_cmd = Command::new(CommandType::New)
            .add_argument(Argument::new("project-name", "Name of project").required())
            .add_flag(
                Flag::new("template", "Project template")
                    .with_short('t')
                    .with_value(),
            )
            .add_flag(Flag::new("force", "Overwrite existing").with_short('f'));
        self.commands.insert("new".to_string(), new_cmd);

        let add_cmd = Command::new(CommandType::Add)
            .add_argument(Argument::new("component", "Component to add").required())
            .add_flag(Flag::new("dry-run", "Preview changes").with_short('d'));
        self.commands.insert("add".to_string(), add_cmd);

        let gen_cmd = Command::new(CommandType::Generate)
            .add_argument(Argument::new("target", "Generate icons/themes").required())
            .add_flag(
                Flag::new("output", "Output directory")
                    .with_short('o')
                    .with_value(),
            );
        self.commands.insert("generate".to_string(), gen_cmd);

        let theme_cmd = Command::new(CommandType::Theme)
            .add_argument(Argument::new("action", "customize/export/preview").required())
            .add_flag(
                Flag::new("colors", "Color file")
                    .with_short('c')
                    .with_value(),
            );
        self.commands.insert("theme".to_string(), theme_cmd);

        let export_cmd = Command::new(CommandType::Export)
            .add_argument(Argument::new("target", "icons/fonts/tokens").required())
            .add_flag(
                Flag::new("format", "Export format")
                    .with_short('f')
                    .with_value(),
            );
        self.commands.insert("export".to_string(), export_cmd);

        let init_cmd = Command::new(CommandType::Init)
            .add_flag(Flag::new("force", "Reinitialize").with_short('f'));
        self.commands.insert("init".to_string(), init_cmd);
    }

    pub fn register(&mut self, name: &str, command: Command) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn get(&self, name: &str) -> std::option::Option<&Command> {
        self.commands.get(name)
    }

    pub fn commands(&self) -> Vec<&Command> {
        self.commands.values().collect()
    }

    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn help(&self) -> String {
        let mut help = String::from("Aurora v1.1 - GNOME Design System CLI\n\n");
        help.push_str("USAGE:\n  aurora <COMMAND> [ARGS] [OPTIONS]\n\n");
        help.push_str("COMMANDS:\n");
        for (name, cmd) in &self.commands {
            help.push_str(&format!("  {}  {}\n", name, cmd.command_type.description()));
        }
        help.push_str("\nRun 'aurora <COMMAND> --help' for more information.\n");
        help
    }
}

impl Default for AuroraCli {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_type_names() {
        assert_eq!(CommandType::New.name(), "new");
        assert_eq!(CommandType::Add.name(), "add");
    }

    #[test]
    fn test_argument_creation() {
        let arg = Argument::new("project-name", "Project name");
        assert_eq!(arg.name(), "project-name");
        assert!(!arg.is_required());
    }

    #[test]
    fn test_argument_required() {
        let arg = Argument::new("name", "Name").required();
        assert!(arg.is_required());
    }

    #[test]
    fn test_argument_default() {
        let arg = Argument::new("template", "Template").with_default("basic");
        assert_eq!(arg.default_value(), Some("basic"));
    }

    #[test]
    fn test_flag_creation() {
        let flag = Flag::new("verbose", "Verbose output").with_short('v');
        assert_eq!(flag.long_flag(), "verbose");
        assert_eq!(flag.short_flag(), Some('v'));
        assert!(!flag.has_value());
    }

    #[test]
    fn test_flag_with_value() {
        let flag = Flag::new("output", "Output file")
            .with_short('o')
            .with_value();
        assert!(flag.has_value());
    }

    #[test]
    fn test_command_creation() {
        let cmd = Command::new(CommandType::New)
            .add_argument(Argument::new("name", "Project name").required());
        assert_eq!(cmd.command_type(), CommandType::New);
        assert_eq!(cmd.arguments().len(), 1);
    }

    #[test]
    fn test_command_help() {
        let cmd =
            Command::new(CommandType::New).add_argument(Argument::new("name", "Name").required());
        let help = cmd.help();
        assert!(help.contains("aurora new"));
        assert!(help.contains("name"));
    }

    #[test]
    fn test_cli_creation() {
        let cli = AuroraCli::new();
        assert!(cli.command_count() > 0);
    }

    #[test]
    fn test_cli_default_commands() {
        let cli = AuroraCli::new();
        assert!(cli.get("new").is_some());
        assert!(cli.get("add").is_some());
        assert!(cli.get("generate").is_some());
    }

    #[test]
    fn test_cli_version() {
        let cli = AuroraCli::new();
        assert_eq!(cli.version(), "1.1.0");
    }

    #[test]
    fn test_cli_help() {
        let cli = AuroraCli::new();
        let help = cli.help();
        assert!(help.contains("Aurora v1.1"));
        assert!(help.contains("COMMANDS:"));
    }

    #[test]
    fn test_cli_register_custom() {
        let mut cli = AuroraCli::new();
        let cmd = Command::new(CommandType::New);
        cli.register("custom", cmd);
        assert!(cli.get("custom").is_some());
    }

    #[test]
    fn test_cli_all_commands() {
        let cli = AuroraCli::new();
        let cmds = cli.commands();
        assert!(!cmds.is_empty());
    }

    #[test]
    fn test_default_cli() {
        let cli = AuroraCli::default();
        assert_eq!(cli.command_count(), 6);
    }
}
