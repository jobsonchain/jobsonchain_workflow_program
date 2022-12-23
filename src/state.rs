use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CompanyInfoState {
    pub is_initialized: bool, //1
    pub owner_pubkey: Pubkey, //32
    pub username: String, //32
    pub name: String, //64
    pub image_uri: String, //128
    pub cover_image_uri: String, //128
    pub founded_in: String, //8
    pub empoliyee_size: u64, //8
    pub address: String, //512
    pub description: String// 1024
    pub website: String //128
}
impl Sealed for CompanyInfoState {}
impl IsInitialized for CompanyInfoState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl CompanyInfoState {
    pub const LEN: usize = 1+32+32+64+128+128+8+8+512+1024+128; //2065 ~2100
}


    
