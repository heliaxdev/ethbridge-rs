#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_bridge_events::*;
use ::ethbridge_governance_events::*;
use ::ethers::abi::AbiType;
use ::ethers::abi::Tokenizable;
use ::ethers::contract::EthEvent;
#[doc = r"Codec to deserialize Ethereum events."]
pub trait EventCodec {
    #[doc = r"ABI signature of the Ethereum event."]
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str>;
    #[doc = r"The kind of event (Bridge or Governance)."]
    fn kind(&self) -> EventKind;
    #[doc = r"Decode an Ethereum event."]
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error>;
}
impl EventCodec for ::std::marker::PhantomData<TransferToErcFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        TransferToErcFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error> {
        let param = TransferToErcFilter::param_type();
        ::ethers::abi::decode(&[param], data).and_then(|mut toks| {
            let tok = toks.remove(0);
            let event = TransferToErcFilter::from_token(tok).map_err(|e| {
                ::ethers::abi::Error::Other(::std::borrow::Cow::Owned(e.to_string()))
            })?;
            Ok(Events::Bridge(BridgeEvents::TransferToErcFilter(event)))
        })
    }
}
impl EventCodec for ::std::marker::PhantomData<TransferToNamadaFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        TransferToNamadaFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error> {
        let param = TransferToNamadaFilter::param_type();
        ::ethers::abi::decode(&[param], data).and_then(|mut toks| {
            let tok = toks.remove(0);
            let event = TransferToNamadaFilter::from_token(tok).map_err(|e| {
                ::ethers::abi::Error::Other(::std::borrow::Cow::Owned(e.to_string()))
            })?;
            Ok(Events::Bridge(BridgeEvents::TransferToNamadaFilter(event)))
        })
    }
}
impl EventCodec for ::std::marker::PhantomData<NewContractFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        NewContractFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error> {
        let param = NewContractFilter::param_type();
        ::ethers::abi::decode(&[param], data).and_then(|mut toks| {
            let tok = toks.remove(0);
            let event = NewContractFilter::from_token(tok).map_err(|e| {
                ::ethers::abi::Error::Other(::std::borrow::Cow::Owned(e.to_string()))
            })?;
            Ok(Events::Governance(GovernanceEvents::NewContractFilter(
                event,
            )))
        })
    }
}
impl EventCodec for ::std::marker::PhantomData<UpdateBridgeWhitelistFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        UpdateBridgeWhitelistFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error> {
        let param = UpdateBridgeWhitelistFilter::param_type();
        ::ethers::abi::decode(&[param], data).and_then(|mut toks| {
            let tok = toks.remove(0);
            let event = UpdateBridgeWhitelistFilter::from_token(tok).map_err(|e| {
                ::ethers::abi::Error::Other(::std::borrow::Cow::Owned(e.to_string()))
            })?;
            Ok(Events::Governance(
                GovernanceEvents::UpdateBridgeWhitelistFilter(event),
            ))
        })
    }
}
impl EventCodec for ::std::marker::PhantomData<UpgradedContractFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        UpgradedContractFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error> {
        let param = UpgradedContractFilter::param_type();
        ::ethers::abi::decode(&[param], data).and_then(|mut toks| {
            let tok = toks.remove(0);
            let event = UpgradedContractFilter::from_token(tok).map_err(|e| {
                ::ethers::abi::Error::Other(::std::borrow::Cow::Owned(e.to_string()))
            })?;
            Ok(Events::Governance(
                GovernanceEvents::UpgradedContractFilter(event),
            ))
        })
    }
}
impl EventCodec for ::std::marker::PhantomData<ValidatorSetUpdateFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        ValidatorSetUpdateFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Governance
    }
    fn decode(&self, data: &[u8]) -> Result<Events, ::ethers::abi::Error> {
        let param = ValidatorSetUpdateFilter::param_type();
        ::ethers::abi::decode(&[param], data).and_then(|mut toks| {
            let tok = toks.remove(0);
            let event = ValidatorSetUpdateFilter::from_token(tok).map_err(|e| {
                ::ethers::abi::Error::Other(::std::borrow::Cow::Owned(e.to_string()))
            })?;
            Ok(Events::Governance(
                GovernanceEvents::ValidatorSetUpdateFilter(event),
            ))
        })
    }
}
#[doc = r"Return all Ethereum event codecs."]
pub fn event_codecs() -> &'static [&'static dyn EventCodec] {
    &[
        &::std::marker::PhantomData::<TransferToErcFilter>,
        &::std::marker::PhantomData::<TransferToNamadaFilter>,
        &::std::marker::PhantomData::<NewContractFilter>,
        &::std::marker::PhantomData::<UpdateBridgeWhitelistFilter>,
        &::std::marker::PhantomData::<UpgradedContractFilter>,
        &::std::marker::PhantomData::<ValidatorSetUpdateFilter>,
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
