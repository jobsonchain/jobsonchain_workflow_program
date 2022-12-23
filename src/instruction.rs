#![allow(clippy::too_many_arguments)]
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(BorshDeserialize, Debug)]
pub struct SaveCompanyInfoPayload {
    username: String, //32
    name: String, //64
    image_uri: String, //128
    cover_image_uri: String, //128
    founded_in: String, //8
    empoliyee_size: u64, //8
    address: String, //512
    description: String// 1024
    website: String //128
}

#[derive(BorshDeserialize, Debug)]
pub struct UpdateCompanyInfoPayload {
    username: String, //32
    name: String, //64
    image_uri: String, //128
    cover_image_uri: String, //128
    founded_in: String, //8
    empoliyee_size: u64, //8
    address: String, //512
    description: String// 1024
    website: String //128
}

#[derive(Clone)]
pub enum CompanyInfoInstruction {
    SaveCompanyInfo {
        username: String, //32
        name: String, //64
        image_uri: String, //128
        cover_image_uri: String, //128
        founded_in: String, //8
        empoliyee_size: u64, //8
        address: String, //512
        description: String// 1024
        website: String //128
    },
    UpdateCompanyInfo {
        username: String, //32
        name: String, //64
        image_uri: String, //128
        cover_image_uri: String, //128
        founded_in: String, //8
        empoliyee_size: u64, //8
        address: String, //512
        description: String// 1024
        website: String //128
    },
}

impl CompanyInfoInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidAccountData)?;
        msg!("Tag received -> {}", tag);

        Ok(match tag {
            0 => {
                let payload = SaveCompanyInfoPayload::try_from_slice(rest).unwrap();

                Self::SaveCompanyInfo {
                    username: payload.username,
                    name: payload.name,
                    image_uri: payload.image_uri,
                    cover_image_uri: payload.cover_image_uri,
                    founded_in: payload.founded_in,
                    empoliyee_size: payload.empoliyee_size,
                    address: payload.address,
                    description: payload.description,
                    website: payload.website,
                    
                }
            }
            1 => {
                let payload = UpdateCompanyInfoPayload::try_from_slice(rest).unwrap();
                Self::UpdateCompanyInfo { 
                    username: payload.username,
                    name: payload.name,
                    image_uri: payload.image_uri,
                    cover_image_uri: payload.cover_image_uri,
                    founded_in: payload.founded_in,
                    empoliyee_size: payload.empoliyee_size,
                    address: payload.address,
                    description: payload.description,
                    website: payload.website,
                }
            }
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }
}
