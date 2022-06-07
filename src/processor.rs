use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    borsh::try_from_slice_unchecked
};
use crate::{instruction::IntroInstruction, state::MovieInfo};


pub struct Processor;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("unpacking instruction data");
        let instruction = IntroInstruction::unpack(instruction_data)?;
        let account_info_iter = &mut accounts.iter();

        match instruction {
            IntroInstruction::InitMovieRating {title, rating, description} => {
                msg!("fee account");
                let fee_payer = next_account_info(account_info_iter)?;
                msg!("movie account");
                let movie_review = next_account_info(account_info_iter)?;
                msg!("Deserialize movie account data");
                let account_data = try_from_slice_unchecked::<MovieInfo>(&movie_review.data.borrow()).unwrap();
                
                msg!("Title: {}", account_data.title);
                msg!("Rating: {}", account_data.rating);
                msg!("Description: {}", account_data.description);

                Ok(())
            }
        }
	}
}