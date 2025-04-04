use clap::{
    Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};
use clap_complete::Shell;
use std::path::PathBuf;

fn make_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Blue.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(Parser)]
#[command(version, about, styles=make_style())]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Action,

    /// Verbose output logging
    #[clap(long, global = true)]
    pub(crate) verbose: bool,
}

#[derive(Subcommand)]
pub(crate) enum Action {
    Completions(CompletionSubcommand),
    Snapshot(SnapshotSubcommand),
    Clean(CleanSubcommand),
}

#[derive(Parser)]
pub(crate) struct SubvolumeArgs {
    /// Mount point of btrfs filesystem
    pub(crate) mount_point: PathBuf,

    /// Path to subvolume to snapshot (relative to mount point)
    pub(crate) subvol_path: PathBuf,
}

#[derive(Parser)]
pub(crate) struct CleanSubcommand {
    #[clap(flatten)]
    pub(crate) subvol_args: SubvolumeArgs,
    #[clap(flatten)]
    pub(crate) cleaning_args: CleaningArgs,
    /// Limit clean task only to mentioned path (relative to mount point)
    #[clap(long, short = 'p', default_value = ".snapshots")]
    pub(crate) snapshot_path: PathBuf,
}

#[derive(Parser)]
pub(crate) struct SnapshotSubcommand {
    #[clap(flatten)]
    pub(crate) subvol_args: SubvolumeArgs,

    #[clap(flatten)]
    pub(crate) snapshot_args: SnapshotArgs,

    #[clap(flatten)]
    pub(crate) cleaning_args: Option<CleaningArgs>,
}

#[derive(Parser)]
/// Generate shell completions file
pub(crate) struct CompletionSubcommand {
    /// Compatible shell for completions file
    pub(crate) shell_completion: Shell,
}

#[derive(Parser)]
/// Create snapshots of btrfs subvolumes and optionally invoke cleaning
pub(crate) struct SnapshotArgs {
    /// Path in which snapshots are stored (relative to mount point)
    #[clap(long, short = 'p', default_value = ".snapshots")]
    pub(crate) snapshot_path: PathBuf,

    /// Make snapshot readonly
    #[clap(long, short = 'r')]
    pub(crate) readonly: bool,

    /// Prefix for snapshot name (defaults to subvolume name)
    #[clap(long)]
    pub(crate) prefix: Option<PathBuf>,

    /// Datetime suffix format for snapshot name
    #[clap(long, short = 'f', default_value = "%Y-%m-%d-%H%M%S")]
    pub(crate) suffix_format: String,
}

#[derive(Parser)]
#[clap(after_help = "You must specify at least one of --keep-count or --keep-since.")]
pub(crate) struct CleaningArgs {
    /// Minimum number of snapshots to preserve
    #[clap(long, short = 'c')]
    pub(crate) keep_count: Option<usize>,

    /// Minimum age of snapshots to preserve
    /// (does not clean snapshots younger than given duration)
    ///
    /// Example: 5d (5 days), 6h 30m (6 hours 30 minutes), 1y (1 year), 5M 1w (5 months 1 week)
    ///
    /// This takes precedence over "--keep-count", i.e. - Even if number of younger
    /// snapshots is greater than keep_count, they are not removed.
    ///
    /// Only the older snapshots are considered for removal.
    #[clap(long, short = 's')]
    pub(crate) keep_since: Option<humantime::Duration>,
}
