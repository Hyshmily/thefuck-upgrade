pub mod alias;
pub mod delete;
pub mod firstuse;
pub mod fix_command;
pub mod update;

pub fn print_alias() {
    alias::print_alias_command();
}
