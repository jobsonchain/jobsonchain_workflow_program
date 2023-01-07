use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct JobPostState {
    pub is_initialized: bool, //1
    pub archived: bool, //1
    pub owner_pubkey: Pubkey, //32
    pub company_pubkey: Pubkey, //32
    pub job_title: String, //128
    pub short_description: String,//256
    pub long_description: String,//1024
    pub category: Vec<String>,//32*4+10+10 //category is an array of job category like Frontend Developer
    pub job_type: String, //16 full-time, part-time, contract, internship",
    pub currency_type: String, //8 fiat, crypto
    pub currency: String, //8 USD, ETH, BTC, etc
    pub min_salary: u64, //8 u64
    pub max_salary: u64, //8 u64
    pub experience_in_months: u64,//8 u64
    pub skills: Vec<String>, //64*10+10+10 // ReactJs, NodeJs, etc
    pub qualification: String, //512
    pub job_location_type: String, //32
    pub country: String, //64
    pub city: String, //64
    pub job_number: String, //8
}
impl Sealed for JobPostState {}
impl IsInitialized for JobPostState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl JobPostState {
    pub const LEN: usize = 1+1+32+32+128+256+1024+148+16+8+8+8+8+8+660+512+32+64+64+8; //3018 ~3050
}


    
