#![no_std]
use soroban_sdk::{contractimpl, symbol_short, vec, Env, Symbol, Vec, Address, Map, Error};

pub struct MicroDonations;

#[derive(Debug, Error)]
pub enum MicroDonationsError {
    #[error("Project not found")]
    ProjectNotFound,
    #[error("Invalid amount")]
    InvalidAmount,
}

#[contractimpl]
impl MicroDonations {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("projects"), &Map::<Symbol, i128>::new(&env));
    }

    pub fn create_project(env: Env, name: Symbol, goal: i128) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap().unwrap();
        admin.require_auth();

        let mut projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        projects.set(name, 0);
        env.storage().instance().set(&symbol_short!("projects"), &projects);
        env.storage().instance().set(&name, &goal);
    }

    pub fn donate(env: Env, project: Symbol, amount: i128) -> Result<(), MicroDonationsError> {
        let mut projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        let current_amount = projects.get(project).ok_or(MicroDonationsError::ProjectNotFound)?;
        if amount <= 0 {
            return Err(MicroDonationsError::InvalidAmount);
        }
        projects.set(project, current_amount + amount);
        env.storage().instance().set(&symbol_short!("projects"), &projects);
        Ok(())
    }

    pub fn get_project_status(env: Env, project: Symbol) -> Result<Vec<i128>, MicroDonationsError> {
        let projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        let current_amount = projects.get(project).ok_or(MicroDonationsError::ProjectNotFound)?;
        let goal: i128 = env.storage().instance().get(&project).unwrap().unwrap();
        Ok(vec![&env, current_amount, goal])
    }

    pub fn get_all_projects(env: Env) -> Vec<Symbol> {
        let projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        projects.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_init() {
        let env = Env::default();
        let admin = Address::random(&env);
        MicroDonations::init(&env, admin.clone());
        
        let stored_admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap().unwrap();
        assert_eq!(stored_admin, admin);
    }

    #[test]
    fn test_create_project() {
        let env = Env::default();
        let admin = Address::random(&env);
        MicroDonations::init(&env, admin.clone());

        let project_name = symbol_short!("test_project");
        let goal = 1000;

        env.mock_all_auths();

        MicroDonations::create_project(&env, project_name, goal);

        let projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        assert_eq!(projects.get(project_name).unwrap(), 0);

        let stored_goal: i128 = env.storage().instance().get(&project_name).unwrap().unwrap();
        assert_eq!(stored_goal, goal);
    }

    #[test]
    fn test_donate() {
        let env = Env::default();
        let admin = Address::random(&env);
        MicroDonations::init(&env, admin.clone());

        let project_name = symbol_short!("test_project");
        let goal = 1000;

        env.mock_all_auths();

        MicroDonations::create_project(&env, project_name, goal);
        MicroDonations::donate(&env, project_name, 500).unwrap();

        let projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        assert_eq!(projects.get(project_name).unwrap(), 500);
    }

    #[test]
    fn test_get_project_status() {
        let env = Env::default();
        let admin = Address::random(&env);
        MicroDonations::init(&env, admin.clone());

        let project_name = symbol_short!("test_project");
        let goal = 1000;

        env.mock_all_auths();

        MicroDonations::create_project(&env, project_name, goal);
        MicroDonations::donate(&env, project_name, 500).unwrap();

        let status = MicroDonations::get_project_status(&env, project_name).unwrap();
        assert_eq!(status, vec![&env, 500, 1000]);
    }

    #[test]
    fn test_get_all_projects() {
        let env = Env::default();
        let admin = Address::random(&env);
        MicroDonations::init(&env, admin.clone());

        env.mock_all_auths();

        MicroDonations::create_project(&env, symbol_short!("project1"), 1000);
        MicroDonations::create_project(&env, symbol_short!("project2"), 2000);

        let all_projects = MicroDonations::get_all_projects(&env);
        assert_eq!(all_projects.len(), 2);
        assert!(all_projects.contains(&symbol_short!("project1")));
        assert!(all_projects.contains(&symbol_short!("project2")));
    }
}