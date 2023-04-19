use crate::Request;

/// Permissions for server.
#[derive(Debug, Clone, Default, PartialEq)]
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
            Request::Get(_) | Request::Exists(_) => true,
            // requires admin or higher
            Request::Set(_, _)
            | Request::Delete(_)
            | Request::Increment(_, _)
            | Request::Decrement(_, _) => self == &Permission::Admin || self == &Permission::Owner,
            // owner only
            Request::Flush => self == &Permission::Owner,
        }
    }

    /// Check if request is allowed for permission and return a result.
    pub fn allowed(&self, request: &Request) -> crate::Result<()> {
        if self.is_allowed(request) {
            Ok(())
        } else {
            Err(crate::Error::PermissionFailure)
        }
    }
}
