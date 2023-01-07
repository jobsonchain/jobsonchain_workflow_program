use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, PartialEq, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CompanyInfoState {
    pub is_initialized: bool, //1
    pub archived: bool, //1
    pub user_info_state_account_pubkey: Pubkey, //32
    pub username: String, //32
    pub name: String, //64
    pub logo_uri: String, //128
    pub domain: String, //64
    pub company_type: String, //8 "product, service, both"
    pub company_size: String, //8 "small, medium, large"
    pub company_stage: String, //32
    pub funding_amount: String, //32
    pub funding_currency: String, //8  
    pub image_uri: String, //128
    pub cover_image_uri: String, //128
    pub founded_in: String, //16
    pub employee_size: String, //32
    pub address: String, //512
    pub description: String,// 1024
    pub website: String, //128
    pub linkedin: String,//128 //"string - max 32 characters",
    pub twitter: String, //128 "string - max 32 characters",
    pub facebook: String, //128
    pub instagram: String, //128
    pub subscription_plan: String, //16 "paynuse, sixmonths, yearly, forever" //default is paynuse
    pub subscription_purchased_on: u64, //8 unix timestamp of the date on which the subscription was purchased
    pub subscription_valid_till: u64, //8 unix timestamp of the date till which the subscription is valid
    pub company_seq_number: String, //8
}
impl Sealed for CompanyInfoState {}
impl IsInitialized for CompanyInfoState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl CompanyInfoState {
    pub const LEN: usize = 1+1+32+32+64+128+64+8+8+32+32+8+128+128+16+32+512+1024+128+128+128+128+128+16+8+8+8; //2922 ~2950
}


    
