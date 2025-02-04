use crate::api::spot::SpotApi;
use crate::error::ApiFineryError;
use crate::error::LibError;
use crate::error::LibResult;
use crate::types::GetUserNameRequest;
use crate::types::GetUserNameResponse;
use crate::types::Nonce;
use crate::types::Time;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        #[deprecated = "\
            API Method \"getUsername\" was replaced with \"getSubaccounts\". \
            Consider using `.get_subaccounts()` method instead.
        "]
        pub async fn get_user_name(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<GetUserNameRequest>,
        ) -> LibResult<GetUserNameResponse> {
            let req: GetUserNameRequest = request.into();

            // Temporary Backward compatibility layer.
            let subaccs = self
                .get_subaccounts(nonce, time, crate::types::GetSubaccountsRequest {})
                .await?;

            subaccs
                .0
                .into_iter()
                .find_map(|subacc| {
                    (subacc.info.clinet_id == req.counterparty_id).then(|| GetUserNameResponse {
                        username: subacc.info.username,
                    })
                })
                .ok_or_else(|| LibError::ApiError(ApiFineryError::ClientNotFound))
        }
    }
}
