use crate::embedded::{err, messaging::Request, Result};

/// Permissions for server.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Permission {
    /// They can't modify contents in database (read-only).
    Guest,
    /// They can modify contents in database but they cannot use some sensitive commands (like flush).
    Admin,
    /// Can run any command. and also this is the default permission for servers.
    #[default]
    Owner,
}

impl Permission {
    /// Check if request is allowed for permission.
    pub fn is_allowed(&self, request: &Request) -> bool {
        match request {
            // read-only
            Request::Get(_) | Request::Exists(_) | Request::DowngradePermission => true,
            // requires admin or higher
            Request::Set(_, _)
            | Request::Delete(_)
            | Request::Increment(_, _)
            | Request::Decrement(_, _)
            | Request::Search(_) => self == &Permission::Admin || self == &Permission::Owner,
            // owner only
            Request::Flush => self == &Permission::Owner,
        }
    }

    /// Check if request is allowed for permission and return a result.
    pub fn allowed(&self, request: &Request) -> Result<()> {
        if self.is_allowed(request) {
            Ok(())
        } else {
            Err(err!(PermissionFailure))
        }
    }

    /// Downgrade permission.
    pub fn lower(&self) -> Self {
        match self {
            Permission::Guest => Permission::Guest,
            Permission::Admin => Permission::Guest,
            Permission::Owner => Permission::Admin,
        }
    }
}
