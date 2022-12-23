use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized},
    pubkey::Pubkey,
    system_instruction::create_account,
    sysvar::{rent::Rent}, borsh::try_from_slice_unchecked,
};
// use mpl_candy_machine::state::CandyMachine;
use borsh::{BorshSerialize};
use crate::{
    instruction::{CompanyInfoInstruction},
    state::{CompanyInfoState},
    
};
pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CompanyInfoInstruction::unpack(instruction_data)?;
        match instruction {
            CompanyInfoInstruction::SaveCompanyInfo{
                username,         //32
                name,             //64
                image_uri,        //128
                cover_image_uri,  //128
                founded_in,       //8
                empoliyee_size,   //8
                address,          //512
                description,      //1024
                website           //128
            } => {
                msg!("Instruction: Save Company Info");
                return Self::save_company_info(accounts, program_id, 
                    username,
                    name,             //64
                    image_uri,        //128
                    cover_image_uri,  //128
                    founded_in,       //8
                    empoliyee_size,   //8
                    address,          //512
                    description,      //1024
                    website           //128
                );
            } 
            CompanyInfoInstruction::UpdateCompanyInfo{
                username,         //32
                name,             //64
                image_uri,        //128
                cover_image_uri,  //128
                founded_in,       //8
                empoliyee_size,   //8
                address,          //512
                description,      //1024
                website           //128
            } => {
                msg!("Instruction: Update Company Info");
                return Self::update_company_info(accounts, program_id, 
                    username,
                    name,             //64
                    image_uri,        //128
                    cover_image_uri,  //128
                    founded_in,       //8
                    empoliyee_size,   //8
                    address,          //512
                    description,      //1024
                    website           //128
                );
            }
        }
    }

    
    pub fn save_company_info(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        username: String,         //32
        name: String,             //64
        image_uri: String,        //128
        cover_image_uri: String,  //128
        founded_in: String,       //8
        empoliyee_size: u64,      //8
        address: String,          //512
        description: String,      //1024
        website: String           //128

    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        msg!("Saving Applicant Info");
        let owner_account = next_account_info(account_info_iter)?;
        
        let company_info_state_account = next_account_info(account_info_iter)?;

        let system_program_id = next_account_info(account_info_iter)?;
        
        let company_info_pda_prefix = "jobsonchain_company_info";

        let company_info_pda_seed = &[
            company_info_pda_prefix.as_bytes(),
            owner_account.key.as_ref()
        ];

        let (company_info_pda, nonce) =
            Pubkey::find_program_address(company_info_pda_seed, program_id);

        if company_info_pda != *company_info_state_account.key{
            return Err(ProgramError::InvalidAccountData);
        }

        msg!("Creating Company Info State Account");

        invoke_signed(
            &create_account(
                owner_account.key,
                company_info_state_account.key,
                Rent::default().minimum_balance(CompanyInfoState::LEN),
                CompanyInfoState::LEN as u64,
                program_id,
            ),
            &[
                owner_account.clone(),              //payer of the account - owner
                company_info_state_account.clone(), //state account key pair of the program id created by owner
                system_program_id.clone(), // always prefer to send from outside which is use to create the account
            ],
            &[&[
                company_info_pda_prefix.as_bytes(),
                owner_account.key.as_ref(),
                    &[nonce],
                ]]
        )?;


        let mut company_info_state_data =
        try_from_slice_unchecked::<CompanyInfoState>(&company_info_state_account.data.borrow()).unwrap();

        if company_info_state_data.is_initialized() {
            msg!("Applicant info state account is already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        
         //Check if the game_category_state_account owner is program id
         if company_info_state_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        company_info_state_data.is_initialized = true;
        company_info_state_data.owner_pubkey = *owner_account.key;
        company_info_state_data.username = username;
        company_info_state_data.name = name;
        company_info_state_data.image_uri = image_uri;
        company_info_state_data.cover_image_uri = cover_image_uri;
        company_info_state_data.founded_in = founded_in;
        company_info_state_data.empoliyee_size = empoliyee_size;
        company_info_state_data.address = address;
        company_info_state_data.description = description;
        company_info_state_data.website = website;

        company_info_state_data.serialize(&mut &mut company_info_state_account.data.borrow_mut()[..])?;

        msg!("Current state of the Company Info State Account {:?}",company_info_state_account.data.borrow());

        Ok(())
    }

    pub fn update_company_info(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        username: String,         //32
        name: String,             //64
        image_uri: String,        //128
        cover_image_uri: String,  //128
        founded_in: String,       //8
        empoliyee_size: u64,      //8
        address: String,          //512
        description: String,      //1024
        website: String           //128
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        msg!("Updating Company Info");
        let owner_account = next_account_info(account_info_iter)?;
        
        let company_info_state_account = next_account_info(account_info_iter)?;

        let system_program_id = next_account_info(account_info_iter)?;
        
        let company_info_pda_prefix = "jobsonchain_company_info";

        let company_info_pda_seed = &[
            company_info_pda_prefix.as_bytes(),
            owner_account.key.as_ref()
        ];

        let (company_info_pda, nonce) =
            Pubkey::find_program_address(company_info_pda_seed, program_id);

        if company_info_pda != *company_info_state_account.key{
            return Err(ProgramError::InvalidAccountData);
        }

        let mut company_info_state_data =
        try_from_slice_unchecked::<CompanyInfoState>(&company_info_state_account.data.borrow()).unwrap();

        if !company_info_state_data.is_initialized() {
            msg!("Applicant info state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }
        
         if company_info_state_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if company_info_state_data.owner_pubkey != *owner_account.key {
            return Err(ProgramError::MissingRequiredSignature);
        }

        company_info_state_data.username = username;
        company_info_state_data.name = name;
        company_info_state_data.image_uri = image_uri;
        company_info_state_data.cover_image_uri = cover_image_uri;
        company_info_state_data.founded_in = founded_in;
        company_info_state_data.empoliyee_size = empoliyee_size;
        company_info_state_data.address = address;
        company_info_state_data.description = description;
        company_info_state_data.website = website;
        

        company_info_state_data.serialize(&mut &mut company_info_state_account.data.borrow_mut()[..])?;

        msg!("Current state of the Company Info State Account {:?}",company_info_state_account.data.borrow());

        Ok(())
    }

}
