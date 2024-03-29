use colored::Colorize;

use crate::structs::auth::{AuthResponse, UserRole};

pub fn custom_exit(reason: Option<String>) {
    match reason {
        Some(value) => {
            println!("{}: {}", "Error".red(), value);
            std::process::exit(1);
        }
        None => {
            println!("{}: unexpected error happend", "Error".red());
            std::process::exit(1);
        }
    }
}

pub fn check_user_role(auth_response: &AuthResponse) {
    if !auth_response
        .roles
        .iter()
        .any(|item| UserRole::ADMIN.check_role(&item.role_name))
    {
        custom_exit(Some("Only megahub admin can use this tool".to_string()));
    }
}
