use tonic::transport::Channel;

use crate::authentication_proto;
use crate::authentication_proto::authentication_service_client::AuthenticationServiceClient;

#[derive(Debug, Clone)]
pub struct AuthClient {
    authentication_client: AuthenticationServiceClient<Channel>,
}

impl AuthClient {
    pub async fn new(addr: &'static str) -> anyhow::Result<Self> {
        tracing::info!("Establishing connection to authentication service...");

        let channel = loop {
            match Channel::from_static(addr).connect().await {
                Ok(channel) => break channel,
                Err(error) => {
                    tracing::error!("Failed to connect to authentication service: {error:?}");
                    tracing::info!("Trying to connect to the authentication service again...");
                }
            }
        };

        tracing::info!("Connection to authentication service established successfully");

        let authentication_client = AuthenticationServiceClient::new(channel);

        Ok(Self {
            authentication_client,
        })
    }

    pub async fn login(
        &mut self,
        req: authentication_proto::LoginRequest,
    ) -> Result<authentication_proto::LoginResponse, tonic::Status> {
        let req = tonic::Request::new(req);
        let res = self.authentication_client.login(req).await?.into_inner();

        Ok(res)
    }

    pub async fn register(
        &mut self,
        req: authentication_proto::RegisterRequest,
    ) -> Result<authentication_proto::RegisterResponse, tonic::Status> {
        let req = tonic::Request::new(req);
        let res = self.authentication_client.register(req).await?.into_inner();

        Ok(res)
    }

    pub async fn validate(
        &mut self,
        req: authentication_proto::ValidateRequest,
    ) -> Result<authentication_proto::ValidateResponse, tonic::Status> {
        let req = tonic::Request::new(req);
        let res = self.authentication_client.validate(req).await?.into_inner();

        Ok(res)
    }

    pub async fn refresh(
        &mut self,
        req: authentication_proto::RefreshRequest,
    ) -> Result<authentication_proto::RefreshResponse, tonic::Status> {
        let req = tonic::Request::new(req);
        let res = self.authentication_client.refresh(req).await?.into_inner();

        Ok(res)
    }

    pub async fn logout(
        &mut self,
        req: authentication_proto::LogoutRequest,
    ) -> Result<(), tonic::Status> {
        let req = tonic::Request::new(req);
        let res = self.authentication_client.logout(req).await?.into_inner();

        Ok(res)
    }

    pub async fn logout_all(
        &mut self,
        req: authentication_proto::LogoutAllRequest,
    ) -> Result<(), tonic::Status> {
        let req = tonic::Request::new(req);
        let res = self
            .authentication_client
            .logout_all(req)
            .await?
            .into_inner();

        Ok(res)
    }
}
