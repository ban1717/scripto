#[rustfmt::skip]
pub mod test_runner;

use crate::test_runner::TestRunner;
use radix_engine::ledger::InMemorySubstateStore;
use radix_engine::transaction::*;
use scrypto::prelude::*;
use scrypto::resource::AuthRule::{OneOf, AnyOfResource, NonFungible};

#[test]
fn can_withdraw_from_my_1_of_2_account_with_key0_sign() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(&mut substate_store);
    let (key0, non_fungible_address0) = test_runner.new_public_key_and_non_fungible_address();
    let (_, non_fungible_address1) = test_runner.new_public_key_and_non_fungible_address();
    let auth_rule_1_of_2 = OneOf(vec![NonFungible(non_fungible_address0), NonFungible(non_fungible_address1)]);
    let account = test_runner.new_account(&auth_rule_1_of_2);
    let (_, other_account) = test_runner.new_public_key_with_account();

    // Act
    let transaction = test_runner
        .new_transaction_builder()
        .withdraw_from_account(&ResourceSpecification::Fungible {
            amount: Decimal(100),
            resource_def_id: RADIX_TOKEN
        }, account)
        .call_method_with_all_resources(other_account, "deposit_batch")
        .build(vec![key0])
        .unwrap();
    let receipt = test_runner.run(transaction);

    // Assert
    assert!(receipt.result.is_ok());
}

#[test]
fn can_withdraw_from_my_1_of_2_account_with_key1_sign() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(&mut substate_store);
    let (_, non_fungible_address0) = test_runner.new_public_key_and_non_fungible_address();
    let (key1, non_fungible_address1) = test_runner.new_public_key_and_non_fungible_address();
    let auth_rule_1_of_2 = OneOf(vec![NonFungible(non_fungible_address0), NonFungible(non_fungible_address1)]);
    let account = test_runner.new_account(&auth_rule_1_of_2);
    let (_, other_account) = test_runner.new_public_key_with_account();

    // Act
    let transaction = test_runner
        .new_transaction_builder()
        .withdraw_from_account(&ResourceSpecification::Fungible {
            amount: Decimal(100),
            resource_def_id: RADIX_TOKEN
        }, account)
        .call_method_with_all_resources(other_account, "deposit_batch")
        .build(vec![key1])
        .unwrap();
    let receipt = test_runner.run(transaction);

    // Assert
    assert!(receipt.result.is_ok());
}

#[test]
fn can_withdraw_from_my_any_xrd_auth_account_with_no_signature() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(&mut substate_store);
    let xrd_auth_rule = AnyOfResource(RADIX_TOKEN);
    let account = test_runner.new_account(&xrd_auth_rule);
    let (_, other_account) = test_runner.new_public_key_with_account();

    // Act
    let transaction = test_runner
        .new_transaction_builder()
        .call_method(SYSTEM_COMPONENT, "free_xrd", vec![], None)
        .take_from_worktop(&ResourceSpecification::Fungible {
            amount: Decimal(1),
            resource_def_id: RADIX_TOKEN,
        }, |builder, bucket_id| {
            builder.create_bucket_proof(bucket_id, |builder, proof_id| {
                builder.push_auth(proof_id);
                builder.withdraw_from_account(&ResourceSpecification::Fungible {
                    amount: Decimal(100),
                    resource_def_id: RADIX_TOKEN
                }, account);
                builder.pop_auth(|builder, proof_id| builder.drop_proof(proof_id));
                builder
            });
            builder
        })
        .call_method_with_all_resources(other_account, "deposit_batch")
        .build(vec![])
        .unwrap();
    let receipt = test_runner.run(transaction);

    // Assert
    assert!(receipt.result.is_ok());
}

