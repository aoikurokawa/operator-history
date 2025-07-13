use helpers::TestBuilder;
use jito_restaking_core::operator::Operator;
use operator_history_core::{
    circ_buf::CircBuf, operator_history::OperatorHistory, MAX_REALLOC_BYTES,
};

pub mod helpers;

#[tokio::test]
async fn realloc_operator_history_account() {
    let mut fixture = TestBuilder::new().await;

    let mut jito_restaking_program_client = fixture.jito_restaking_program_client();
    let mut operator_history_program_client = fixture.operator_history_program_client();

    jito_restaking_program_client
        .do_initialize_config()
        .await
        .unwrap();
    let operator_root = jito_restaking_program_client
        .do_initialize_operator()
        .await
        .unwrap();

    operator_history_program_client
        .do_initialize_config()
        .await
        .unwrap();
    operator_history_program_client
        .do_initialize_operator_history_account(&operator_root.operator_pubkey)
        .await
        .unwrap();

    let operator_history_pubkey = OperatorHistory::find_program_address(
        &operator_history_program::id(),
        &operator_root.operator_pubkey,
    )
    .0;

    let raw_account = fixture
        .get_raw_account(&operator_history_pubkey)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(raw_account.data.len(), MAX_REALLOC_BYTES as usize);
    assert_eq!(raw_account.owner, operator_history_program::id());
    assert_eq!(raw_account.data[0], 0);

    for _ in 0..20 {
        operator_history_program_client
            .do_realloc_operator_history_account(&operator_root.operator_pubkey)
            .await
            .unwrap();
    }
    let operator_history_acc: OperatorHistory = operator_history_program_client
        .get_account(&operator_history_pubkey)
        .await
        .unwrap();
    let operator: Operator = jito_restaking_program_client
        .get_operator(&operator_root.operator_pubkey)
        .await
        .unwrap();

    assert_eq!(
        operator_history_acc.operator_account(),
        operator_root.operator_pubkey
    );
    assert_eq!(operator_history_acc.index(), operator.index());
    assert_eq!(operator_history_acc.struct_version(), 0);
    assert_eq!(operator_history_acc.history(), CircBuf::default());
}
