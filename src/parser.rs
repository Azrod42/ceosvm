use crate::{
    api::{
        ceos_firmware_create::create_ceos_firmware, ceos_firmware_list::list_ceos_firmware,
        ceos_firmware_set_stable::update_ceos_firmware,
    },
    structs::{
        args::{CeosFirmware, CliArgs, Command},
        auth::AuthResponse,
    },
};

pub async fn pars_args(args: CliArgs, auth_data: AuthResponse) {
    let client: reqwest::Client = reqwest::Client::new();

    match args.cmd {
        Command::CeosFirmware(ceos_firmware) => match ceos_firmware {
            CeosFirmware::List(flags) => {
                list_ceos_firmware(&client, &auth_data, &flags.tenant).await
            }
            CeosFirmware::CreateVersion(flags) => {
                create_ceos_firmware(&client, &auth_data, flags).await
            }
            CeosFirmware::SetStable(flags) => {
                update_ceos_firmware(&client, &auth_data, flags).await
            }
        },
    }
}
