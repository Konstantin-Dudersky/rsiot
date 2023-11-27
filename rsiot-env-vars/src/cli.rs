//! CLI интерфейс

use clap::{Parser, Subcommand};
use tracing::info;

use crate::{create_env_file::create_env_file, load_config, Errors, IConfig};

const ENV_EXAMPLE_FILE: &str = ".env.example";

/// Структура входа CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Перечент команд CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Создать файл .env.example с настройками по-умолчанию
    Create,
    /// Проверка файла .env на наличие и правильность конфигурации
    Check,
}

/// Запускаем CLI
pub fn env_vars_cli<TConfig>()
where
    TConfig: IConfig,
{
    let cli = Cli::parse();

    let value = cli.command;
    let command = match value {
        Commands::Create => command_create::<TConfig>(),
        Commands::Check => command_check::<TConfig>(),
    };
    command.ok();
}

fn command_create<TConfig>() -> Result<(), Errors>
where
    TConfig: IConfig,
{
    info!("Создаем файл {}", ENV_EXAMPLE_FILE);
    create_env_file::<TConfig>(ENV_EXAMPLE_FILE)?;
    info!("Файл {} создан", ENV_EXAMPLE_FILE);
    Ok(())
}

fn command_check<TConfig>() -> Result<(), Errors>
where
    TConfig: IConfig,
{
    info!("Пробуем загрузить файл .env");
    let config = load_config::<TConfig>()?;
    info!("Загружены настройки: {:#?}", config);
    Ok(())
}
