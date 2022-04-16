// aMFConfigurationUpdate NGAP-ELEMENTARY-PROCEDURE ::= {
// 	INITIATING MESSAGE		AMFConfigurationUpdate
// 	SUCCESSFUL OUTCOME		AMFConfigurationUpdateAcknowledge
// 	UNSUCCESSFUL OUTCOME	AMFConfigurationUpdateFailure
// 	PROCEDURE CODE			id-AMFConfigurationUpdate
// 	CRITICALITY				reject
// }

use asn1_codecs::aper::{AperCodec, AperCodecData};

use super::pdu::*;

enum Error<T, P> {
    TransportError(T),
    UnsuccessfulOutcome(P),
}

struct AllPossibleProviders {
    foo_request: FooRequestProvider,
    zho_indication: ZhoIndicationProvider,
}
trait Initiating {
    fn invoke_on_provider(x: AllPossibleProviders);
}

struct FooRequest;
struct ZhoIndication;

enum AllTheInitiating {
    FooRequest(FooRequest),
    ZhoIndication(ZhoIndication),
}

enum AllTheSuccessfulMessages {
    FooResponse,
    BarResponse,
    BazResponse,
}

enum NgapPdu {
    InitiatingMessage(AllTheInitiating),
    SuccessfulOutcome(AllTheSuccessfulMessages),
    UnsuccessfulOutcome,
}

fn decode(pdu: NgapPdu) {
    // match request_response

    match pdu {
        InitiatingMessage => {
            // spawn task
            // match on ID
            // From() the PDU and call the provider initiate function
            // what if this was a method on the PDU
            // get the response
            // encode + send
        }
    }
}

fn handle_ng_setup_request() {
    // Spawn a task and call a provider.
}

fn handle_ng_setup_response() {
    // Find an outstanding NG Setup request.
}

trait AmfConfigurationUpdateProvider {
    type TransportError;
    fn initiate_amf_configuration_update(
        pdu: AmfConfigurationUpdate,
    ) -> Result<
        AmfConfigurationUpdateAcknowledge,
        Error<Self::TransportError, AmfConfigurationUpdateFailure>,
    >;
}

// nGSetup NGAP-ELEMENTARY-PROCEDURE ::= {
// 	INITIATING MESSAGE		NGSetupRequest
// 	SUCCESSFUL OUTCOME		NGSetupResponse
// 	UNSUCCESSFUL OUTCOME	NGSetupFailure
// 	PROCEDURE CODE			id-NGSetup
// 	CRITICALITY				reject
// }

// trait NgSetupProvider {
//     type TransportError;
//     fn initiate_ng_setup(
//         pdu: NgSetupRequest,
//     ) -> Result<NgSetupResponse, Error<Self::TransportError, NgSetupResponse>>;
// }

//trait NgapAmf: NgSetupProvider {}
trait NgapRan: AmfConfigurationUpdateProvider {}

trait Procedure {
    type InitiatingMessage: AperCodec;
    type SuccessfulOutcome;
    type UnsuccessfulOutcome;
}

struct NgSetupProcedure;
impl Procedure for NgSetupProcedure {
    type InitiatingMessage = NgSetupRequest;
    type SuccessfulOutcome = NgSetupResponse;
    type UnsuccessfulOutcome = NgSetupFailure;
}

trait Provider<P: Procedure> {
    type TransportError;
    fn initiate(
        &self,
        pdu: P::InitiatingMessage,
    ) -> Result<P::SuccessfulOutcome, Error<Self::TransportError, P::UnsuccessfulOutcome>>;
}
trait NgSetupProvider: Provider<NgSetupProcedure> {}

// This would be for the SCTP transport.  I.e. it can encode anything.
struct Encoder;
impl<P: Procedure> Provider<P> for Encoder {
    type TransportError = u32;
    fn initiate(
        &self,
        pdu: P::InitiatingMessage,
    ) -> Result<P::SuccessfulOutcome, Error<Self::TransportError, P::UnsuccessfulOutcome>> {
        encode(pdu);
        Err(Error::TransportError(3))
    }
}

fn encode<P: AperCodec>(pdu: P) {
    let mut data = AperCodecData::new();
    pdu.encode(&mut data).unwrap();
}

use super::ies::*;
use bitvec::prelude::*;

pub fn do_something() {
    let encoder = Encoder;
    let plmn_identity = PlmnIdentity(vec![0x02, 0xf8, 0x39]);
    let ng_setup = NgSetupRequest {
        global_ran_node_id: GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
            plmn_identity: plmn_identity.clone(),
            gnb_id: GnbId::GnbId(bitvec![Msb0, u8; 0x00, 0x01, 0x02]),
        }),
        ran_node_name: Some(RanNodeName("free5GC".to_string())),
        supported_ta_list: SupportedTaList(vec![SupportedTaItem {
            tac: Tac(vec![0, 0, 1]),
            broadcast_plmn_list: BroadcastPlmnList(vec![BroadcastPlmnItem {
                plmn_identity: plmn_identity.clone(),
                tai_slice_support_list: SliceSupportList(vec![SliceSupportItem {
                    s_nssai: SNssai {
                        sst: Sst(vec![0x01]),
                        sd: Some(Sd(vec![1, 2, 3])),
                    },
                }]),
            }]),
        }]),
        default_paging_drx: PagingDrx::V128,
        ue_retention_information: None,
        nb_iot_default_paging_drx: None,
        extended_ran_node_name: None,
    };
    //let _result = NgSetupProvider::initiate(&encoder, ng_setup);

    //let _result2 = (encoder as Provider<NgSetupProcedure, TransportError = u32>).initiate(ng_setup);
}
