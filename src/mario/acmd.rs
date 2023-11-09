mod aerials;
mod throws;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    throws::install(agent);
}