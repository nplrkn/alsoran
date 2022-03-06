extern crate asn1;
use asn1::aper::{self, encode_int, APerElement, Constraint, Constraints, Encoding, UNCONSTRAINED};
use asn1::{BitString, ExtensionMarker};

struct AmfUeNgapId;
struct RanUeNgapId;
struct RanPagingPriority;
struct NasPdu;
struct PduSessionResourceSetupListSuReq;
struct UeAggregateMaximumBitRate;
struct AmfcpRelocationIndication;
struct ProcedureCode;
struct Criticality;

pub struct PduSessionResourceSetupRequest {
    pub protocol_i_es: PduSessionResourceSetupRequestProtocolIEs,
}

pub enum NgapPdu {
    InitiatingMessage(InitiatingMessage),
}

pub struct InitiatingMessage {
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessageValue,
}

pub struct PduSessionResourceSetupRequestProtocolIEs(
    Vec<PduSessionResourceSetupRequestProtocolIEsItem>,
);

pub struct ProtocolIeId(pub u16);

pub struct PduSessionResourceSetupRequestProtocolIEsItem {
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupRequestProtocolIEsItemValue,
}

pub enum PduSessionResourceSetupRequestProtocolIEsItemValue {
    IdAmfUeNgapId(AmfUeNgapId),
    IdNasPdu(NasPdu),
    IdPduSessionResourceSetupListSuReq(PduSessionResourceSetupListSuReq),
    IdRanUeNgapId(RanUeNgapId),
    IdRanPagingPriority(RanPagingPriority),
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
}

pub enum InitiatingMessageValue {
    IdAmfcpRelocationIndication(AmfcpRelocationIndication),
    IdPduSessionResourceSetup(PduSessionResourceSetupRequest),
}

// this can be code generated underneath PduSessionResourceSetupRequestProtocolIEsItem
// or a derive on it if we ahve the protocol id and criticality
impl From<AmfUeNgapId> for PduSessionResourceSetupRequestProtocolIEsItem {
    fn from(value: AmfUeNgapId) -> Self {
        PduSessionResourceSetupRequestProtocolIEsItem {
            id: ProtocolIeId(3),
            criticality: Criticality,
            value: PduSessionResourceSetupRequestProtocolIEsItemValue::IdAmfUeNgapId(value),
        }
    }
}
impl From<RanUeNgapId> for PduSessionResourceSetupRequestProtocolIEsItem {
    fn from(value: RanUeNgapId) -> Self {
        PduSessionResourceSetupRequestProtocolIEsItem {
            id: ProtocolIeId(3),
            criticality: Criticality,
            value: PduSessionResourceSetupRequestProtocolIEsItemValue::IdAmfUeNgapId(value),
        }
    }
}
impl From<RanPagingPriority> for PduSessionResourceSetupRequestProtocolIEsItem {
    fn from(value: RanPagingPriority) -> Self {
        PduSessionResourceSetupRequestProtocolIEsItem {
            id: ProtocolIeId(3),
            criticality: Criticality,
            value: PduSessionResourceSetupRequestProtocolIEsItemValue::IdAmfUeNgapId(value),
        }
    }
}

struct PduSessionResourceSetupRequest2 {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_ue_ngap_id: RanUeNgapId,
    pub ran_paging_priority: Option<RanPagingPriority>,
    pub nas_pdu: Option<NasPdu>,
    pub pdu_session_resource_setup_list_su_req: PduSessionResourceSetupListSuReq,
    pub ue_aggregate_maximum_bit_rate: Option<UeAggregateMaximumBitRate>,
}

// this could be generated or derived if we have the procedure code and criticality
impl From<PduSessionResourceSetupRequest2> for NgapPdu {
    fn from(x: PduSessionResourceSetupRequest2) -> Self {
        let mut v = Vec::new();
        v.push(x.amf_ue_ngap_id.into());
        v.push(x.ran_ue_ngap_id.into());
        if let Some(x) = x.ran_paging_priority {
            v.push(x.into())
        };

        NgapPdu::InitiatingMessage(InitiatingMessage {
            procedure_code: ProcedureCode,
            criticality: Criticality,
            value: InitiatingMessageValue::IdPduSessionResourceSetup(
                PduSessionResourceSetupRequest {
                    protocol_i_es: PduSessionResourceSetupRequestProtocolIEs(v),
                },
            ),
        })
    }
}

impl APerElement for PduSessionResourceSetupRequest2 {
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, aper::EncodeError> {
        // CHOICE
        let mut enc = (false as ExtensionMarker).to_aper(UNCONSTRAINED).unwrap();
        enc.append(&encode_int(0, Some(0), Some(2)).unwrap()).unwrap();
        // ProcedureCode		::= INTEGER (0..255)
        // id-PDUSessionResourceSetup					ProcedureCode ::= 29
        enc.append(&encode_int(29, Some(0), Some(255)).unwrap()).unwrap();
        // Criticality		::= ENUMERATED { reject, ignore, notify }
        // reject
        enc.append(&encode_int(0, Some(0), Some(2)).unwrap()).unwrap();
        // Then the message which is a sequence of IEs.

        let mut v = Vec::new();
        v.push(x.amf_ue_ngap_id.into());
        v.push(x.ran_ue_ngap_id.into());
        if let Some(x) = x.ran_paging_priority {
            v.push(x.into())
        };


        let mut count = 0;
        let amf_ue_ngap_id = 
        let ran_ue_ngap_id = Some(self.amf_ue_ngap_id.to_aper()?);
        let ran_paging_priority = x.ran_paging_priority.map(|x| x.to_aper()?); 
        let items = vec!(amf_ue_ngap_id, ran_ue_ngap_id, ran_paging_priority)










        let mut enc = self.bar.to_aper(Constraints {
            value: None,
            size: Some(Constraint::new(None, Some(4))),
        }).unwrap();

        enc.append(&self.baz.to_aper(UNCONSTRAINED).unwrap());

        Ok(enc)
    }
    pub fn encode 
}
