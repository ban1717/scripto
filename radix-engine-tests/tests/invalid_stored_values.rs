use radix_engine::errors::{InvalidOwnership, KernelError, RuntimeError};
use radix_engine::types::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn stored_bucket_in_committed_component_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/stored_values");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "InvalidInitStoredBucket",
            "create",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| match e {
        RuntimeError::KernelError(KernelError::InvalidOwnership(invalid_ownership)) => {
            if let InvalidOwnership(
                SubstateOffset::Component(ComponentOffset::State0),
                RESOURCE_MANAGER_PACKAGE,
                ..,
            ) = **invalid_ownership
            {
                return true;
            } else {
                return false;
            }
        }
        _ => false,
    });
}

#[test]
fn stored_bucket_in_owned_component_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/stored_values");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "InvalidStoredBucketInOwnedComponent",
            "create_bucket_in_owned_component",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| match e {
        RuntimeError::KernelError(KernelError::InvalidOwnership(invalid_ownership)) => {
            if let InvalidOwnership(
                SubstateOffset::Component(ComponentOffset::State0),
                RESOURCE_MANAGER_PACKAGE,
                ..,
            ) = **invalid_ownership
            {
                return true;
            } else {
                return false;
            }
        }
        _ => false,
    });
}
