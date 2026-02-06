use aws_config::BehaviorVersion;
use aws_sdk_emr::Client;
use aws_sdk_emr::Error;
use aws_sdk_emr::types::{
    Application, BootstrapActionConfig, Configuration, JobFlowInstancesConfig,
    ScriptBootstrapActionConfig,
};
use tracing::info;

/// Get an EMR client
pub async fn get_emr_client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2026_01_12()).await;
    let client = Client::new(&config);
    Ok(client)
}

/// Get the status of an EMR cluster
pub async fn get_emr_cluster_status(client: &Client, cluster_id: &str) -> Result<String, Error> {
    let describe_cluster_output = client
        .describe_cluster()
        .cluster_id(cluster_id)
        .send()
        .await?;
    let cluster_status = describe_cluster_output
        .cluster()
        .unwrap()
        .status()
        .unwrap()
        .state()
        .unwrap()
        .to_string();
    Ok(cluster_status)
}

pub async fn wait_for_emr_cluster_to_be_ready(
    client: &Client,
    cluster_id: &str,
) -> Result<(), Error> {
    loop {
        let cluster_status = get_emr_cluster_status(client, cluster_id).await?;
        match cluster_status.as_str() {
            "WAITING" | "RUNNING" => {
                println!("EMR cluster is ready.");
                return Ok(());
            }
            _ => {
                println!("Waiting for EMR cluster to be ready...");
            }
        }
    }
}

/// Create an EMR cluster
pub async fn create_emr_cluster(
    client: &Client,
    name: &str,
    instance_type: &str,
    instance_count: i32,
) -> Result<(), Error> {
    let client = get_emr_client().await?;

    // 2. 构建 EMR 集群配置
    let cluster_name = "my-rust-emr-cluster";

    // 实例配置
    let instances = JobFlowInstancesConfig::builder()
        .master_instance_type("m5.xlarge")
        .slave_instance_type("m5.xlarge")
        .instance_count(3) // 1 master + 2 core
        .ec2_subnet_id("subnet-12345678") // 替换为你的子网 ID
        .keep_job_flow_alive_when_no_steps(true)
        .build();

    // 启用的应用（如 Spark, Hive）
    let applications = vec![
        Application::builder().name("Spark").build(),
        Application::builder().name("Hive").build(),
    ];

    // 可选：引导操作（Bootstrap Actions）
    let bootstrap_actions = vec![
        BootstrapActionConfig::builder()
            .name("Install custom package")
            .script_bootstrap_action(
                ScriptBootstrapActionConfig::builder()
                    .path("s3://my-bucket/bootstrap.sh")
                    .build(),
            )
            .build(),
    ];

    // 可选：自定义配置（如 Spark 默认参数）
    let configurations = vec![
        Configuration::builder()
            .classification("spark-defaults")
            .build(),
    ];

    // 3. 发起创建请求
    let resp = client
        .run_job_flow()
        .name(cluster_name)
        .release_label("emr-7.12.0") // 必须指定 EMR 版本
        .instances(instances)
        .set_applications(Some(applications))
        .set_bootstrap_actions(Some(bootstrap_actions))
        .set_configurations(Some(configurations))
        // 可选：服务角色（必须有足够权限）
        .service_role("EMR_DefaultRole")
        .job_flow_role("EMR_EC2_DefaultRole")
        // 可选：日志 URI
        .log_uri("s3://my-emr-logs-bucket/")
        .send()
        .await?;

    // 4. 输出结果
    info!("✅ EMR 集群创建成功!");
    info!("Cluster ID: {}", resp.job_flow_id.unwrap());
    info!("Cluster ARN: {}", resp.cluster_arn.unwrap_or_default());

    Ok(())
}
