use soroban_sdk::{testutils::Address as _, Env, Symbol};
use micro_donations::{MicroDonations, MicroDonationsError};

#[test]
fn test_init() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());
    
    let stored_admin: Address = env.storage().instance().get(&Symbol::short("admin")).unwrap().unwrap();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_create_project() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 1000;

    env.mock_all_auths();

    MicroDonations::create_project(&env, project_name, goal).unwrap();

    let projects: Map<Symbol, i128> = env.storage().instance().get(&Symbol::short("projects")).unwrap().unwrap();
    assert_eq!(projects.get(project_name).unwrap(), 0);

    let stored_goal: i128 = env.storage().instance().get(&project_name).unwrap().unwrap();
    assert_eq!(stored_goal, goal);
}

#[test]
fn test_create_project_invalid_goal() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 0;

    env.mock_all_auths();

    let result = MicroDonations::create_project(&env, project_name, goal);
    assert!(matches!(result, Err(MicroDonationsError::InvalidAmount)));
}

#[test]
fn test_create_duplicate_project() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 1000;

    env.mock_all_auths();

    MicroDonations::create_project(&env, project_name, goal).unwrap();
    let result = MicroDonations::create_project(&env, project_name, goal);
    assert!(matches!(result, Err(MicroDonationsError::ProjectAlreadyExists)));
}

#[test]
fn test_donate() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 1000;

    env.mock_all_auths();

    MicroDonations::create_project(&env, project_name, goal).unwrap();
    MicroDonations::donate(&env, project_name, 500).unwrap();

    let projects: Map<Symbol, i128> = env.storage().instance().get(&Symbol::short("projects")).unwrap().unwrap();
    assert_eq!(projects.get(project_name).unwrap(), 500);
}

#[test]
fn test_donate_exceed_goal() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 1000;

    env.mock_all_auths();

    MicroDonations::create_project(&env, project_name, goal).unwrap();
    MicroDonations::donate(&env, project_name, 500).unwrap();
    let result = MicroDonations::donate(&env, project_name, 501);
    assert!(matches!(result, Err(MicroDonationsError::GoalExceeded)));
}

#[test]
fn test_get_project_status() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 1000;

    env.mock_all_auths();

    MicroDonations::create_project(&env, project_name, goal).unwrap();
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

    MicroDonations::create_project(&env, Symbol::short("project1"), 1000).unwrap();
    MicroDonations::create_project(&env, Symbol::short("project2"), 2000).unwrap();

    let all_projects = MicroDonations::get_all_projects(&env);
    assert_eq!(all_projects.len(), 2);
    assert!(all_projects.contains(&Symbol::short("project1")));
    assert!(all_projects.contains(&Symbol::short("project2")));
}

#[test]
fn test_withdraw() {
    let env = Env::default();
    let admin = Address::random(&env);
    MicroDonations::init(&env, admin.clone());

    let project_name = Symbol::short("test_project");
    let goal = 1000;

    env.mock_all_auths();

    MicroDonations::create_project(&env, project_name, goal).unwrap();
    MicroDonations::donate(&env, project_name, 500).unwrap();
    MicroDonations::withdraw(&env, project_name, 200).unwrap();

    let projects: Map<Symbol, i128> = env.storage().instance().get(&Symbol::short("projects")).unwrap().unwrap();
    assert_eq!(projects.get(project_name).unwrap(), 300);
}