#![allow(clippy::too_many_arguments)]
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(BorshDeserialize, Debug)]
pub struct AddWorkflowStatePayload {
    pub status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
}

#[derive(BorshDeserialize, Debug)]
pub struct UpdateWorkflowStatePayload {
    pub archived: bool, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
    pub is_saved: bool, //1 true when job is in 'saved' status
    pub status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
}

#[derive(BorshDeserialize, Debug)]
pub struct UpdateWorkflowPaymentStatePayload {
    pub is_paid: bool, //1
    pub paid_amount: u64,//8
}

#[derive(Clone)]
pub enum WorkflowStateInstruction {
    AddWorkflowState {
        status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
    },
    UpdateWorkflowState {
        archived: bool, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
        is_saved: bool, //1 true when job is in 'saved' status
        status: String, //16 => 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
    },
    UpdateWorkflowPaymentState {
        is_paid: bool, //1
        paid_amount: u64,//8
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
                    status: payload.status
                }
            }
            1 => {
                let payload = UpdateWorkflowStatePayload::try_from_slice(rest).unwrap();
                Self::UpdateWorkflowState  { 
                    archived: payload.archived,
                    is_saved: payload.is_saved,
                    status: payload.status,
                }
            }
            2 => {
                let payload = UpdateWorkflowPaymentStatePayload::try_from_slice(rest).unwrap();
                Self::UpdateWorkflowPaymentState  { 
                    is_paid: payload.is_paid,
                    paid_amount: payload.paid_amount,
                }
            }
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }
}
