#![allow(clippy::too_many_arguments)]
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(BorshDeserialize, Debug)]
pub struct AddWorkflowStatePayload {
    pub status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
    pub job_applied_at: u64, //8 => timestamp in unix format
    pub last_updated_at: u64, //8 => timestamp in unix format
}

#[derive(BorshDeserialize, Debug)]
pub struct UpdateWorkflowStatePayload {
    pub archived: bool, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
    pub status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
    pub last_updated_at: u64, //8 => timestamp in unix format
}

#[derive(BorshDeserialize, Debug)]
pub struct UpdateWorkflowPaymentStatePayload {
    pub is_paid: bool, //1
    pub paid_amount: u64,//8
    pub paid_at: u64, //8 => timestamp in unix format
    pub last_updated_at: u64, //8 => timestamp in unix format
}

#[derive(Clone)]
pub enum WorkflowStateInstruction {
    AddWorkflowState {
        status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
        job_applied_at: u64, //8 => timestamp in unix format
        last_updated_at: u64, //8 => timestamp in unix format
    },
    UpdateWorkflowState {
        archived: bool, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
        status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
        last_updated_at: u64, //8 => timestamp in unix format
    },
    UpdateWorkflowPaymentState {
        is_paid: bool, //1
        paid_amount: u64,//8
        paid_at: u64, //8 => timestamp in unix format
        last_updated_at: u64, //8 => timestamp in unix format
    },
}

impl WorkflowStateInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidAccountData)?;
        msg!("Tag received -> {}", tag);

        Ok(match tag {
            0 => {
                let payload = AddWorkflowStatePayload::try_from_slice(rest).unwrap();

                Self::AddWorkflowState {
                    status: payload.status,
                    job_applied_at: payload.job_applied_at,
                    last_updated_at: payload.last_updated_at,
                }
            }
            1 => {
                let payload = UpdateWorkflowStatePayload::try_from_slice(rest).unwrap();
                Self::UpdateWorkflowState  { 
                    archived: payload.archived,
                    status: payload.status,
                    last_updated_at: payload.last_updated_at,
                }
            }
            2 => {
                let payload = UpdateWorkflowPaymentStatePayload::try_from_slice(rest).unwrap();
                Self::UpdateWorkflowPaymentState  { 
                    is_paid: payload.is_paid,
                    paid_amount: payload.paid_amount,
                    paid_at: payload.paid_at,
                    last_updated_at: payload.last_updated_at,
                }
            }
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }
}
