use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn mario_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(16)
        AttackModule::clear_all() // clear all previous hitboxes
        if(is_execute) {
            ATTACK(
                ID=0, Part=0, Bone=hash40("arml"), 19.0, 361, 80, 0, 30, 113.0, 3.2, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
                ATTACK_SETOFF_KIND_ON, ATTACK_LR_CHECK_F, ATTACK_LR_CHECK_F, ATTACK_LR_CHECK_F,
                ATTACK_LR_CHECK_F, ATTACK_LR_CHECK_F, false, false, false, false, true,
                COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL,
                false, hash40("collision_attr_fire"), ATTACK_SOUND_LEVEL_M, COLLISION_SOUND_ATTR_PUNCH,
                ATTACK_REGION_PUNCH
            )
            rust {
                println!("Fair frame 16");
            }
        }
        sv_kinetic_energy::add_speed(1.0);

    });
}

pub fn install() {
    acmd::add_hooks!(
        mario_fair,
    );
}
