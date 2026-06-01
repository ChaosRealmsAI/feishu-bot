use super::*;

#[derive(Subcommand)]
#[command(after_long_help = SETUP_AFTER_HELP)]
pub(in crate::app) enum SetupCommand {
    #[command(about = "Print an AI-readable setup plan without making API writes")]
    Plan(SetupPlanArgs),
    #[command(about = "Build and optionally open a Feishu Open Platform scope grant URL")]
    OpenScopes(SetupOpenScopesArgs),
    #[command(about = "Add the current app bot to the configured Wiki space")]
    WikiBot(SetupWikiBotArgs),
    #[command(about = "Run the recommended first-run setup checklist for common AI office use")]
    Quickstart(SetupQuickstartArgs),
    #[command(about = "Run the setup automation sequence and return next actions")]
    Auto(SetupAutoArgs),
}

#[derive(Args)]
#[command(after_long_help = SETUP_AFTER_HELP)]
pub(in crate::app) struct SetupPlanArgs {
    #[arg(long = "group", help = "Scope group or profile. Can repeat.")]
    pub(in crate::app) groups: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant)]
    pub(in crate::app) token_type: ApiAuthArg,
}

#[derive(Args)]
#[command(after_long_help = SETUP_AFTER_HELP)]
pub(in crate::app) struct SetupOpenScopesArgs {
    #[arg(long = "group", help = "Scope group or profile. Can repeat.")]
    pub(in crate::app) groups: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant)]
    pub(in crate::app) token_type: ApiAuthArg,

    #[arg(
        long,
        help = "Open the grant URL through the Playwright MCP browser bridge"
    )]
    pub(in crate::app) browser: bool,

    #[arg(
        long,
        help = "Open the grant URL with the operating system default browser"
    )]
    pub(in crate::app) system_browser: bool,
}

#[derive(Args)]
#[command(after_long_help = SETUP_AFTER_HELP)]
pub(in crate::app) struct SetupWikiBotArgs {
    #[arg(long, help = "Wiki space ID. Defaults to FEISHU_WIKI_SPACE_ID.")]
    pub(in crate::app) space_id: Option<String>,

    #[arg(long, default_value = "admin", help = "Wiki role: admin or member")]
    pub(in crate::app) member_role: String,

    #[arg(long, help = "Ask Feishu to notify the added member")]
    pub(in crate::app) need_notification: Option<bool>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::User)]
    pub(in crate::app) auth: ApiAuthArg,
}

#[derive(Args)]
#[command(after_long_help = SETUP_AFTER_HELP)]
pub(in crate::app) struct SetupAutoArgs {
    #[arg(long = "group", help = "Scope group or profile. Can repeat.")]
    pub(in crate::app) groups: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant)]
    pub(in crate::app) token_type: ApiAuthArg,

    #[arg(
        long,
        help = "Open the generated scope grant URL through Playwright MCP"
    )]
    pub(in crate::app) open_browser: bool,

    #[arg(
        long,
        help = "Do not try to add the current app bot to FEISHU_WIKI_SPACE_ID"
    )]
    pub(in crate::app) no_wiki_bot: bool,

    #[arg(long, help = "Wiki space ID. Defaults to FEISHU_WIKI_SPACE_ID.")]
    pub(in crate::app) space_id: Option<String>,
}

#[derive(Args)]
#[command(after_long_help = SETUP_AFTER_HELP)]
pub(in crate::app) struct SetupQuickstartArgs {
    #[arg(long = "group", help = "Scope group or profile. Can repeat.")]
    pub(in crate::app) groups: Vec<String>,

    #[arg(long, value_enum, default_value_t = ApiAuthArg::Tenant)]
    pub(in crate::app) token_type: ApiAuthArg,

    #[arg(
        long,
        help = "Open the generated scope grant URL through Playwright MCP"
    )]
    pub(in crate::app) open_browser: bool,

    #[arg(
        long,
        help = "Open the generated scope grant URL with the operating system default browser"
    )]
    pub(in crate::app) system_browser: bool,

    #[arg(
        long,
        help = "Do not try to add the current app bot to FEISHU_WIKI_SPACE_ID"
    )]
    pub(in crate::app) no_wiki_bot: bool,

    #[arg(long, help = "Wiki space ID. Defaults to FEISHU_WIKI_SPACE_ID.")]
    pub(in crate::app) space_id: Option<String>,

    #[arg(
        long,
        default_value = "AI Project",
        help = "Example project name used in returned bootstrap/progress commands"
    )]
    pub(in crate::app) project: String,
}
