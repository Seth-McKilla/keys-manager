use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::{
        AccountHash, ActionType, AddKeyFailure, RemoveKeyFailure, SetThresholdFailure,
        UpdateKeyFailure, Weight,
    },
    runtime_args,
    system::auction,
    PublicKey, RuntimeArgs, U512,
};

mod api;
mod errors;

use api::Api;
use errors::Error;

pub fn execute() {
    let result = match Api::from_args() {
        Api::SetKeyWeight(key, weight) => set_key_weight(key, weight),
        Api::SetDeploymentThreshold(threshold) => set_threshold(ActionType::Deployment, threshold),
        Api::SetKeyManagementThreshold(threshold) => {
            set_threshold(ActionType::KeyManagement, threshold)
        }
        Api::SetAll(deployment_thereshold, key_management_threshold, accounts, weights) => {
            for (account, weight) in accounts.iter().zip(weights) {
                set_key_weight(*account, weight).unwrap_or_revert();
            }
            set_threshold(ActionType::KeyManagement, key_management_threshold).unwrap_or_revert();
            set_threshold(ActionType::Deployment, deployment_thereshold).unwrap_or_revert();
            Ok(())
        }
        Api::Delegate(delegator, validator, amount) => {
            delegate(delegator, validator, amount);
            Ok(())
        }
        Api::Undelegate(delegator, validator, amount) => {
            undelegate(delegator, validator, amount);
            Ok(())
        }
    };
    result.unwrap_or_revert()
}

fn set_key_weight(key: AccountHash, weight: Weight) -> Result<(), Error> {
    if weight.value() == 0 {
        remove_key_if_exists(key)
    } else {
        add_or_update_key(key, weight)
    }
}

fn add_or_update_key(key: AccountHash, weight: Weight) -> Result<(), Error> {
    match account::update_associated_key(key, weight) {
        Ok(()) => Ok(()),
        Err(UpdateKeyFailure::MissingKey) => add_key(key, weight),
        Err(UpdateKeyFailure::PermissionDenied) => Err(Error::PermissionDenied),
        Err(UpdateKeyFailure::ThresholdViolation) => Err(Error::ThresholdViolation),
    }
}

fn add_key(key: AccountHash, weight: Weight) -> Result<(), Error> {
    match account::add_associated_key(key, weight) {
        Ok(()) => Ok(()),
        Err(AddKeyFailure::MaxKeysLimit) => Err(Error::MaxKeysLimit),
        Err(AddKeyFailure::DuplicateKey) => Err(Error::DuplicateKey), // Should never happen.
        Err(AddKeyFailure::PermissionDenied) => Err(Error::PermissionDenied),
    }
}

fn remove_key_if_exists(key: AccountHash) -> Result<(), Error> {
    match account::remove_associated_key(key) {
        Ok(()) => Ok(()),
        Err(RemoveKeyFailure::MissingKey) => Ok(()),
        Err(RemoveKeyFailure::PermissionDenied) => Err(Error::PermissionDenied),
        Err(RemoveKeyFailure::ThresholdViolation) => Err(Error::ThresholdViolation),
    }
}

fn set_threshold(permission_level: ActionType, threshold: Weight) -> Result<(), Error> {
    match account::set_action_threshold(permission_level, threshold) {
        Ok(()) => Ok(()),
        Err(SetThresholdFailure::KeyManagementThreshold) => Err(Error::KeyManagementThresholdError),
        Err(SetThresholdFailure::DeploymentThreshold) => Err(Error::DeploymentThresholdError),
        Err(SetThresholdFailure::PermissionDeniedError) => Err(Error::PermissionDenied),
        Err(SetThresholdFailure::InsufficientTotalWeight) => Err(Error::InsufficientTotalWeight),
    }
}

fn delegate(delegator: PublicKey, validator: PublicKey, amount: U512) {
    call_auction(auction::METHOD_DELEGATE, delegator, validator, amount);
}

fn undelegate(delegator: PublicKey, validator: PublicKey, amount: U512) {
    call_auction(auction::METHOD_UNDELEGATE, delegator, validator, amount);
}

fn call_auction(method: &str, delegator: PublicKey, validator: PublicKey, amount: U512) {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    runtime::call_contract::<U512>(contract_hash, method, args);
}
