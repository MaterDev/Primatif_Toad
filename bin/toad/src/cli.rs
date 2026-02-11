use clap::{Parser, Subcommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "toad")]
#[command(about = "Primatif_Toad: Toad Control CLI", version = VERSION)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output results as JSON
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project directory
    Create {
        /// Name of the project
        name: String,

        /// Simulate the action without creating files
        #[arg(long, short = 'd')]
        dry_run: bool,

        /// Skip interactive prompts (e.g., editor launch)
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Find projects matching a query
    Reveal {
        /// Case-insensitive search query
        query: String,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
    /// Scan projects and report Git status
    Status {
        /// Optional query to filter projects
        query: Option<String>,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
    /// Ecosystem health and disk usage analytics
    Stats {
        /// Optional query to filter projects
        query: Option<String>,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Show details for all matching projects
        #[arg(long, short = 'a')]
        all: bool,
    },
    /// Manage the global Toad workspace anchor
    Home {
        /// Set a new absolute path for the Toad home
        path: Option<String>,

        /// Skip confirmation prompts
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Execute a shell command across projects matching a query
    Do {
        /// Command to execute
        command: String,

        /// Query to filter projects
        #[arg(long, short = 'q')]
        query: String,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Simulate the action without executing
        #[arg(long, short = 'd')]
        dry_run: bool,

        /// Halt the entire batch if a single project fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Assign a tag to projects
    Tag {
        /// Project name (optional if using filters)
        project: Option<String>,
        /// Tag name
        tag: Option<String>,

        /// Filter by name query
        #[arg(long, short = 'q')]
        query: Option<String>,

        /// Filter by existing tag
        #[arg(long, short = 't')]
        filter_tag: Option<String>,

        /// Automatically assign tags based on detected stacks
        #[arg(long)]
        harvest: bool,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Remove a tag from projects
    Untag {
        /// Project name (optional if using filters)
        project: Option<String>,
        /// Tag name
        tag: Option<String>,

        /// Filter by name query
        #[arg(long, short = 'q')]
        query: Option<String>,

        /// Filter by existing tag
        #[arg(long, short = 't')]
        filter_tag: Option<String>,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Manage and synchronize AI agent skills
    Skill {
        #[command(subcommand)]
        subcommand: SkillCommand,
    },
    /// Synchronize the project registry cache
    Sync,
    /// Manage language/stack strategies
    Strategy {
        #[command(subcommand)]
        subcommand: StrategyCommands,
    },
    /// Reclaim disk space by removing build artifacts
    Clean {
        /// Optional query to filter projects
        query: Option<String>,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Filter by activity tier (active, cold, archive)
        #[arg(long)]
        tier: Option<String>,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Simulate the action without deleting
        #[arg(long, short = 'd')]
        dry_run: bool,
    },
    /// Generate programmatic CLI documentation (Markdown)
    Docs,
    /// Manage project contexts (register, switch, list)
    Project {
        #[command(subcommand)]
        subcommand: ProjectCommand,
    },
    /// Multi-repo Git orchestration
    Ggit {
        #[command(subcommand)]
        subcommand: GgitCommand,
    },
    /// Custom workflows and script orchestration
    Cw {
        #[command(subcommand)]
        subcommand: CwCommand,
    },
    /// List all available commands
    List,
    /// Display version information and the Toad banner
    Version,
}

#[derive(Subcommand)]
pub enum SkillCommand {
    /// Synchronize all skills (Blueprint, CLI, Manifest) to AI vendors
    Sync,
    /// List distributed skills and registered vendors
    List,
}

#[derive(Subcommand)]
pub enum CwCommand {
    /// Execute a custom workflow script
    Run {
        /// Name of the workflow
        name: String,
        /// Arguments to pass to the script
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Register a new custom workflow
    Register {
        /// Name of the workflow
        name: String,
        /// Path to the script
        script: String,
        /// Optional description
        #[arg(long, short = 'd')]
        description: Option<String>,
    },
    /// List all registered custom workflows
    List,
    /// Show detailed info for a custom workflow
    Info {
        /// Name of the workflow
        name: String,
    },
    /// Remove a registered custom workflow
    Delete {
        /// Name of the workflow
        name: String,
    },
}

#[derive(Subcommand)]
pub enum GgitCommand {
    /// Show consolidated Git status across repositories
    Status {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
    /// Commit changes across repositories
    Commit {
        /// Commit message
        #[arg(long, short = 'm')]
        message: String,
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Automatically commit the Hub root if submodules are changed (Cascade)
        #[arg(long, short = 'c')]
        cascade: bool,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Push changes across repositories
    Push {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Pull changes across repositories
    Pull {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Switch branches across repositories
    Checkout {
        /// Branch name
        branch: String,
        /// Create the branch if it doesn't exist
        #[arg(long, short = 'b')]
        create: bool,
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Synchronize and align repositories (safe multi-repo update)
    Sync {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Skip pre-flight safety checks
        #[arg(long, short = 'f')]
        force: bool,
    },
    /// List all branches across repositories
    Branches {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Show remote branches
        #[arg(long, short = 'r')]
        all: bool,
    },
    /// Force-align submodules to Hub root expectations
    Align {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Register a new project context
    Register {
        /// Name of the context
        name: String,
        /// Absolute path to the workspace root
        path: String,
        /// Optional description
        #[arg(long, short = 'd')]
        description: Option<String>,
        /// Explicitly set the context type (hub, pond, generic)
        #[arg(long, short = 't', value_enum)]
        context_type: Option<ContextTypeChoice>,
        /// AI Vendors to sync memory to (comma-separated: windsurf,cursor,gemini)
        #[arg(long, short = 'a')]
        ai: Option<String>,
    },
    /// Switch the active project context
    Switch {
        /// Name of the context
        name: String,
    },
    /// Show the currently active context
    Current,
    /// List all registered contexts
    List,
    /// Update an existing context
    Update {
        /// Name of the context
        name: String,
        /// New path for the context
        #[arg(long, short = 'p')]
        path: Option<String>,
        /// New description
        #[arg(long, short = 'd')]
        description: Option<String>,
        /// New context type
        #[arg(long, short = 't', value_enum)]
        context_type: Option<ContextTypeChoice>,
        /// Update AI Vendors
        #[arg(long, short = 'a')]
        ai: Option<String>,
    },
    /// Remove a registered context
    Delete {
        /// Name of the context
        name: String,
        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Show detailed info for a context
    Info {
        /// Name of the context
        name: String,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum ContextTypeChoice {
    Hub,
    Pond,
    Generic,
}

impl From<ContextTypeChoice> for toad_core::ContextType {
    fn from(choice: ContextTypeChoice) -> Self {
        match choice {
            ContextTypeChoice::Hub => toad_core::ContextType::Hub,
            ContextTypeChoice::Pond => toad_core::ContextType::Pond,
            ContextTypeChoice::Generic => toad_core::ContextType::Generic,
        }
    }
}

impl From<toad_core::ContextType> for ContextTypeChoice {
    fn from(t: toad_core::ContextType) -> Self {
        match t {
            toad_core::ContextType::Hub => ContextTypeChoice::Hub,
            toad_core::ContextType::Pond => ContextTypeChoice::Pond,
            toad_core::ContextType::Generic => ContextTypeChoice::Generic,
        }
    }
}

#[derive(Subcommand)]
pub enum StrategyCommands {
    /// List all active strategies
    List,
    /// Add a new custom strategy
    Add {
        /// Name of the strategy (e.g., Elixir)
        name: String,
        /// Files that identify this stack (comma-separated, e.g., mix.exs)
        #[arg(long, short = 'm')]
        match_files: String,
        /// Build artifacts to clean (comma-separated, e.g., deps,_build)
        #[arg(long, short = 'c')]
        artifacts: Option<String>,
        /// Tags to auto-assign (comma-separated, e.g., #elixir)
        #[arg(long, short = 't')]
        tags: Option<String>,
        /// Priority for matching (higher = earlier check)
        #[arg(long, default_value = "10")]
        priority: i32,
    },
    /// Show details of a specific strategy
    Info {
        /// Name of the strategy
        name: String,
    },
    /// Remove a custom strategy
    Remove {
        /// Name of the strategy
        name: String,
    },
}
