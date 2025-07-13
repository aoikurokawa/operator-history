use helpers::TestBuilder;
use operator_history_core::config::Config;
use solana_signer::Signer;

mod helpers;

#[tokio::test]
async fn initialize_config() {
    let fixture = TestBuilder::new().await;

    let mut operator_history_program_client = fixture.operator_history_program_client();

    let config_admin = operator_history_program_client
        .do_initialize_config()
        .await
        .unwrap();

    let config = Config::find_program_address(&operator_history_program::id()).0;
    let config: Config = operator_history_program_client
        .get_account(&config)
        .await
        .unwrap();

    assert_eq!(config.admin(), config_admin.pubkey());
    assert_eq!(
        config.jito_restaking_program_id(),
        jito_restaking_program::id()
    );
}
