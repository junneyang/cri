use crate::{
    criapi::{self, runtime_service_server::RuntimeService},
    sandbox::{pinned::PinnedSandbox, SandboxBuilder, SandboxDataBuilder},
};
use log::{debug, info};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MyRuntime {}

#[tonic::async_trait]
impl RuntimeService for MyRuntime {
    async fn version(
        &self,
        _request: Request<criapi::VersionRequest>,
    ) -> Result<Response<criapi::VersionResponse>, Status> {
        let resp = criapi::VersionResponse {
            version: "0.1.0".into(),
            runtime_api_version: "v1alpha1".into(),
            runtime_name: "crust".into(),
            runtime_version: "0.0.1".into(),
        };
        Ok(Response::new(resp))
    }

    async fn create_container(
        &self,
        _request: Request<criapi::CreateContainerRequest>,
    ) -> Result<Response<criapi::CreateContainerResponse>, Status> {
        let resp = criapi::CreateContainerResponse {
            container_id: "stub".into(),
        };
        Ok(Response::new(resp))
    }

    async fn start_container(
        &self,
        _request: Request<criapi::StartContainerRequest>,
    ) -> Result<Response<criapi::StartContainerResponse>, Status> {
        let resp = criapi::StartContainerResponse {};
        Ok(Response::new(resp))
    }

    async fn stop_container(
        &self,
        _request: Request<criapi::StopContainerRequest>,
    ) -> Result<Response<criapi::StopContainerResponse>, Status> {
        let resp = criapi::StopContainerResponse {};
        Ok(Response::new(resp))
    }

    async fn remove_container(
        &self,
        _request: Request<criapi::RemoveContainerRequest>,
    ) -> Result<Response<criapi::RemoveContainerResponse>, Status> {
        let resp = criapi::RemoveContainerResponse {};
        Ok(Response::new(resp))
    }

    async fn list_containers(
        &self,
        _request: Request<criapi::ListContainersRequest>,
    ) -> Result<Response<criapi::ListContainersResponse>, Status> {
        let resp = criapi::ListContainersResponse {
            containers: Vec::new(),
        };
        Ok(Response::new(resp))
    }

    async fn container_status(
        &self,
        _request: Request<criapi::ContainerStatusRequest>,
    ) -> Result<Response<criapi::ContainerStatusResponse>, Status> {
        let resp = criapi::ContainerStatusResponse {
            info: HashMap::new(),
            status: None,
        };
        Ok(Response::new(resp))
    }

    async fn container_stats(
        &self,
        _request: Request<criapi::ContainerStatsRequest>,
    ) -> Result<Response<criapi::ContainerStatsResponse>, Status> {
        let resp = criapi::ContainerStatsResponse { stats: None };
        Ok(Response::new(resp))
    }

    async fn list_container_stats(
        &self,
        _request: Request<criapi::ListContainerStatsRequest>,
    ) -> Result<Response<criapi::ListContainerStatsResponse>, Status> {
        let resp = criapi::ListContainerStatsResponse { stats: Vec::new() };
        Ok(Response::new(resp))
    }

    async fn update_container_resources(
        &self,
        _request: Request<criapi::UpdateContainerResourcesRequest>,
    ) -> Result<Response<criapi::UpdateContainerResourcesResponse>, Status> {
        let resp = criapi::UpdateContainerResourcesResponse {};
        Ok(Response::new(resp))
    }

    async fn reopen_container_log(
        &self,
        _request: Request<criapi::ReopenContainerLogRequest>,
    ) -> Result<Response<criapi::ReopenContainerLogResponse>, Status> {
        let resp = criapi::ReopenContainerLogResponse {};
        Ok(Response::new(resp))
    }

    async fn exec_sync(
        &self,
        _request: Request<criapi::ExecSyncRequest>,
    ) -> Result<Response<criapi::ExecSyncResponse>, Status> {
        let resp = criapi::ExecSyncResponse {
            exit_code: -1,
            stderr: Vec::new(),
            stdout: Vec::new(),
        };
        Ok(Response::new(resp))
    }

    async fn exec(
        &self,
        _request: Request<criapi::ExecRequest>,
    ) -> Result<Response<criapi::ExecResponse>, Status> {
        let resp = criapi::ExecResponse { url: "url".into() };
        Ok(Response::new(resp))
    }

    async fn attach(
        &self,
        _request: Request<criapi::AttachRequest>,
    ) -> Result<Response<criapi::AttachResponse>, Status> {
        let resp = criapi::AttachResponse { url: "url".into() };
        Ok(Response::new(resp))
    }
    async fn port_forward(
        &self,
        _request: Request<criapi::PortForwardRequest>,
    ) -> Result<Response<criapi::PortForwardResponse>, Status> {
        let resp = criapi::PortForwardResponse { url: "url".into() };
        Ok(Response::new(resp))
    }

