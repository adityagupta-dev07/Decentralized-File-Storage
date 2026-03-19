#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, String, Vec, Address
};

#[contracttype]
#[derive(Clone)]
pub struct File {
    pub owner: Address,
    pub file_hash: String,
    pub file_name: String,
}

#[contract]
pub struct FileStorageContract;

#[contractimpl]
impl FileStorageContract {

    pub fn upload_file(env: Env, owner: Address, file_hash: String, file_name: String) {
        owner.require_auth();

        let key = Symbol::new(&env, "FILES");

        let mut files: Vec<File> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        let file = File {
            owner: owner.clone(),
            file_hash,
            file_name,
        };

        files.push_back(file);
        env.storage().instance().set(&key, &files);
    }

    pub fn get_files(env: Env) -> Vec<File> {
        let key = Symbol::new(&env, "FILES");
        env.storage().instance().get(&key).unwrap_or(Vec::new(&env))
    }

    pub fn get_files_by_owner(env: Env, owner: Address) -> Vec<File> {
        let key = Symbol::new(&env, "FILES");

        let files: Vec<File> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for file in files.iter() {
            if file.owner == owner {
                result.push_back(file);
            }
        }

        result
    }
}

mod test;