use super::*;

#[derive(Subcommand)]
#[command(after_long_help = OKR_AFTER_HELP)]
pub(in crate::app) enum OkrCommand {
    #[command(subcommand, about = "Operate OKR periods")]
    Period(OkrPeriodCommand),
    #[command(subcommand, name = "period-rule", about = "Operate OKR period rules")]
    PeriodRule(OkrPeriodRuleCommand),
    #[command(about = "Get one user's OKR list")]
    UserOkrs(OkrUserOkrsArgs),
    #[command(about = "Batch get OKRs by OKR IDs")]
    BatchGet(OkrBatchGetArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum OkrPeriodCommand {
    #[command(about = "List OKR periods")]
    List(OkrPeriodListArgs),
}

#[derive(Subcommand)]
pub(in crate::app) enum OkrPeriodRuleCommand {
    #[command(about = "List OKR period rules")]
    List,
}

#[derive(Args)]
pub(in crate::app) struct OkrPeriodListArgs {
    #[arg(long, default_value_t = 10, help = "Page size")]
    pub(in crate::app) page_size: u16,

    #[arg(long, help = "Page token")]
    pub(in crate::app) page_token: Option<String>,
}

#[derive(Args)]
pub(in crate::app) struct OkrUserOkrsArgs {
    #[arg(long, help = "User ID")]
    pub(in crate::app) user_id: String,

    #[arg(long, default_value_t = 0, help = "Offset, required by Feishu")]
    pub(in crate::app) offset: u32,

    #[arg(long, default_value_t = 5, help = "Limit, max 10")]
    pub(in crate::app) limit: u16,

    #[arg(
        long,
        default_value = "zh_cn",
        help = "Language, for example zh_cn or en_us"
    )]
    pub(in crate::app) lang: String,

    #[arg(long = "period-id", help = "Restrict to OKR period ID; can repeat")]
    pub(in crate::app) period_ids: Vec<String>,

    #[arg(long, value_enum, default_value_t = OkrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: OkrUserIdTypeArg,
}

#[derive(Args)]
pub(in crate::app) struct OkrBatchGetArgs {
    #[arg(long = "okr-id", help = "OKR ID; can repeat, max 10")]
    pub(in crate::app) okr_ids: Vec<String>,

    #[arg(
        long,
        default_value = "zh_cn",
        help = "Language, for example zh_cn or en_us"
    )]
    pub(in crate::app) lang: String,

    #[arg(long, value_enum, default_value_t = OkrUserIdTypeArg::OpenId)]
    pub(in crate::app) user_id_type: OkrUserIdTypeArg,
}
