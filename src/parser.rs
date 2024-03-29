use crate::{
    api::list_ceos_firmware,
    structs::{args::Args, auth::AuthResponse},
};

pub async fn pars_args(args: Args, auth_data: AuthResponse) {
    if args.list {
        list_ceos_firmware(&auth_data).await
    }
}
