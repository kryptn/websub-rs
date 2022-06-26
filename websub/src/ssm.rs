use std::{
    env,
};

use rusoto_core::Region;
use rusoto_ssm::{GetParametersRequest, Ssm, SsmClient};


pub async fn get_parameters(param_envs: Vec<&str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ssm_client = SsmClient::new(Region::default());
    let names: Vec<String> = param_envs
        .iter()
        .map(|v| env::var(v).expect("required env"))
        .collect();

    let req = GetParametersRequest {
        names,
        with_decryption: None,
    };
    let resp = ssm_client.get_parameters(req).await?;

    let mut out = Vec::new();

    if let Some(parameters) = resp.parameters {
        let p = parameters.iter().map(|p| p.clone().value.unwrap());
        out.extend(p);
    }

    Ok(out)
}