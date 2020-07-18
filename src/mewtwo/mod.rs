use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use acmd::{acmd, acmd_func};

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MEWTWO, 
    animation = "special_hi",
	animcmd = "game_specialhi")]
pub fn mewtwo_upb(fighter: &mut L2CFighterCommon) {
	acmd!({
		if (is_excute){
			GroundModule::select_cliff_hangdata(*FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		frame(15)
		if (is_excute){
			if (((PostureModule::lr(module_accessor))*(ControlModule::get_stick_x(module_accessor))) < 0.0) {
				PostureModule::reverse_lr();
				PostureModule::update_rot_y_lr();
			}
		}
		frame(16)
		if (is_excute) {
			StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL, false);
		}
	});
}

#[acmd::acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MEWTWO, 
    animation = "special_air_hi",
	animcmd = "game_specialairhi")]
pub fn mewtwo_air_upb(fighter: &mut L2CFighterCommon) {
	acmd!({
		if (is_excute){
			GroundModule::select_cliff_hangdata(*FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		frame(14)
		if (is_excute){
			if (((PostureModule::lr(module_accessor))*(ControlModule::get_stick_x(module_accessor))) < 0.0) {
				PostureModule::reverse_lr();
				PostureModule::update_rot_y_lr();
			}
		}
		frame(15)
		if (is_excute) {
			StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL, false);
		}
	});
}

pub fn install() {
    acmd::add_hooks!(
        mewtwo_upb,
        mewtwo_air_upb
    );
}
