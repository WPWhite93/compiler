use miden_prelude::{Felt, Word};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AccountId(Felt);

impl From<AccountId> for Felt {
    fn from(account_id: AccountId) -> Felt {
        account_id.0
    }
}

#[repr(transparent)]
pub struct CoreAsset {
    pub(crate) inner: Word,
}

impl CoreAsset {
    pub fn new(word: Word) -> Self {
        CoreAsset { inner: word }
    }

    pub fn as_word(&self) -> Word {
        self.inner
    }
}

#[repr(transparent)]
pub struct Recipient(pub(crate) Word);

#[repr(transparent)]
pub struct Tag(pub(crate) Felt);

#[repr(transparent)]
pub struct NoteId(pub(crate) Felt);

#[repr(transparent)]
pub struct NoteType(pub(crate) Felt);
