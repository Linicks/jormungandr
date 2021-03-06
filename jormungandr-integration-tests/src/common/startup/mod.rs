use crate::common::{
    configuration::genesis_model::GenesisYaml,
    data::address::{Account, Delegation, Utxo},
    file_utils, jcli_wrapper,
};
use chain_addr::Discrimination;
use chain_crypto::AsymmetricKey;
use jormungandr_lib::crypto::key::KeyPair;
use std::path::PathBuf;

pub fn get_genesis_block_hash(genesis_yaml: &GenesisYaml) -> String {
    let path_to_output_block = build_genesis_block(&genesis_yaml);

    jcli_wrapper::assert_genesis_hash(&path_to_output_block)
}

pub fn build_genesis_block(genesis_yaml: &GenesisYaml) -> PathBuf {
    let input_yaml_file_path = GenesisYaml::serialize(&genesis_yaml);
    let path_to_output_block = file_utils::get_path_in_temp("block-0.bin");
    jcli_wrapper::assert_genesis_encode(&input_yaml_file_path, &path_to_output_block);

    path_to_output_block
}

pub fn create_new_utxo_address() -> Utxo {
    let private_key = jcli_wrapper::assert_key_generate_default();
    let public_key = jcli_wrapper::assert_key_to_public_default(&private_key);
    let address = jcli_wrapper::assert_address_single(&public_key, Discrimination::Test);
    let utxo = Utxo {
        private_key,
        public_key,
        address,
    };
    utxo
}

pub fn create_new_account_address() -> Account {
    let private_key = jcli_wrapper::assert_key_generate_default();
    let public_key = jcli_wrapper::assert_key_to_public_default(&private_key);
    let address = jcli_wrapper::assert_address_account(&public_key, Discrimination::Test);
    Account::new(&private_key, &public_key, &address)
}

pub fn create_new_delegation_address() -> Delegation {
    let private_delegation_key = jcli_wrapper::assert_key_generate_default();
    let public_delegation_key = jcli_wrapper::assert_key_to_public_default(&private_delegation_key);
    create_new_delegation_address_for(&public_delegation_key)
}

pub fn create_new_delegation_address_for(delegation_public_key: &str) -> Delegation {
    let private_key = jcli_wrapper::assert_key_generate_default();
    let public_key = jcli_wrapper::assert_key_to_public_default(&private_key);
    let address = jcli_wrapper::assert_address_delegation(
        &public_key,
        delegation_public_key,
        Discrimination::Test,
    );

    let utxo_with_delegation = Delegation {
        private_key: private_key,
        public_key: public_key,
        address: address,
        delegation_key: delegation_public_key.to_string(),
    };
    println!(
        "New utxo with delegation generated: {:?}",
        &utxo_with_delegation
    );
    utxo_with_delegation
}

pub fn create_new_key_pair<K: AsymmetricKey>() -> KeyPair<K> {
    KeyPair::generate(&mut rand::rngs::OsRng::new().unwrap())
}
