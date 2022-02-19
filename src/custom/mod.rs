use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if StatusModule::is_situation_changed(module_accessor) {
            let situation_kind = &format!("{}", StatusModule::situation_kind(module_accessor));
            println!(
                "[Fighter Hook]\nFighter Kind: {}\nStatus Kind: {}\nSituation Kind: {}",
                get_kind(module_accessor),
                StatusModule::status_kind(module_accessor),
                match StatusModule::situation_kind(module_accessor) {
                    0 => "SITUATION_KIND_GROUND",
                    1 => "SITUATION_KIND_CLIFF",
                    2 => "SITUATION_KIND_AIR",
                    _ => situation_kind
                }
            );
        }
    }
}

// Use this for general per-frame weapon-level hooks
#[weapon_frame_callback]
pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        global_weapon_frame
    );
}