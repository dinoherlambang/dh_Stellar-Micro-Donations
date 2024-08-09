#![no_std]
use soroban_sdk::{contractimpl, symbol_short, vec, Env, Symbol, Vec, Address, Map, Error};

pub struct MicroDonations;

#[derive(Debug, Error)]
pub enum MicroDonationsError {
    #[error("Project not found")]
    ProjectNotFound,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Project already exists")]
    ProjectAlreadyExists,
    #[error("Donation exceeds project goal")]
    GoalExceeded,
    #[error("Unauthorized access")]
    Unauthorized,
}

#[contractimpl]
impl MicroDonations {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("projects"), &Map::<Symbol, i128>::new(&env));
    }

    pub fn create_project(env: Env, name: Symbol, goal: i128) -> Result<(), MicroDonationsError> {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap().unwrap();
        admin.require_auth();

        if goal <= 0 {
            return Err(MicroDonationsError::InvalidAmount);
        }

        let mut projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        if projects.contains_key(name) {
            return Err(MicroDonationsError::ProjectAlreadyExists);
        }

        projects.set(name, 0);
        env.storage().instance().set(&symbol_short!("projects"), &projects);
        env.storage().instance().set(&name, &goal);
        Ok(())
    }

    pub fn donate(env: Env, project: Symbol, amount: i128) -> Result<(), MicroDonationsError> {
        let mut projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        let current_amount = projects.get(project).ok_or(MicroDonationsError::ProjectNotFound)?;
        if amount <= 0 {
            return Err(MicroDonationsError::InvalidAmount);
        }
        let goal: i128 = env.storage().instance().get(&project).unwrap().unwrap();
        let new_amount = current_amount + amount;
        if new_amount > goal {
            return Err(MicroDonationsError::GoalExceeded);
        }
        projects.set(project, new_amount);
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

    pub fn withdraw(env: Env, project: Symbol, amount: i128) -> Result<(), MicroDonationsError> {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap().unwrap();
        admin.require_auth();

        let mut projects: Map<Symbol, i128> = env.storage().instance().get(&symbol_short!("projects")).unwrap().unwrap();
        let current_amount = projects.get(project).ok_or(MicroDonationsError::ProjectNotFound)?;
        
        if amount <= 0 || amount > current_amount {
            return Err(MicroDonationsError::InvalidAmount);
        }
        
        let new_amount = current_amount - amount;
        projects.set(project, new_amount);
        env.storage().instance().set(&symbol_short!("projects"), &projects);
        Ok(())
    }
}
