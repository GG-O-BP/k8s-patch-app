use kube::{api::{Api, PatchParams, Patch}, Client};
use kube::config::Config;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::serde_json;
use getopts::Options;
use std::env;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("n", "", "set deployment name", "NAME");
    opts.optopt("p", "", "set new replica count", "COUNT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    let deployment_name = matches.opt_str("n").expect("Missing -n option").trim_start_matches('=').to_string();
    let new_replica_count_str = matches.opt_str("p").expect("Missing -p option").trim_start_matches('=').to_string();
    let patch_operations: Vec<serde_json::Value> = serde_json::from_str(&new_replica_count_str).expect("Failed to parse -p option as JSON");

    let config = Config::infer().await.map_err(kube::Error::InferConfig)?;
    let client = Client::try_from(config)?;
    let deployments: Api<Deployment> = Api::namespaced(client, "default");

    let patch_params = PatchParams::apply("Implementation");
    for operation in patch_operations {
        let operation = vec![operation];
        let patch_operation: json_patch::Patch = serde_json::from_value(serde_json::Value::Array(operation)).unwrap();
        let patch_payload: Patch<json_patch::Patch> = Patch::Json(patch_operation);
        let patch = deployments.patch(&deployment_name, &patch_params, &patch_payload).await?; 
        println!("{:#?}", patch);
    }
    Ok(())
}