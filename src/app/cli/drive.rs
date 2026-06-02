use super::*;

#[derive(Args)]
pub(in crate::app) struct DrivePermissionMemberUpdateArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long = "file-type", help = "doc, docx, sheet, bitable, file, wiki")]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Collaborator ID")]
    pub(in crate::app) member_id: String,

    #[arg(
        long,
        default_value = "openid",
        help = "email, openid, userid, openchat"
    )]
    pub(in crate::app) member_type: String,

    #[arg(long, default_value = "view", help = "view, edit, full_access")]
    pub(in crate::app) perm: String,

    #[arg(long, default_value = "container", help = "container or single_page")]
    pub(in crate::app) perm_type: String,

    #[arg(
        long = "collaborator-type",
        default_value = "user",
        help = "user, chat, department, group"
    )]
    pub(in crate::app) collaborator_type: String,

    #[arg(long, help = "Set need_notification=true")]
    pub(in crate::app) need_notification: bool,

    #[arg(long, help = "Raw Feishu member update body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw member update body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw member update body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DrivePermissionMemberDeleteArgs {
    #[arg(long, help = "Cloud document token")]
    pub(in crate::app) token: String,

    #[arg(long = "file-type", help = "doc, docx, sheet, bitable, file, wiki")]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Collaborator ID")]
    pub(in crate::app) member_id: String,

    #[arg(
        long,
        default_value = "openid",
        help = "email, openid, userid, openchat"
    )]
    pub(in crate::app) member_type: String,

    #[arg(long, default_value = "container", help = "container or single_page")]
    pub(in crate::app) perm_type: String,

    #[arg(
        long = "collaborator-type",
        default_value = "user",
        help = "user, chat, department, group"
    )]
    pub(in crate::app) collaborator_type: String,

    #[arg(long, help = "Raw Feishu member delete body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw member delete body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw member delete body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DriveFileRefArgs {
    #[arg(long, help = "File token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        help = "doc, docx, sheet, bitable, file, folder, etc."
    )]
    pub(in crate::app) file_type: String,
}

#[derive(Args)]
pub(in crate::app) struct DriveCopyArgs {
    #[arg(long, help = "File token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        help = "doc, docx, sheet, bitable, file, folder, etc."
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Target folder token")]
    pub(in crate::app) folder_token: String,

    #[arg(long, help = "Optional new file name")]
    pub(in crate::app) name: Option<String>,

    #[arg(long, help = "Raw Feishu copy body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw copy body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw copy body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}

#[derive(Args)]
pub(in crate::app) struct DriveMoveArgs {
    #[arg(long, help = "File token")]
    pub(in crate::app) file_token: String,

    #[arg(
        long = "file-type",
        help = "doc, docx, sheet, bitable, file, folder, etc."
    )]
    pub(in crate::app) file_type: String,

    #[arg(long, help = "Target folder token")]
    pub(in crate::app) folder_token: String,

    #[arg(long, help = "Raw Feishu move body JSON")]
    pub(in crate::app) body_json: Option<String>,

    #[arg(long, help = "Read raw move body JSON from file")]
    pub(in crate::app) file: Option<PathBuf>,

    #[arg(long, help = "Read raw move body JSON from stdin")]
    pub(in crate::app) stdin: bool,
}
