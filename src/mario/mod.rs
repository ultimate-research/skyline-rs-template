// mario
mod acmd;
mod status;
mod frame;

// mario_fireball
mod fireball;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    fireball::install();
}
