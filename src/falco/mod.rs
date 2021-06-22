use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;

#[acmd_script( agent = "falco_blaster_bullet", script = "game_fly", category = ACMD_GAME )]
unsafe fn falco_laser(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=100, FKB=20, BKB=0, Size=1.44, X=0.0, Y=0.0, Z=0.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
                Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
            AttackModule::enable_safe_pos()
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        falco_laser
    );
}