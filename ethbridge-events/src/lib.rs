#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_bridge_events::*;
use ::ethbridge_governance_events::*;
use ::ethers::abi::AbiDecode;
use ::ethers::contract::{AbiError, EthEvent};
#[doc = r"[`EventCodec`] as a static reference trait object."]
pub type DynEventCodec = &'static (dyn EventCodec + Sync);
#[doc = r"Codec to deserialize Ethereum events."]
pub trait EventCodec {
    #[doc = r"ABI signature of the Ethereum event."]
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str>;
    #[doc = r"The kind of event (Bridge or Governance)."]
    fn kind(&self) -> EventKind;
    #[doc = r"Decode an Ethereum event."]
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError>;
}
#[doc = "Event codec for [`TransferToErcFilter`]."]
pub static TRANSFER_TO_ERC_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<TransferToErcFilter>;
impl TryFrom<Events> for TransferToErcFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Bridge(BridgeEvents::TransferToErcFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<TransferToErcFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        TransferToErcFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = TransferToErcFilter::decode_log(log)?;
        Ok(Events::Bridge(BridgeEvents::TransferToErcFilter(event)))
    }
}
#[doc = "Event codec for [`TransferToNamadaFilter`]."]
pub static TRANSFER_TO_NAMADA_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<TransferToNamadaFilter>;
impl TryFrom<Events> for TransferToNamadaFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Bridge(BridgeEvents::TransferToNamadaFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<TransferToNamadaFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        TransferToNamadaFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = TransferToNamadaFilter::decode_log(log)?;
        Ok(Events::Bridge(BridgeEvents::TransferToNamadaFilter(event)))
    }
}
#[doc = "Event codec for [`NewContractFilter`]."]
pub static NEW_CONTRACT_CODEC: DynEventCodec = &::std::marker::PhantomData::<NewContractFilter>;
impl TryFrom<Events> for NewContractFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Governance(GovernanceEvents::NewContractFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<NewContractFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        NewContractFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = NewContractFilter::decode_log(log)?;
        Ok(Events::Governance(GovernanceEvents::NewContractFilter(
            event,
        )))
    }
}
#[doc = "Event codec for [`UpgradedContractFilter`]."]
pub static UPGRADED_CONTRACT_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<UpgradedContractFilter>;
impl TryFrom<Events> for UpgradedContractFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Governance(GovernanceEvents::UpgradedContractFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<UpgradedContractFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        UpgradedContractFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = UpgradedContractFilter::decode_log(log)?;
        Ok(Events::Governance(
            GovernanceEvents::UpgradedContractFilter(event),
        ))
    }
}
#[doc = "Event codec for [`ValidatorSetUpdateFilter`]."]
pub static VALIDATOR_SET_UPDATE_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<ValidatorSetUpdateFilter>;
impl TryFrom<Events> for ValidatorSetUpdateFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Governance(GovernanceEvents::ValidatorSetUpdateFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<ValidatorSetUpdateFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        ValidatorSetUpdateFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = ValidatorSetUpdateFilter::decode_log(log)?;
        Ok(Events::Governance(
            GovernanceEvents::ValidatorSetUpdateFilter(event),
        ))
    }
}
#[doc = r"Return all Ethereum event codecs."]
pub fn event_codecs() -> [DynEventCodec; 5usize] {
    [
        TRANSFER_TO_ERC_CODEC,
        TRANSFER_TO_NAMADA_CODEC,
        NEW_CONTRACT_CODEC,
        UPGRADED_CONTRACT_CODEC,
        VALIDATOR_SET_UPDATE_CODEC,
    ]
}
#[doc = r"The Ethereum events generated by `ethbridge-rs`."]
#[derive(Debug)]
pub enum Events {
    #[doc = r" Bridge events."]
    Bridge(BridgeEvents),
    #[doc = r" Governance events."]
    Governance(GovernanceEvents),
}
#[doc = r"The kinds of Ethereum events generated by `ethbridge-rs`."]
#[derive(Debug)]
pub enum EventKind {
    #[doc = r" Bridge events."]
    Bridge,
    #[doc = r" Governance events."]
    Governance,
}
