//! Node command implementations for Redis Enterprise

#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::nodes::NodeHandler;

use super::utils::*;

// Node Operations

pub async fn list_nodes(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let nodes = handler.list().await?;
    let nodes_json = serde_json::to_value(nodes).context("Failed to serialize nodes")?;
    let data = handle_output(nodes_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let node = handler.get(id).await?;
    let node_json = serde_json::to_value(node).context("Failed to serialize node")?;
    let data = handle_output(node_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn add_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let node_data = read_json_data(data).context("Failed to parse node data")?;

    // Note: The actual add node operation typically requires cluster join operations
    // This is a placeholder for the actual implementation which would use cluster join
    let result = client.post_raw("/v1/nodes", node_data).await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn remove_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Remove node {} from cluster?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    handler.remove(id).await?;
    println!("Node {} removed successfully", id);
    Ok(())
}

pub async fn update_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    let update_data = read_json_data(data).context("Failed to parse update data")?;
    let updated = handler.update(id, update_data).await?;
    let updated_json = serde_json::to_value(updated).context("Failed to serialize updated node")?;
    let data = handle_output(updated_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// Node Status & Health

pub async fn get_node_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let status = handler.status(id).await?;
    let data = handle_output(status, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let stats = handler.stats(id).await?;
    let stats_json = serde_json::to_value(stats).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    interval: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Metrics endpoint typically requires interval parameter
    let endpoint = if let Some(interval) = interval {
        format!("/v1/nodes/{}/metrics?interval={}", id, interval)
    } else {
        format!("/v1/nodes/{}/metrics", id)
    };

    let metrics = client.get_raw(&endpoint).await?;
    let data = handle_output(metrics, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn check_node_health(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Health check typically combines multiple status endpoints
    let handler = NodeHandler::new(client);
    let status = handler.status(id).await?;
    let data = handle_output(status, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_alerts(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let alerts = handler.alerts_for(id).await?;
    let data = handle_output(alerts, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// Node Maintenance

pub async fn enable_maintenance(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let result = handler.execute_action(id, "maintenance_on").await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn disable_maintenance(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);
    let result = handler.execute_action(id, "maintenance_off").await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn rebalance_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Rebalance typically uses the rebalance action
    let result = handler.execute_action(id, "rebalance").await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn drain_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Drain is typically done via the drain action
    let result = handler.execute_action(id, "drain").await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn restart_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Restart node {} services?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Restart typically uses the restart action
    let result = handler.execute_action(id, "restart").await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// Node Configuration

pub async fn get_node_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Configuration is typically part of the node details
    let handler = NodeHandler::new(client);
    let node = handler.get(id).await?;
    let node_json = serde_json::to_value(node).context("Failed to serialize node")?;
    let data = handle_output(node_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_node_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    let config_data = read_json_data(data).context("Failed to parse config data")?;
    let updated = handler.update(id, config_data).await?;
    let updated_json = serde_json::to_value(updated).context("Failed to serialize updated node")?;
    let data = handle_output(updated_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_rack(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Rack info is part of node details
    let node = handler.get(id).await?;
    let node_json = serde_json::to_value(node).context("Failed to serialize node")?;
    let data = handle_output(node_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn set_node_rack(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    rack: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    let update_data = serde_json::json!({
        "rack_id": rack
    });

    let updated = handler.update(id, update_data).await?;
    let updated_json = serde_json::to_value(updated).context("Failed to serialize updated node")?;
    let data = handle_output(updated_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Role info is part of node details
    let node = handler.get(id).await?;
    let node_json = serde_json::to_value(node).context("Failed to serialize node")?;
    let data = handle_output(node_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// Node Resources

pub async fn get_node_resources(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Resources are typically in stats
    let stats = handler.stats(id).await?;
    let stats_json = serde_json::to_value(stats).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_memory(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Memory details are in stats
    let stats = handler.stats(id).await?;
    let stats_json = serde_json::to_value(stats).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_cpu(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // CPU details are in stats
    let stats = handler.stats(id).await?;
    let stats_json = serde_json::to_value(stats).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_storage(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Storage details are in stats
    let stats = handler.stats(id).await?;
    let stats_json = serde_json::to_value(stats).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_node_network(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = NodeHandler::new(client);

    // Network stats are typically in stats
    let stats = handler.stats(id).await?;
    let stats_json = serde_json::to_value(stats).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
