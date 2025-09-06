//! RBAC command router for Enterprise

#![allow(dead_code)]

use crate::cli::{
    EnterpriseAclCommands, EnterpriseAuthCommands, EnterpriseLdapCommands, EnterpriseRoleCommands,
    EnterpriseUserCommands, OutputFormat,
};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

use super::rbac_impl;

pub async fn handle_user_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EnterpriseUserCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        EnterpriseUserCommands::List => {
            rbac_impl::list_users(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseUserCommands::Get { id } => {
            rbac_impl::get_user(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseUserCommands::Create { data } => {
            rbac_impl::create_user(conn_mgr, profile_name, data, output_format, query).await
        }
        EnterpriseUserCommands::Update { id, data } => {
            rbac_impl::update_user(conn_mgr, profile_name, *id, data, output_format, query).await
        }
        EnterpriseUserCommands::Delete { id, force } => {
            rbac_impl::delete_user(conn_mgr, profile_name, *id, *force, output_format, query).await
        }
        EnterpriseUserCommands::ResetPassword { id, password } => {
            rbac_impl::reset_user_password(
                conn_mgr,
                profile_name,
                *id,
                password.as_deref(),
                output_format,
                query,
            )
            .await
        }
        EnterpriseUserCommands::GetRoles { user_id } => {
            rbac_impl::get_user_roles(conn_mgr, profile_name, *user_id, output_format, query).await
        }
        EnterpriseUserCommands::AssignRole { user_id, role } => {
            rbac_impl::assign_user_role(
                conn_mgr,
                profile_name,
                *user_id,
                *role,
                output_format,
                query,
            )
            .await
        }
        EnterpriseUserCommands::RemoveRole { user_id, role } => {
            rbac_impl::remove_user_role(
                conn_mgr,
                profile_name,
                *user_id,
                *role,
                output_format,
                query,
            )
            .await
        }
    }
}

pub async fn handle_role_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EnterpriseRoleCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        EnterpriseRoleCommands::List => {
            rbac_impl::list_roles(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseRoleCommands::Get { id } => {
            rbac_impl::get_role(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseRoleCommands::Create { data } => {
            rbac_impl::create_role(conn_mgr, profile_name, data, output_format, query).await
        }
        EnterpriseRoleCommands::Update { id, data } => {
            rbac_impl::update_role(conn_mgr, profile_name, *id, data, output_format, query).await
        }
        EnterpriseRoleCommands::Delete { id, force } => {
            rbac_impl::delete_role(conn_mgr, profile_name, *id, *force, output_format, query).await
        }
        EnterpriseRoleCommands::GetPermissions { id } => {
            rbac_impl::get_role_permissions(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseRoleCommands::GetUsers { role_id } => {
            rbac_impl::get_role_users(conn_mgr, profile_name, *role_id, output_format, query).await
        }
    }
}

pub async fn handle_acl_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EnterpriseAclCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        EnterpriseAclCommands::List => {
            rbac_impl::list_acls(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseAclCommands::Get { id } => {
            rbac_impl::get_acl(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseAclCommands::Create { data } => {
            rbac_impl::create_acl(conn_mgr, profile_name, data, output_format, query).await
        }
        EnterpriseAclCommands::Update { id, data } => {
            rbac_impl::update_acl(conn_mgr, profile_name, *id, data, output_format, query).await
        }
        EnterpriseAclCommands::Delete { id, force } => {
            rbac_impl::delete_acl(conn_mgr, profile_name, *id, *force, output_format, query).await
        }
        EnterpriseAclCommands::Test { user, command } => {
            rbac_impl::test_acl(conn_mgr, profile_name, *user, command, output_format, query).await
        }
    }
}

pub async fn handle_ldap_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EnterpriseLdapCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        EnterpriseLdapCommands::GetConfig => {
            rbac_impl::get_ldap_config(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseLdapCommands::UpdateConfig { data } => {
            rbac_impl::update_ldap_config(conn_mgr, profile_name, data, output_format, query).await
        }
        EnterpriseLdapCommands::TestConnection => {
            rbac_impl::test_ldap_connection(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseLdapCommands::Sync => {
            rbac_impl::sync_ldap(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseLdapCommands::GetMappings => {
            rbac_impl::get_ldap_mappings(conn_mgr, profile_name, output_format, query).await
        }
    }
}

pub async fn handle_auth_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EnterpriseAuthCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        EnterpriseAuthCommands::Test { user } => {
            rbac_impl::test_auth(conn_mgr, profile_name, user, output_format, query).await
        }
        EnterpriseAuthCommands::SessionList => {
            rbac_impl::list_sessions(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseAuthCommands::SessionRevoke { session_id } => {
            rbac_impl::revoke_session(conn_mgr, profile_name, session_id, output_format, query)
                .await
        }
        EnterpriseAuthCommands::SessionRevokeAll { user } => {
            rbac_impl::revoke_all_user_sessions(conn_mgr, profile_name, *user, output_format, query)
                .await
        }
    }
}
