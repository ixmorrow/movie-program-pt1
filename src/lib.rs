use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked,
    program_pack::{IsInitialized},
};
use std::convert::TryInto;
pub mod instruction;
pub mod state;
use instruction::MovieInstruction;
use state::MovieAccountState;
use borsh::BorshSerialize;

entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  let instruction = MovieInstruction::unpack(instruction_data)?;
  match instruction {
    MovieInstruction::AddMovieReview { title, rating, description } => {
      add_movie_review(program_id, accounts, title, rating, description)
    }
  }
}

pub fn add_movie_review(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  title: String,
  rating: u8,
  description: String
) -> ProgramResult {
msg!("Adding movie review...");
msg!("Title: {}", title);
msg!("Rating: {}", rating);
msg!("Description: {}", description);

// Get Account iterator
let account_info_iter = &mut accounts.iter();
  
// Get accounts
let initializer = next_account_info(account_info_iter)?;
let movie_review = next_account_info(account_info_iter)?;
let account_data = try_from_slice_unchecked::<MovieAccountState>(&movie_review.data.borrow()).unwrap();

msg!("Fetching movie review...");
msg!("Title: {}", account_data.title);
msg!("Rating: {}", account_data.rating);
msg!("Description: {}", account_data.description);



  Ok(())
}
