mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario_fireball");
    acmd::install(agent);
    agent.install();
}