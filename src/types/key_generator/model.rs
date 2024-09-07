use crate::types::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct KeyGeneratorAddressListModel;

impl KeyGeneratorAddressListModel {
    const ID: &'static str = stringify!(KeyGeneratorAddressListModel);

    pub fn get() -> Result<KeyGeneratorAddressList, KvStoreError> {
        let key = &(Self::ID);
        kvstore()?.get(key)
    }

    pub fn get_or_default() -> Result<KeyGeneratorAddressList, KvStoreError> {
        let key = &(Self::ID);

        kvstore()?.get_or_default(key)
    }

    pub fn put(key_generator_address_list: &KeyGeneratorAddressList) -> Result<(), KvStoreError> {
        let key = &(Self::ID);
        kvstore()?.put(key, key_generator_address_list)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct KeyGeneratorModel;

impl KeyGeneratorModel {
    const ID: &'static str = stringify!(KeyGeneratorModel);

    pub fn get(address: &Address) -> Result<KeyGenerator, KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.get(&key)
    }

    pub fn put(key_generator: &KeyGenerator) -> Result<(), KvStoreError> {
        let key = (Self::ID, key_generator.address());
        kvstore()?.put(&key, key_generator)
    }
}
