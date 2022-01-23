use trankeel_data::Advertisement;
use trankeel_data::Tenant;

#[derive(Default)]
pub struct State {
    pub advertisements: Vec<Advertisement>,
    pub tenants: Vec<Tenant>,
}
