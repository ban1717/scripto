use radix_engine::blueprints::resource::FungibleResourceManagerError;
use radix_engine::errors::{ApplicationError, RuntimeError};
use radix_engine::types::blueprints::resource::ResourceMethodAuthKey;
use radix_engine::types::*;
use radix_engine_interface::blueprints::resource::FromPublicKey;
use scrypto::prelude::Mutability::LOCKED;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_set_mintable_with_self_resource_address() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key, _, _) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/resource");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "ResourceTest",
            "set_mintable_with_self_resource_address",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn test_resource_manager() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/resource");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "ResourceTest",
            "create_fungible",
            manifest_args!(),
        )
        .call_function(package_address, "ResourceTest", "query", manifest_args!())
        .call_function(package_address, "ResourceTest", "burn", manifest_args!())
        .call_function(
            package_address,
            "ResourceTest",
            "update_resource_metadata",
            manifest_args!(),
        )
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn mint_with_bad_granularity_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/resource");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "ResourceTest",
            "create_fungible_and_mint",
            manifest_args!(0u8, dec!("0.1")),
        )
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_specific_failure(|e| {
        if let RuntimeError::ApplicationError(ApplicationError::ResourceManagerError(
            FungibleResourceManagerError::InvalidAmount(amount, granularity),
        )) = e
        {
            amount.eq(&dec!("0.1")) && *granularity == 0
        } else {
            false
        }
    });
}

#[test]
fn create_fungible_too_high_granularity_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key, _, _) = test_runner.new_allocated_account();
    let _package_address = test_runner.compile_and_publish("./tests/blueprints/resource");
    let mut access_rules = BTreeMap::new();
    access_rules.insert(ResourceMethodAuthKey::Withdraw, (rule!(allow_all), LOCKED));
    access_rules.insert(ResourceMethodAuthKey::Deposit, (rule!(allow_all), LOCKED));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .create_fungible_resource(23u8, BTreeMap::new(), access_rules, Some(dec!("100")))
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_specific_failure(|e| {
        if let RuntimeError::ApplicationError(ApplicationError::ResourceManagerError(
            FungibleResourceManagerError::InvalidDivisibility(granularity),
        )) = e
        {
            *granularity == 23u8
        } else {
            false
        }
    });
}

#[test]
fn mint_too_much_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/resource");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "ResourceTest",
            "create_fungible_and_mint",
            manifest_args!(0u8, dec!("1000000000000000001")),
        )
        .call_method(
            account,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::ResourceManagerError(
                FungibleResourceManagerError::MaxMintAmountExceeded
            ))
        )
    })
}
