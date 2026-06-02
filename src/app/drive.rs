#![allow(clippy::too_many_arguments)]

use super::*;

mod comment;
mod comment_exec;
mod file_ops;
mod helpers;
mod media_exec;
mod permission_exec;
mod permissions;
mod transfer;
mod version_exec;

pub(super) use comment::*;
use comment_exec::run_drive_comment_command;
use file_ops::{
    run_drive_copy_command, run_drive_delete_command, run_drive_download_command,
    run_drive_folder_command, run_drive_list_command, run_drive_move_command,
    run_drive_stats_command, run_drive_upload_command, run_drive_view_record_command,
};
pub(super) use helpers::*;
use media_exec::run_drive_media_command;
use permission_exec::run_drive_permission_command;
pub(super) use permissions::*;
use transfer::{run_drive_export_command, run_drive_import_command};
use version_exec::{run_drive_subscription_command, run_drive_version_command};
pub(super) async fn run_drive_command(
    api: &mut FeishuClient,
    command: DriveCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        DriveCommand::List(args) => run_drive_list_command(api, args).await?,
        DriveCommand::Folder(command) => run_drive_folder_command(api, command).await?,
        DriveCommand::Upload(args) => run_drive_upload_command(api, args).await?,
        DriveCommand::UploadLarge(args) => upload_large_drive_file(api, args).await?,
        DriveCommand::Media(command) => run_drive_media_command(api, command).await?,
        DriveCommand::Import(command) => run_drive_import_command(api, command).await?,
        DriveCommand::Export(command) => run_drive_export_command(api, command).await?,
        DriveCommand::Comment(command) => run_drive_comment_command(api, command).await?,
        DriveCommand::Version(command) => run_drive_version_command(api, command).await?,
        DriveCommand::Subscription(command) => run_drive_subscription_command(api, command).await?,
        DriveCommand::ViewRecord(args) => run_drive_view_record_command(api, args).await?,
        DriveCommand::Download(args) => run_drive_download_command(api, args).await?,
        DriveCommand::Permission(command) => run_drive_permission_command(api, command).await?,
        DriveCommand::Stats(args) => run_drive_stats_command(api, args).await?,
        DriveCommand::Copy(args) => run_drive_copy_command(api, args).await?,
        DriveCommand::Move(args) => run_drive_move_command(api, args).await?,
        DriveCommand::Delete(args) => run_drive_delete_command(api, args).await?,
    };
    print_response(raw_json, "drive operation completed", data)
}
