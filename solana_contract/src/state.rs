use anchor_lang::prelude::*;

declare_id!("521j9JRbE1KNycZjvtYN43cL1MB3JaHs9Cg1SnrzzUjK");

#[account]
#[program_error]
pub struct ProgramState {
    pub code_duel_fee: u64,
    pub prize_pool: AccountInfo,
    pub questions: AccountInfo,
    pub rivals: AccountInfo,
    pub rivals_balances: AccountInfo,
}

impl ProgramState {
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        let mut data = account_info.data.as_mut_slice();
        let mut state = anchor_lang::deserialize::<Self>(&mut data)?;
        state.prize_pool = AccountInfo::deserialize(&mut data)?;
        state.questions = AccountInfo::deserialize(&mut data)?;
        state.rivals = AccountInfo::deserialize(&mut data)?;
        state.rivals_balances = AccountInfo::deserialize(&mut data)?;
        Ok(state)
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Question {
    pub id: u32,
    pub name: String,
    pub status: QuestionStatus,
    pub reward: u64,
    pub rival_id: u32,
    pub daredevil_id: u32,
    pub current_prize_pool: u64,
    pub entrance_fee: u64,
    pub deadline: Timestamp,
}

pub enum QuestionStatus {
    Open,
    OpenNotSolved,
    ClosedSolved,
    ClosedNotSolved,
}

impl IntoAccount for Question {
    fn into_account(self) -> Account {
        let mut account = Account::new_init(
            EMPTY_ACCOUNT_DATA_LEN,
            anchor_lang::solana_program::program_id(),
            &ProgramState::create_instruction_discriminant(),
        );
        account.data.as_mut_slice()[0..std::mem::size_of::<Self>()]
            .copy_from_slice(&anchor_lang::serialize(&self));
        account
    }
}

pub fn create_instruction_discriminant() -> [u8; 8] {
    *b"code_duel"
}

// You can add more data structures as needed, such as for "Daredevil" or "Rival" information.
