use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WorkflowState {
    pub is_initialized: bool, //1
    pub archived: bool, //1 true when job is in 'accepted' or 'rejected' or 'withdraw' status
    pub company_owner_pubkey: Pubkey, //32
    pub company_pubkey: Pubkey, //32
    pub user_pubkey: Pubkey, //32
    pub job_pubkey: Pubkey, //32
    pub status: String, //16 => 'saved' or 'applied' or 'in_progress' or 'accepted' or 'rejected' or 'withdraw'
    pub job_applied_at: u64, //8 => timestamp in unix format
    pub is_paid: bool, //1
    pub paid_amount: u64,//8
    pub paid_at: u64, //8 => timestamp in unix format
    pub last_updated_at: u64, //8 => timestamp in unix format
}
impl Sealed for WorkflowState {}
impl IsInitialized for WorkflowState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl WorkflowState {
    pub const LEN: usize = 1+1+16+32+32+32+32+8+1+8+8+8; //179 ~200
}


    
