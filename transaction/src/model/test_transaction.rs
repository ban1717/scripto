use crate::model::*;
use radix_engine_interface::blueprints::resource::NonFungibleGlobalId;
use radix_engine_interface::crypto::hash;
use radix_engine_interface::data::manifest::*;
use radix_engine_interface::*;
use sbor::rust::vec::Vec;
use std::collections::BTreeSet;

#[derive(ManifestSbor)]
pub struct TestTransaction {
    nonce: u64,
    cost_unit_limit: u32,
    manifest: TransactionManifest,
}

impl TestTransaction {
    pub fn new(manifest: TransactionManifest, nonce: u64, cost_unit_limit: u32) -> Self {
        Self {
            nonce,
            cost_unit_limit,
            manifest,
        }
    }

    pub fn get_executable<'a>(
        &'a self,
        initial_proofs: Vec<NonFungibleGlobalId>,
    ) -> Executable<'a> {
        let payload = manifest_encode(self).unwrap();
        let payload_size = payload.len();
        let transaction_hash = hash(payload);

        Executable::new(
            self.manifest.instructions.clone(),
            &self.manifest.blobs,
            ExecutionContext {
                transaction_hash,
                payload_size,
                auth_zone_params: AuthZoneParams {
                    initial_proofs,
                    virtual_resources: BTreeSet::new(),
                },
                fee_payment: FeePayment::User {
                    cost_unit_limit: self.cost_unit_limit,
                    tip_percentage: 0,
                },
                runtime_validations: vec![],
                pre_allocated_ids: BTreeSet::new(),
            },
        )
    }
}
