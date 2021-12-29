use crate::TestContext;
use also_net::TransportProvider;
use bitvec::prelude::BitVec;
use common::ngap::*;
use slog::info;

pub async fn handle(test_context: &TestContext) {
    let amf = &test_context.amf;
    let logger = &test_context.logger;

    // Catch NG Setup from the GNB
    info!(logger, "Wait for NG Setup from GNB");

    let pdu = amf.receive_ngap_pdu().await;

    if let NgapPdu::InitiatingMessage(InitiatingMessage {
        value: InitiatingMessageValue::IdNgSetup(_ng_setup),
        ..
    }) = pdu
    {
        info!(logger, "Got NG Setup, send setup response");
    } else {
        panic!("Not an NG setup");
    }

    // TODO - due to an apparent bug in the decoder, this has a spurious 00 on the end.
    // TODO - deduplicate with worker test
    let ng_setup_response = NgapPdu::InitiatingMessage(InitiatingMessage {
        procedure_code: ProcedureCode(21),
        criticality: Criticality(Criticality::REJECT),
        value: InitiatingMessageValue::IdNgSetup(NgSetupRequest {
            protocol_i_es: NgSetupRequestProtocolIEs(vec![NgSetupRequestProtocolIEsItem {
                id: ProtocolIeId(27),
                criticality: Criticality(Criticality::REJECT),
                value: NgSetupRequestProtocolIEsItemValue::IdGlobalRanNodeId(
                    GlobalRanNodeId::GlobalGnbId(GlobalGnbId {
                        plmn_identity: PlmnIdentity(vec![2, 3, 2, 1, 5, 6]),
                        gnb_id: GnbId::GnbId(BitString26(BitVec::from_element(0x10))),
                        ie_extensions: None,
                    }),
                ),
            }]),
        }),
    });

    amf.sender
        .send_pdu(ng_setup_response, &logger)
        .await
        .expect("Failed mock send");
}
