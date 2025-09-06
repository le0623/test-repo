//! Utility functions for Enterprise commands

use crate::cli::OutputFormat;
use crate::error::{RedisCtlError, Result as CliResult};
use crate::output::print_output;
use anyhow::Context;
use dialoguer::Confirm;
use serde_json::Value;

/// Apply JMESPath query to JSON data
pub fn apply_jmespath(data: &Value, query: &str) -> CliResult<Value> {
    let expr = jmespath::compile(query)
        .with_context(|| format!("Invalid JMESPath expression: {}", query))?;
    let result = expr
        .search(data)
        .with_context(|| format!("Failed to apply JMESPath query: {}", query))?;
    // Convert jmespath Variable to serde_json Value
    let json_str = serde_json::to_string(&result).context("Failed to serialize JMESPath result")?;
    let value =
        serde_json::from_str(&json_str).context("Failed to parse JMESPath result as JSON")?;
    Ok(value)
}

/// Handle output with optional JMESPath query
pub fn handle_output(
    data: Value,
    _output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<Value> {
    if let Some(q) = query {
        apply_jmespath(&data, q)
    } else {
        Ok(data)
    }
}

/// Print formatted output based on format type
pub fn print_formatted_output(data: Value, output_format: OutputFormat) -> CliResult<()> {
    match output_format {
        OutputFormat::Json => {
            print_output(data, crate::output::OutputFormat::Json, None).map_err(|e| {
                RedisCtlError::OutputError {
                    message: e.to_string(),
                }
            })?;
        }
        OutputFormat::Yaml => {
            print_output(data, crate::output::OutputFormat::Yaml, None).map_err(|e| {
                RedisCtlError::OutputError {
                    message: e.to_string(),
                }
            })?;
        }
        OutputFormat::Table | OutputFormat::Auto => {
            // For now, output as JSON for table format
            // TODO: Implement proper table formatting for enterprise commands
            print_output(data, crate::output::OutputFormat::Json, None).map_err(|e| {
                RedisCtlError::OutputError {
                    message: e.to_string(),
                }
            })?;
        }
    }
    Ok(())
}

/// Confirm an action with the user
pub fn confirm_action(message: &str) -> CliResult<bool> {
    #[cfg(unix)]
    {
        use std::io::IsTerminal;
        if std::io::stdin().is_terminal() {
            Ok(Confirm::new()
                .with_prompt(message)
                .default(false)
                .interact()
                .context("Failed to get user confirmation")?)
        } else {
            // In non-interactive mode, print warning and return false
            eprintln!("Warning: {} Use --force to skip confirmation.", message);
            Ok(false)
        }
    }

    #[cfg(not(unix))]
    {
        Ok(Confirm::new()
            .with_prompt(message)
            .default(false)
            .interact()
            .context("Failed to get user confirmation")?)
    }
}

/// Read JSON data from string or file
pub fn read_json_data(data: &str) -> CliResult<Value> {
    let json_str = if let Some(file_path) = data.strip_prefix('@') {
        std::fs::read_to_string(file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path, e))?
    } else {
        data.to_string()
    };

    serde_json::from_str(&json_str).map_err(|e| anyhow::anyhow!("Invalid JSON: {}", e).into())
}
