use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ApplicantInfoState {
    pub is_initialized: bool, //1
    pub owner_pubkey: Pubkey, //32
    pub created_at: u64, //8
    pub updated_at: u64, //8
    pub username: String, //32
    pub name: String, //32
    pub address: String, //256
    pub image_uri: String, //128
    pub bio: String, //512
    pub skills: Vec<String>, //64*10 //640+10+10 ~700, 
    pub designation: String, //64
    pub current_employment_status: String, //32
    pub can_join_in: String, //32
    pub user_type: String, //16 //recruiter, applicant
    pub is_company_profile_complete: bool, //1
    pub is_overview_complete: bool, //1
    pub is_projects_complete: bool, //1
    pub is_contact_info_complete: bool, //1
    pub is_education_complete: bool, //1
    pub is_work_experience_complete: bool, //1

}
impl Sealed for ApplicantInfoState {}
impl IsInitialized for ApplicantInfoState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl ApplicantInfoState {
    pub const LEN: usize = 1+32+8+8+32+32+256+128+512+660+64+32+32+16+1+1+1+1+1+1; //1819 ~1850
}

