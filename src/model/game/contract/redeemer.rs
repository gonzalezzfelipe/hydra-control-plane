use pallas::{
    codec::utils::MaybeIndefArray,
    ledger::primitives::{BigInt, Constr, PlutusData},
};

pub struct Redeemer {
    new_state_index: u64,
    spend_action: SpendAction,
}
pub enum SpendAction {
    AddPlayer,
    EndGame,
    Collect,
}

impl Redeemer {
    pub fn new(new_state_index: u64, spend_action: SpendAction) -> Self {
        Self {
            new_state_index,
            spend_action,
        }
    }
}

impl From<Redeemer> for PlutusData {
    fn from(value: Redeemer) -> Self {
        PlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: MaybeIndefArray::Indef(vec![
                PlutusData::BigInt(BigInt::Int((value.new_state_index as i64).into())),
                value.spend_action.into(),
            ]),
        })
    }
}

impl From<SpendAction> for PlutusData {
    fn from(value: SpendAction) -> Self {
        PlutusData::Constr(Constr {
            tag: 121,
            any_constructor: match value {
                SpendAction::AddPlayer => Some(0),
                SpendAction::EndGame => Some(1),
                SpendAction::Collect => Some(2),
            },
            fields: MaybeIndefArray::Indef(vec![]),
        })
    }
}