mod specials;

pub fn install(agent: &mut smashline::Agent) {
    specials::install(agent);
}