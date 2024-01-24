pub mod instruction;
pub mod state;

use borsh::BorshSerialize;
use crate::instruction::ReviewInstruction;
use crate::state::AccountState;
use crate::state::ReviewError;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ReviewInstruction::unpack(instruction_data)?;
    match instruction {
        ReviewInstruction::AddReview {
            title,
            rating,
            description,
            location,
        } => add_review(program_id, accounts, title, rating, description, location),
        ReviewInstruction::UpdateReview {
            title,
            rating,
            description,
            location,
        } => update_review(program_id, accounts, title, rating, description, location),
    }
}

pub fn add_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
    location: String,
) -> ProgramResult {
    msg!("Adding  review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    // Iterate through accounts to get correct ones
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Account checks
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }
    //-------- Account checks end

    // PDA account requirements
    let account_len: usize = 1000;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create PDA account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    // Modify the PDA account data
    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    msg!("checking if  account is already initialized");
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.location = location;
    account_data.is_initialized = true;

    // Serialize and store the account in into blockchain
    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

pub fn update_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _title: String,
    rating: u8,
    description: String,
    _location: String,
) -> ProgramResult {
    msg!("Updating  review...");

    // Iterate through accounts to get correct ones
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    // Account checks
    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }
    //-------- Account checks end

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();
    msg!("review title: {}", account_data.title);

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            account_data.title.as_bytes().as_ref(),
        ],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    msg!("checking if  account is initialized");
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }

    // Modify the description
    msg!("Review before update:");
    msg!("Title: {}", account_data.title);
    msg!("Rating: {}", account_data.rating);
    msg!("Description: {}", account_data.description);

    account_data.rating = rating;
    account_data.description = description;

    msg!("Review after update:");
    msg!("Title: {}", account_data.title);
    msg!("Rating: {}", account_data.rating);
    msg!("Description: {}", account_data.description);

    // Serialize and store the account in into blockchain
    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
