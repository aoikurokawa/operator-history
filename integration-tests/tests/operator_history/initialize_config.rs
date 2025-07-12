#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_account::Account;
    use solana_instruction::{AccountMeta, Instruction};
    use solana_pubkey::Pubkey;

    #[test]
    fn test_initialize_config() {
        let program_id = Pubkey::new_unique();
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();

        // let instruction = Instruction::new_with_bytes(
        //     program_id,
        //     &[],
        //     vec![
        //         AccountMeta::new(key1, false),
        //         AccountMeta::new_readonly(key2, false),
        //     ],
        // );

        // let accounts = vec![(key1, Account::default()), (key2, Account::default())];

        // let mollusk = Mollusk::new(&program_id, "my_program");

        // Execute the instruction and get the result.
        // let result = mollusk.process_instruction(&instruction, &accounts);
    }
}
