use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::CounterInstructions;

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter : u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter = counter_account.counter.saturating_add(args.value);
        }
        CounterInstructions::Decrement(args) => {
            counter_account.counter = counter_account.counter.saturating_sub(args.value);
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}



#[cfg(test)]
mod test {
	use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];
        let new_value = 33u32;
        let max_u32: u32 = u32::MAX;

        // Increment by 33
        let mut increment_instruction_data: Vec<u8> = vec![0];
        increment_instruction_data.extend_from_slice(&new_value.to_le_bytes());
        
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        // Increment by max u32 so that we check max boundary
        let mut increment_instruction_data: Vec<u8> = vec![0];
        increment_instruction_data.extend_from_slice(&max_u32.to_le_bytes());
        
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            max_u32
        );

        // Increment by 33 so that we still get max boundary
        let mut increment_instruction_data: Vec<u8> = vec![0];
        increment_instruction_data.extend_from_slice(&new_value.to_le_bytes());
        
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            max_u32
        );
        
        // Decrement by max u32 so that we get 0 again
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        decrement_instruction_data.extend_from_slice(&max_u32.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // Decrement by max 33 so that we check negative
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        decrement_instruction_data.extend_from_slice(&new_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // Update with 33 so that we get 33
        let mut update_instruction_data: Vec<u8> = vec![2];
        update_instruction_data.extend_from_slice(&new_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        // Reset to the zero so that we get zero again
        let reset_instruction_data: Vec<u8> = vec![3];
        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}