use radix_engine_interface::api::types::*;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::package::{
    PackageSetRoyaltyConfigInput, PACKAGE_SET_ROYALTY_CONFIG_IDENT,
};
use radix_engine_interface::data::scrypto::model::PackageAddress;
use radix_engine_interface::data::scrypto::{scrypto_encode, ScryptoDecode};
use sbor::rust::collections::BTreeMap;
use sbor::rust::fmt::Debug;
use sbor::rust::string::String;

#[derive(Debug)]
pub struct BorrowedPackage(pub(crate) PackageAddress);

impl BorrowedPackage {
    pub fn sys_set_royalty_config<Y, E: Debug + ScryptoDecode>(
        &self,
        royalty_config: BTreeMap<String, RoyaltyConfig>,
        api: &mut Y,
    ) -> Result<&Self, E>
    where
        Y: ClientApi<E>,
    {
        api.call_module_method(
            RENodeId::GlobalObject(self.0.into()),
            NodeModuleId::SELF,
            PACKAGE_SET_ROYALTY_CONFIG_IDENT,
            scrypto_encode(&PackageSetRoyaltyConfigInput { royalty_config }).unwrap(),
        )?;

        Ok(self)
    }
}
