pub mod alias;
pub mod firstuse;
pub mod fix_command;

pub async fn print_alias() {
    alias::print_alias_command().await;
}
