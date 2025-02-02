use std::collections::HashMap;

use crate::json;
use crate::json::prelude::*;
use crate::model::application::command::CommandPermissionType;

/// A builder for creating several [`CommandPermission`].
///
/// [`CommandPermission`]: crate::model::application::command::CommandPermission
#[derive(Clone, Debug, Default)]
#[deprecated(note = "use `CreateApplicationCommandPermissionsData`")]
pub struct CreateApplicationCommandsPermissions(pub Vec<Value>);

impl CreateApplicationCommandsPermissions {
    /// Creates a new application command.
    pub fn create_application_command<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(
            &mut CreateApplicationCommandPermissions,
        ) -> &mut CreateApplicationCommandPermissions,
    {
        let mut data = CreateApplicationCommandPermissions::default();
        f(&mut data);

        self.add_application_command(data);

        self
    }

    /// Adds a new application command.
    pub fn add_application_command(
        &mut self,
        application_command: CreateApplicationCommandPermissions,
    ) -> &mut Self {
        let new_data = Value::from(json::hashmap_to_json_map(application_command.0));

        self.0.push(new_data);

        self
    }

    /// Sets all the application commands.
    pub fn set_application_commands(
        &mut self,
        application_commands: Vec<CreateApplicationCommandPermissions>,
    ) -> &mut Self {
        let new_application_commands =
            application_commands.into_iter().map(|f| Value::from(json::hashmap_to_json_map(f.0)));

        for application_command in new_application_commands {
            self.0.push(application_command);
        }

        self
    }
}
/// A builder for creating an [`CommandPermission`].
///
/// [`CommandPermission`]: crate::model::application::command::CommandPermission
#[derive(Clone, Debug, Default)]
#[deprecated(note = "use `CreateApplicationCommandPermissionsData`")]
pub struct CreateApplicationCommandPermissions(pub HashMap<&'static str, Value>);

impl CreateApplicationCommandPermissions {
    /// The [`CommandId`] these permissions belong to.
    ///
    /// [`CommandId`]: crate::model::id::CommandId
    pub fn id(&mut self, application_command_id: u64) -> &mut Self {
        self.0.insert("id", Value::from(application_command_id.to_string()));
        self
    }

    /// Creates permissions for the application command.
    pub fn create_permissions<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(
            &mut CreateApplicationCommandPermissionData,
        ) -> &mut CreateApplicationCommandPermissionData,
    {
        let mut data = CreateApplicationCommandPermissionData::default();
        f(&mut data);

        self.add_permissions(data);

        self
    }

    /// Adds permission for the application command.
    pub fn add_permissions(
        &mut self,
        permission: CreateApplicationCommandPermissionData,
    ) -> &mut Self {
        let new_data = json::hashmap_to_json_map(permission.0);
        let permissions =
            self.0.entry("permissions").or_insert_with(|| Value::from(Vec::<Value>::new()));

        let permissions_array = permissions.as_array_mut().expect("Must be an array");

        permissions_array.push(Value::from(new_data));

        self
    }

    /// Sets permissions for the application command.
    pub fn set_permissions(
        &mut self,
        permissions: Vec<CreateApplicationCommandPermissionData>,
    ) -> &mut Self {
        let new_permissions = permissions
            .into_iter()
            .map(|f| Value::from(json::hashmap_to_json_map(f.0)))
            .collect::<Vec<Value>>();

        self.0.insert("permissions", Value::from(new_permissions));

        self
    }
}

/// A builder for creating several [`CommandPermissionData`].
///
/// [`CommandPermissionData`]: crate::model::application::command::CommandPermissionData
#[derive(Clone, Debug, Default)]
pub struct CreateApplicationCommandPermissionsData(pub HashMap<&'static str, Value>);

impl CreateApplicationCommandPermissionsData {
    /// Creates a permission for the application command.
    pub fn create_permission<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(
            &mut CreateApplicationCommandPermissionData,
        ) -> &mut CreateApplicationCommandPermissionData,
    {
        let mut data = CreateApplicationCommandPermissionData::default();
        f(&mut data);

        self.add_permission(data);

        self
    }

    /// Adds a permission for the application command.
    pub fn add_permission(
        &mut self,
        permission: CreateApplicationCommandPermissionData,
    ) -> &mut Self {
        let new_data = json::hashmap_to_json_map(permission.0);
        let permissions =
            self.0.entry("permissions").or_insert_with(|| Value::from(Vec::<Value>::new()));

        let permissions_array = permissions.as_array_mut().expect("Must be an array");

        permissions_array.push(Value::from(new_data));

        self
    }

    /// Sets permissions for the application command.
    pub fn set_permissions(
        &mut self,
        permissions: Vec<CreateApplicationCommandPermissionData>,
    ) -> &mut Self {
        let new_permissions = permissions
            .into_iter()
            .map(|f| Value::from(json::hashmap_to_json_map(f.0)))
            .collect::<Vec<Value>>();

        self.0.insert("permissions", Value::from(new_permissions));

        self
    }
}

/// A builder for creating an [`CommandPermissionData`].
///
/// All fields are required.
///
/// [`CommandPermissionData`]: crate::model::application::command::CommandPermissionData
#[derive(Clone, Debug, Default)]
pub struct CreateApplicationCommandPermissionData(pub HashMap<&'static str, Value>);

impl CreateApplicationCommandPermissionData {
    /// Sets the `CommandPermissionType` for the [`CommandPermissionData`].
    ///
    /// [`CommandPermissionData`]: crate::model::application::command::CommandPermissionData
    pub fn kind(&mut self, kind: CommandPermissionType) -> &mut Self {
        self.0.insert("type", from_number(kind as u8));
        self
    }

    /// Sets the CommandPermissionId for the [`CommandPermissionData`].
    ///
    /// [`CommandPermissionData`]: crate::model::application::command::CommandPermissionData
    pub fn id(&mut self, id: u64) -> &mut Self {
        self.0.insert("id", Value::from(id.to_string()));
        self
    }

    /// Sets the permission for the [`CommandPermissionData`].
    ///
    /// **Note**: Setting it to `false` will only grey the application command in the
    /// list, it will not fully hide it to the user.
    ///
    /// [`CommandPermissionData`]: crate::model::application::command::CommandPermissionData
    pub fn permission(&mut self, permission: bool) -> &mut Self {
        self.0.insert("permission", Value::from(permission));
        self
    }
}
