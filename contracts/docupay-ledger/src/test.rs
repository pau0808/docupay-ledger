#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn test_happy_path_upload_and_pay() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DocuPayContract);
    let client = DocuPayContractClient::new(&env, &contract_id);

    let employee = Address::generate(&env);

    client.upload_doc(&employee, &Symbol::short("doc1"));
    client.pay_salary(&0);

    let record = client.get_record(&0);

    assert_eq!(record.employee, employee);
    assert_eq!(record.doc_hash, Symbol::short("doc1"));
    assert_eq!(record.paid, true);
}

#[test]
#[should_panic]
fn test_invalid_record_id_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DocuPayContract);
    let client = DocuPayContractClient::new(&env, &contract_id);

    client.get_record(&99);
}

#[test]
fn test_state_after_upload() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DocuPayContract);
    let client = DocuPayContractClient::new(&env, &contract_id);

    let employee = Address::generate(&env);

    client.upload_doc(&employee, &Symbol::short("doc2"));

    let record = client.get_record(&0);

    assert_eq!(record.employee, employee);
    assert_eq!(record.doc_hash, Symbol::short("doc2"));
    assert_eq!(record.paid, false);
}

#[test]
fn test_multiple_document_records() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DocuPayContract);
    let client = DocuPayContractClient::new(&env, &contract_id);

    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);

    client.upload_doc(&employee1, &Symbol::short("docA"));
    client.upload_doc(&employee2, &Symbol::short("docB"));

    let record1 = client.get_record(&0);
    let record2 = client.get_record(&1);

    assert_eq!(record1.doc_hash, Symbol::short("docA"));
    assert_eq!(record2.doc_hash, Symbol::short("docB"));
}

#[test]
fn test_payment_status_changes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DocuPayContract);
    let client = DocuPayContractClient::new(&env, &contract_id);

    let employee = Address::generate(&env);

    client.upload_doc(&employee, &Symbol::short("doc3"));

    let before = client.get_record(&0);
    assert_eq!(before.paid, false);

    client.pay_salary(&0);

    let after = client.get_record(&0);
    assert_eq!(after.paid, true);
}