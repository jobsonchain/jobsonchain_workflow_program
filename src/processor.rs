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
    instruction::{WorkflowStateInstruction},
    state::{WorkflowState},
    company_info_state::{CompanyInfoState},
    applicant_info_state::ApplicantInfoState, contants::{WORKFLOW_STATE_ACCOUNT_PREFIX, APPLICANT_STATE_ACCOUNT_PREFIX, COMPANY_STATE_ACCOUNT_PREFIX, JOBPOST_STATE_ACCOUNT_PREFIX, SUBSCRIPTION_MODIFIER_PUBKEY}, jobpost_info_state::JobPostState
    
};
pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = WorkflowStateInstruction::unpack(instruction_data)?;
        match instruction {
            WorkflowStateInstruction::AddWorkflowState{
                status, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
                job_applied_at, //8 => timestamp in unix format
                last_updated_at
            } => {
                msg!("Instruction: Add Workflow State");
                return Self::add_workflow_state(accounts, program_id, 
                    status, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
                    job_applied_at, //8 => timestamp in unix format
                    last_updated_at
                );
            }
            WorkflowStateInstruction::UpdateWorkflowState{
                archived, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
                status, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
                last_updated_at
            } => {
                msg!("Instruction: Update Workflow State");
                return Self::update_workflow_state(accounts, program_id, 
                    archived, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
                    status, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
                    last_updated_at
                );
            }
            WorkflowStateInstruction::UpdateWorkflowPaymentState{
                is_paid, //1
                paid_amount,//8
                paid_at, //8 => timestamp in unix format
                last_updated_at
            } => {
                msg!("Instruction: Update Workflow Payment State");
                return Self::update_workflow_payment_state(accounts, program_id, 
                    is_paid, //1
                    paid_amount,//8
                    paid_at, //8 => timestamp in unix format
                    last_updated_at
                );
            }
        }
    }

    
    pub fn add_workflow_state(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
        job_applied_at: u64, //8 => timestamp in unix format
        last_updated_at: u64
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        msg!("Add Workflow State of Job");
        let owner_account = next_account_info(account_info_iter)?;
        let company_info_state_account = next_account_info(account_info_iter)?;
        let applicant_info_state_account = next_account_info(account_info_iter)?;
        let jobpost_info_state_account = next_account_info(account_info_iter)?;
        let workflow_info_state_account = next_account_info(account_info_iter)?;

        let user_info_program_id = next_account_info(account_info_iter)?;
        let company_info_program_id: &AccountInfo = next_account_info(account_info_iter)?;
        let jobpost_info_program_id: &AccountInfo = next_account_info(account_info_iter)?;
        let system_program_id = next_account_info(account_info_iter)?;
        
        //State: Verify Applicant state account
        let applicant_info_pda_prefix = APPLICANT_STATE_ACCOUNT_PREFIX;

        let applicant_pda_seed = &[
            applicant_info_pda_prefix.as_bytes(),
            owner_account.key.as_ref()
        ];
        let (applicant_info_pda, _nonce) =
            Pubkey::find_program_address(applicant_pda_seed, user_info_program_id.key);

        if applicant_info_pda != *applicant_info_state_account.key{
            msg!("Invalid Applicant Info State Account");
            return Err(ProgramError::InvalidAccountData);
        }

        //End: Verify Applicant state account

        //State: Verify Company Info state account
        let company_info_state_data = 
        try_from_slice_unchecked::<CompanyInfoState>(&company_info_state_account.data.borrow()).unwrap();


        let company_info_pda_prefix = COMPANY_STATE_ACCOUNT_PREFIX;

        let company_info_pda_seed = &[
            company_info_pda_prefix.as_bytes(),
            company_info_state_data.company_seq_number.as_bytes(),
            applicant_info_state_account.key.as_ref()
        ];

        let (company_info_pda, _nonce) =
            Pubkey::find_program_address(company_info_pda_seed, company_info_program_id.key);

        if company_info_pda != *company_info_state_account.key{
            msg!("Invalid Company Info State Account");
            return Err(ProgramError::InvalidAccountData);
        }

        if !company_info_state_data.is_initialized() {
            msg!("Company info state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        if company_info_state_data.user_info_state_account_pubkey != *applicant_info_state_account.key{
            msg!("company_info_state_account does not match the user_info_state_account_pubkey of the company_info_state_account");
            return Err(ProgramError::InvalidAccountData);
        }

        //End: Verify Company Info state account

        //State: Verify Jobpost Info state account
        let jobpost_info_state_data =
        try_from_slice_unchecked::<JobPostState>(&jobpost_info_state_account.data.borrow()).unwrap();
        
        let jobpost_info_pda_prefix = JOBPOST_STATE_ACCOUNT_PREFIX;

        let jobpost_info_pda = &[
            jobpost_info_pda_prefix.as_bytes(),
            jobpost_info_state_data.job_number.as_bytes(),
            company_info_state_account.key.as_ref(),
        ];

        let (jobpost_info_pda, _nonce) =
            Pubkey::find_program_address(jobpost_info_pda, jobpost_info_program_id.key);

        if jobpost_info_pda != *jobpost_info_state_account.key{
            msg!("JobPost Info State Account does not match the derived PDA");
            return Err(ProgramError::InvalidSeeds);
        }
        //Send: Verify Jobpost Info state account

        if owner_account.is_signer == false {
            msg!("Owner account is not a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let wokrflow_state_pda_prefix = WORKFLOW_STATE_ACCOUNT_PREFIX;

        let wokrflow_state_pda_seed = &[
            wokrflow_state_pda_prefix.as_bytes(),
            jobpost_info_state_account.key.as_ref(),
        ];

        let (wokrflow_state_pda, nonce) =
            Pubkey::find_program_address(wokrflow_state_pda_seed, program_id);

        if wokrflow_state_pda != *workflow_info_state_account.key{
            msg!("Invalid Workflow State PDA");
            return Err(ProgramError::InvalidSeeds);
        }

        let applicant_info_state_data = 
        try_from_slice_unchecked::<ApplicantInfoState>(&applicant_info_state_account.data.borrow()).unwrap();

        if !company_info_state_data.is_initialized() {
            msg!("Company info state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        //If any of the conditions are not met, the transaction will fail
        //if 1. condition is true that means applicant_info_state_account is not the correct owner of the company_info_state_account
        //if 2. condition is true that means owner_account is not the correct owner of the applicant_info_state_account
        if company_info_state_data.user_info_state_account_pubkey != *applicant_info_state_account.key && applicant_info_state_data.owner_pubkey != *owner_account.key{
            msg!("1. user_info_state_account_pubkey of the company_info_state_account does not match the applicant_info_state_account");
            msg!("2. owner_pubkey of the applicant_info_state_data does not match the owner_account of the transaction");
            return Err(ProgramError::InvalidAccountData);
        }


        msg!("Creating Workflow State Account");

        invoke_signed(
            &create_account(
                owner_account.key,
                workflow_info_state_account.key,
                Rent::default().minimum_balance(WorkflowState::LEN),
                WorkflowState::LEN as u64,
                program_id,
            ),
            &[
                owner_account.clone(),              //payer of the account - owner
                workflow_info_state_account.clone(), //account to be created
                company_info_state_account.clone(), //state account key pair of the program id created by owner
                system_program_id.clone(), // always prefer to send from outside which is use to create the account
            ],
            &[&[
                wokrflow_state_pda_prefix.as_bytes(),
                jobpost_info_state_account.key.as_ref(),
                &[nonce],
                ]]
        )?;

        msg!("Workflow State Account Created");

        let mut workflow_state_data =
        try_from_slice_unchecked::<WorkflowState>(&workflow_info_state_account.data.borrow()).unwrap();

        if workflow_state_data.is_initialized() {
            msg!("Workflow state account is already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        
         if workflow_info_state_account.owner != program_id {
            msg!("Workflow info state account is not owned by the program");
            return Err(ProgramError::IncorrectProgramId);
        }

        if owner_account.is_signer == false {
            msg!("Owner account is not a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        workflow_state_data.is_initialized = true;
        workflow_state_data.archived = false;
        workflow_state_data.status = status;
        workflow_state_data.company_owner_pubkey = owner_account.key.clone();
        workflow_state_data.company_pubkey = *company_info_state_account.key;
        workflow_state_data.user_pubkey = applicant_info_state_account.key.clone();
        workflow_state_data.job_pubkey = jobpost_info_state_account.key.clone();
        workflow_state_data.job_applied_at = job_applied_at;

        //check for subscription plan from the company info state account
        let mut subscription_status = false;
        let mut subscription_purchased_at = 0;
        msg!("Subscription Plan: {}", company_info_state_data.subscription_plan);
        if company_info_state_data.subscription_plan != "paynuse"{
            subscription_status = true;
        }

        if subscription_status {
            subscription_purchased_at = company_info_state_data.subscription_purchased_on
        }

        workflow_state_data.is_paid = subscription_status;
        workflow_state_data.paid_amount = 0;
        workflow_state_data.paid_at = subscription_purchased_at;
        workflow_state_data.last_updated_at = last_updated_at;
        workflow_state_data.serialize(&mut &mut workflow_info_state_account.data.borrow_mut()[..])?;

        msg!("Workflow State Account data added");

        Ok(())
    }

    pub fn update_workflow_state(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        archived: bool, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
        status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
        last_updated_at: u64, //8
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        msg!("Updating Company Info");
        let owner_account = next_account_info(account_info_iter)?;
        let company_info_state_account = next_account_info(account_info_iter)?;
        let applicant_info_state_account = next_account_info(account_info_iter)?;
        let jobpost_info_state_account = next_account_info(account_info_iter)?;
        let workflow_info_state_account = next_account_info(account_info_iter)?;

        let user_info_program_id = next_account_info(account_info_iter)?;
        let company_info_program_id: &AccountInfo = next_account_info(account_info_iter)?;
        let jobpost_info_program_id: &AccountInfo = next_account_info(account_info_iter)?;
        let _system_program_id = next_account_info(account_info_iter)?;
        
        //State: Verify Applicant state account
        let applicant_info_pda_prefix = APPLICANT_STATE_ACCOUNT_PREFIX;

        let applicant_pda_seed = &[
            applicant_info_pda_prefix.as_bytes(),
            owner_account.key.as_ref()
        ];
        let (applicant_info_pda, _nonce) =
            Pubkey::find_program_address(applicant_pda_seed, user_info_program_id.key);

        if applicant_info_pda != *applicant_info_state_account.key{
            msg!("Invalid Applicant Info State Account");
            return Err(ProgramError::InvalidAccountData);
        }

        //End: Verify Applicant state account

        //State: Verify Company Info state account
        let company_info_state_data = 
        try_from_slice_unchecked::<CompanyInfoState>(&company_info_state_account.data.borrow()).unwrap();


        let company_info_pda_prefix = COMPANY_STATE_ACCOUNT_PREFIX;

        let company_info_pda_seed = &[
            company_info_pda_prefix.as_bytes(),
            company_info_state_data.company_seq_number.as_bytes(),
            applicant_info_state_account.key.as_ref()
        ];

        let (company_info_pda, _nonce) =
            Pubkey::find_program_address(company_info_pda_seed, company_info_program_id.key);

        if company_info_pda != *company_info_state_account.key{
            msg!("Invalid Company Info State Account");
            return Err(ProgramError::InvalidAccountData);
        }

        if !company_info_state_data.is_initialized() {
            msg!("Company info state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        if company_info_state_data.user_info_state_account_pubkey != *applicant_info_state_account.key{
            msg!("company_info_state_account does not match the user_info_state_account_pubkey of the company_info_state_account");
            return Err(ProgramError::InvalidAccountData);
        }

        //End: Verify Company Info state account

        //State: Verify Jobpost Info state account
        let jobpost_info_state_data =
        try_from_slice_unchecked::<JobPostState>(&jobpost_info_state_account.data.borrow()).unwrap();
        
        let jobpost_info_pda_prefix = JOBPOST_STATE_ACCOUNT_PREFIX;

        let jobpost_info_pda = &[
            jobpost_info_pda_prefix.as_bytes(),
            jobpost_info_state_data.job_number.as_bytes(),
            company_info_state_account.key.as_ref(),
        ];

        let (jobpost_info_pda, _nonce) =
            Pubkey::find_program_address(jobpost_info_pda, jobpost_info_program_id.key);

        if jobpost_info_pda != *jobpost_info_state_account.key{
            msg!("JobPost Info State Account does not match the derived PDA");
            return Err(ProgramError::InvalidSeeds);
        }
        //Send: Verify Jobpost Info state account

        if owner_account.is_signer == false {
            msg!("Owner account is not a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let wokrflow_state_pda_prefix = WORKFLOW_STATE_ACCOUNT_PREFIX;

        let wokrflow_state_pda_seed = &[
            wokrflow_state_pda_prefix.as_bytes(),
            jobpost_info_state_account.key.as_ref(),
        ];

        let (wokrflow_state_pda, _nonce) =
            Pubkey::find_program_address(wokrflow_state_pda_seed, program_id);

        if wokrflow_state_pda != *workflow_info_state_account.key{
            msg!("Invalid Workflow State PDA");
            return Err(ProgramError::InvalidSeeds);
        }

        let mut workflow_state_data =
        try_from_slice_unchecked::<WorkflowState>(&workflow_info_state_account.data.borrow()).unwrap();

        if !workflow_state_data.is_initialized() {
            msg!("Workflow state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        if workflow_info_state_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if workflow_state_data.company_owner_pubkey != *owner_account.key && workflow_state_data.user_pubkey != *applicant_info_state_account.key {
            msg!("Workflow state account does not belong to the owner");
            return Err(ProgramError::InvalidAccountData);
        }

        if workflow_state_data.job_pubkey != *jobpost_info_state_account.key {
            msg!("Workflow state account does not belong to the jobpost_info_state_account");
            return Err(ProgramError::InvalidAccountData);
        }

        workflow_state_data.archived = archived;
        workflow_state_data.status = status;
        workflow_state_data.last_updated_at = last_updated_at;

        workflow_state_data.serialize(&mut &mut workflow_info_state_account.data.borrow_mut()[..])?;

        msg!("Workflow State Account data updated");

        Ok(())
    }

    pub fn update_workflow_payment_state(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        is_paid: bool,
        paid_amount: u64,
        paid_at: u64,
        last_updated_at: u64,
    ) -> ProgramResult{
        let account_info_iter = &mut accounts.iter();

        msg!("Updating Workflow Payment Info");
        let owner_account = next_account_info(account_info_iter)?;
        let logged_in_user_pubkey = next_account_info(account_info_iter)?;
        let company_info_state_account = next_account_info(account_info_iter)?;
        let applicant_info_state_account = next_account_info(account_info_iter)?;
        let jobpost_info_state_account = next_account_info(account_info_iter)?;
        let workflow_info_state_account = next_account_info(account_info_iter)?;

        let user_info_program_id = next_account_info(account_info_iter)?;
        let company_info_program_id: &AccountInfo = next_account_info(account_info_iter)?;
        let jobpost_info_program_id: &AccountInfo = next_account_info(account_info_iter)?;
        let _system_program_id = next_account_info(account_info_iter)?;
        
        //State: Verify Applicant state account
        let applicant_info_pda_prefix = APPLICANT_STATE_ACCOUNT_PREFIX;

        let applicant_pda_seed = &[
            applicant_info_pda_prefix.as_bytes(),
            logged_in_user_pubkey.key.as_ref()
        ];
        let (applicant_info_pda, _nonce) =
            Pubkey::find_program_address(applicant_pda_seed, user_info_program_id.key);

        if applicant_info_pda != *applicant_info_state_account.key{
            msg!("Invalid Applicant Info State Account");
            return Err(ProgramError::InvalidAccountData);
        }

        //End: Verify Applicant state account

        //State: Verify Company Info state account
        let company_info_state_data = 
        try_from_slice_unchecked::<CompanyInfoState>(&company_info_state_account.data.borrow()).unwrap();


        let company_info_pda_prefix = COMPANY_STATE_ACCOUNT_PREFIX;

        let company_info_pda_seed = &[
            company_info_pda_prefix.as_bytes(),
            company_info_state_data.company_seq_number.as_bytes(),
            applicant_info_state_account.key.as_ref()
        ];

        let (company_info_pda, _nonce) =
            Pubkey::find_program_address(company_info_pda_seed, company_info_program_id.key);

        if company_info_pda != *company_info_state_account.key{
            msg!("Invalid Company Info State Account");
            return Err(ProgramError::InvalidAccountData);
        }

        if !company_info_state_data.is_initialized() {
            msg!("Company info state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        if company_info_state_data.user_info_state_account_pubkey != *applicant_info_state_account.key{
            msg!("company_info_state_account does not match the user_info_state_account_pubkey of the company_info_state_account");
            return Err(ProgramError::InvalidAccountData);
        }

        //End: Verify Company Info state account

        //State: Verify Jobpost Info state account
        let jobpost_info_state_data =
        try_from_slice_unchecked::<JobPostState>(&jobpost_info_state_account.data.borrow()).unwrap();
        
        let jobpost_info_pda_prefix = JOBPOST_STATE_ACCOUNT_PREFIX;

        let jobpost_info_pda = &[
            jobpost_info_pda_prefix.as_bytes(),
            jobpost_info_state_data.job_number.as_bytes(),
            company_info_state_account.key.as_ref(),
        ];

        let (jobpost_info_pda, _nonce) =
            Pubkey::find_program_address(jobpost_info_pda, jobpost_info_program_id.key);

        if jobpost_info_pda != *jobpost_info_state_account.key{
            msg!("JobPost Info State Account does not match the derived PDA");
            return Err(ProgramError::InvalidSeeds);
        }
        //Send: Verify Jobpost Info state account

        if owner_account.is_signer == false {
            msg!("Owner account is not a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let wokrflow_state_pda_prefix = WORKFLOW_STATE_ACCOUNT_PREFIX;

        let wokrflow_state_pda_seed = &[
            wokrflow_state_pda_prefix.as_bytes(),
            jobpost_info_state_account.key.as_ref(),
        ];

        let (wokrflow_state_pda, _nonce) =
            Pubkey::find_program_address(wokrflow_state_pda_seed, program_id);

        if wokrflow_state_pda != *workflow_info_state_account.key{
            msg!("Invalid Workflow State PDA");
            return Err(ProgramError::InvalidSeeds);
        }

        let mut workflow_state_data =
        try_from_slice_unchecked::<WorkflowState>(&workflow_info_state_account.data.borrow()).unwrap();

        if !workflow_state_data.is_initialized() {
            msg!("Workflow state account is not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        if workflow_info_state_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if workflow_state_data.company_owner_pubkey != *logged_in_user_pubkey.key {
            msg!("Workflow state account does not belong to the owner");
            return Err(ProgramError::InvalidAccountData);
        }

        if workflow_state_data.job_pubkey != *jobpost_info_state_account.key {
            msg!("Workflow state account does not belong to the jobpost_info_state_account");
            return Err(ProgramError::InvalidAccountData);
        }

        if owner_account.key.to_string() != SUBSCRIPTION_MODIFIER_PUBKEY.to_string() {
            msg!("Unauthorized subscription modifier trying to update the subscription");
            return Err(ProgramError::InvalidAccountData);
        }

        workflow_state_data.is_paid = is_paid;
        workflow_state_data.paid_amount = paid_amount;
        workflow_state_data.paid_at = paid_at;
        workflow_state_data.last_updated_at = last_updated_at;        

        workflow_state_data.serialize(&mut &mut workflow_info_state_account.data.borrow_mut()[..])?;

        msg!("Workflow State Account payment staus updated");

        Ok(())
    }

}
