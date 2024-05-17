use rhaki_cw_plus::deploy::{
    cosmos_grpc_client::GrpcClient, functions::deploy_create_wallet, tokio, Deploier,
};

use {{crate_name}}_deploy::data::Data;

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() {
    let data = Data::read_data().unwrap();

    let mut grpc = GrpcClient::new(&data.chain_info.grpc).await.unwrap();
    let _wallet = deploy_create_wallet(&mut grpc, &data.chain_info)
        .await
        .unwrap();

    todo!("Insert your decode here")
}
