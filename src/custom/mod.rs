use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 60 == 0 {
            println!("[Fighter Hook] Frame : {}", frame);
        }
    }
}

// Use this for general per-frame weapon-level hooks
pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}