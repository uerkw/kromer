use std::sync::Arc;

use surrealdb::engine::any::Any;
use surrealdb::Surreal;

use crate::database::models::wallet::Model as Wallet;
use crate::errors::wallet::WalletError;
use crate::errors::KromerError;
use crate::models::addresses::AddressJson;
use crate::{models::auth::LoginDetails, websockets::wrapped_ws::WrappedWsData};

pub async fn perform_login(
    ws_metadata: &WrappedWsData,
    login_details: LoginDetails,
    db: Arc<Surreal<Any>>,
) -> Result<(WrappedWsData, AddressJson), KromerError> {
    // We don't necessarily care if they are logged in to a wallet already, or if they are a guest,
    // so we just want to verify that the WsMessageType has the LoginDetails struct on it,
    // and that it is valid

    // Check the wallet to verify
    let privatekey = login_details.private_key;
    let wallet = Wallet::verify(&db, privatekey.clone())
        .await
        .map_err(|_| KromerError::Wallet(WalletError::InvalidPassword))?;

    if let Some(wallet) = wallet {
        let address = wallet.address.clone();
        let new_ws_data = WrappedWsData {
            address,
            privatekey: Some(privatekey),
            ..ws_metadata.to_owned()
        };
        let wallet: AddressJson = wallet.into();
        return Ok((new_ws_data, wallet));
    }

    // Base fail case
    Err(KromerError::Wallet(WalletError::InvalidPassword))
}