    async fn run_pod_sandbox(
        &self,
        request: Request<criapi::RunPodSandboxRequest>,
    ) -> Result<Response<criapi::RunPodSandboxResponse>, Status> {
        // Take the pod sandbox config
        let config = request
            .into_inner()
            .config
            .take()
            .ok_or_else(|| Status::invalid_argument("no pod sandbox config provided"))?;

        // Verify that the metadata exists
        let metadata = config
            .metadata
            .ok_or_else(|| Status::invalid_argument("no pod sandbox metadata provided"))?;

        // Build a new sandbox from it
        let mut sandbox = SandboxBuilder::<PinnedSandbox>::default()
            .data(
                SandboxDataBuilder::default()
                    .id(metadata.uid)
                    .name(metadata.name)
                    .namespace(metadata.namespace)
                    .attempt(metadata.attempt)
                    .build()
                    .map_err(|e| {
                        Status::internal(format!("build sandbox data from metadata: {}", e))
                    })?,
            )
            .build()
            .map_err(|e| Status::internal(format!("build sandbox from config: {}", e)))?;

        debug!("Created pod sandbox {:?}", sandbox);

        // Run the sandbox
        sandbox
            .run()
            .map_err(|e| Status::internal(format!("run pod sandbox: {}", e)))?;
        info!("Started pod sandbox {}", sandbox);

        // Build and return the response
        let reply = criapi::RunPodSandboxResponse {
            pod_sandbox_id: sandbox.id().into(),
        };
        Ok(Response::new(reply))
    }

    async fn stop_pod_sandbox(
        &self,
        _request: Request<criapi::StopPodSandboxRequest>,
    ) -> Result<Response<criapi::StopPodSandboxResponse>, Status> {
        let reply = criapi::StopPodSandboxResponse {};
        Ok(Response::new(reply))
    }

    async fn remove_pod_sandbox(
        &self,
        _request: Request<criapi::RemovePodSandboxRequest>,
    ) -> Result<Response<criapi::RemovePodSandboxResponse>, Status> {
        let reply = criapi::RemovePodSandboxResponse {};
        Ok(Response::new(reply))
    }

    async fn list_pod_sandbox(
        &self,
        _request: Request<criapi::ListPodSandboxRequest>,
    ) -> Result<Response<criapi::ListPodSandboxResponse>, Status> {
        let reply = criapi::ListPodSandboxResponse { items: Vec::new() };
        Ok(Response::new(reply))
    }

    async fn pod_sandbox_status(
        &self,
        _request: Request<criapi::PodSandboxStatusRequest>,
    ) -> Result<Response<criapi::PodSandboxStatusResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn status(
        &self,
        _request: Request<criapi::StatusRequest>,
    ) -> Result<Response<criapi::StatusResponse>, Status> {
        let resp = criapi::StatusResponse {
            status: None,
            info: HashMap::new(),
        };
        Ok(Response::new(resp))
    }

    async fn update_runtime_config(
        &self,
        _request: Request<criapi::UpdateRuntimeConfigRequest>,
    ) -> Result<Response<criapi::UpdateRuntimeConfigResponse>, Status> {
        let resp = criapi::UpdateRuntimeConfigResponse {};
        Ok(Response::new(resp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn run_pod_sandbox_success() {
        let sut = MyRuntime::default();
        let test_id = "123";
        let request = criapi::RunPodSandboxRequest {
            config: Some(criapi::PodSandboxConfig {
                metadata: Some(criapi::PodSandboxMetadata {
                    name: "".into(),
                    uid: test_id.into(),
                    namespace: "".into(),
                    attempt: 0,
                }),
                hostname: "".into(),
                log_directory: "".into(),
                dns_config: None,
                port_mappings: vec![],
                labels: HashMap::new(),
                annotations: HashMap::new(),
                linux: None,
            }),
            runtime_handler: "".into(),
        };
        let response = sut.run_pod_sandbox(Request::new(request)).await.unwrap();
        assert_eq!(response.get_ref().pod_sandbox_id, test_id);
    }

    #[tokio::test]
    async fn run_pod_sandbox_fail_no_config() {
        let sut = MyRuntime::default();
        let request = criapi::RunPodSandboxRequest {
            config: None,
            runtime_handler: "".into(),
        };
        let response = sut.run_pod_sandbox(Request::new(request)).await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn run_pod_sandbox_fail_no_config_metadata() {
        let sut = MyRuntime::default();
        let request = criapi::RunPodSandboxRequest {
            config: Some(criapi::PodSandboxConfig {
                metadata: None,
                hostname: "".into(),
                log_directory: "".into(),
                dns_config: None,
                port_mappings: vec![],
                labels: HashMap::new(),
                annotations: HashMap::new(),
                linux: None,
            }),
            runtime_handler: "".into(),
        };
        let response = sut.run_pod_sandbox(Request::new(request)).await;
        assert!(response.is_err());
    }
}
