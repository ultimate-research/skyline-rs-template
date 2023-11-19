use smash::{
    lua2cpp::*,
    app::{lua_bind::*, *}
};

pub unsafe extern "C" fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let situation_kind = &format!("{}", StatusModule::situation_kind(fighter.module_accessor));
        println!(
            "[Fighter Hook]\nFighter Kind: {}\nStatus Kind: {}\nSituation Kind: {}",
            utility::get_kind(&mut *fighter.module_accessor),
            StatusModule::status_kind(fighter.module_accessor),
            match StatusModule::situation_kind(fighter.module_accessor) {
                0 => "SITUATION_KIND_GROUND",
                1 => "SITUATION_KIND_CLIFF",
                2 => "SITUATION_KIND_AIR",
                _ => situation_kind
            }
        );
    }
}