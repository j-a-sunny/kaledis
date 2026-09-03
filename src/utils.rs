use std::{env, path::PathBuf};

use colored::Colorize;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

pub fn relative(path: Option<PathBuf>) -> PathBuf {
    let cur = env::current_dir().unwrap();
    path.map(|x| cur.join(x)).unwrap_or(cur)
}

#[macro_export]
macro_rules! allow {
    ($target:expr, $equal:expr) => {
        {
            $target == $equal
        }
    };
    ($target:expr, $equal:expr, $($eq:expr),+) => {
        {
            allow!($target, $equal) ||
            allow!($target, $($eq),+)
        }
    };
}

pub struct CliFormatter;

impl<S, N> FormatEvent<S, N> for CliFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let prefix = match *event.metadata().level() {
            Level::ERROR => "  [X]".red(),
            Level::WARN => "  [!]".yellow(),
            Level::INFO => "  [+]".green(),
            Level::DEBUG => "  [#]".blue(),
            Level::TRACE => "  [*]".bright_black(),
        };
        write!(writer, "{} ", prefix)?;

        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
