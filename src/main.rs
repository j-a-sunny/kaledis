mod commands;
mod dalbit;
mod dev_server;
mod editpe;
mod home_manager;
mod toml_conf;
mod utils;
mod zip_utils;

use std::{process::ExitCode, thread};

use colored::Colorize;
use commands::{CLI, handle_commands};

use clap::Parser;
use tokio::runtime;
use tracing_error::ErrorLayer;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::utils::CliFormatter;

const STACK_SIZE: usize = 4 * 1024 * 1024 * 1024;

fn print_banner() {
    println!(
        "{}",
        "░  ░░░░  ░░░      ░░░  ░░░░░░░░        ░░       ░░░        ░░░      ░░".purple()
    );
    println!(
        "{}",
        "▒  ▒▒▒  ▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒  ▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒▒▒▒".purple()
    );
    println!(
        "{}",
        "▓     ▓▓▓▓▓  ▓▓▓▓  ▓▓  ▓▓▓▓▓▓▓▓      ▓▓▓▓  ▓▓▓▓  ▓▓▓▓▓  ▓▓▓▓▓▓      ▓▓".cyan()
    );
    println!(
        "{}",
        "█  ███  ███        ██  ████████  ████████  ████  █████  ███████████  █".cyan()
    );
    println!(
        "{}",
        "█  ████  ██  ████  ██        ██        ██       ███        ███      ██".cyan()
    );
    println!();
}

fn run() -> ExitCode {
    print_banner();
    let args = CLI::parse();
    let rt = runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(handle_commands(args.cli));
    ExitCode::SUCCESS
}

fn main() -> color_eyre::Result<ExitCode> {
    color_eyre::install()?;

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,darklua=warn,backhand=warn"));
    let indicatif_layer = IndicatifLayer::new();
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_writer(std::io::stderr)
        .event_format(CliFormatter);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .with(indicatif_layer)
        .init();

    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();
    Ok(child.join().unwrap_or(ExitCode::FAILURE))
}
