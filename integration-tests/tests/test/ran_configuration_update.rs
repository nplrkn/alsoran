use crate::TestContext;
use also_net::TransportProvider;
use anyhow::{anyhow, Result};
use common::ngap::*;
use slog::info;

pub async fn handle(test_context: &TestContext) -> Result<()> {
    let amf = &test_context.amf;
    let logger = &test_context.logger;

    // Catch RAN Configuration update from the GNB
    info!(logger, "Wait for RAN Configuration update from GNB");

    let pdu = amf.receive_ngap_pdu().await;
    if let NgapPdu::InitiatingMessage(InitiatingMessage {
        value: InitiatingMessageValue::IdRanConfigurationUpdate(_ran_configuration_update),
        ..
    }) = pdu
    {
        info!(logger, "Got RAN configuration update, send response");
        Ok(())
    } else {
        Err(anyhow!("Not a RAN configuration update"))
    }?;

    let response = NgapPdu::SuccessfulOutcome(SuccessfulOutcome {
        procedure_code: ProcedureCode(35),
        criticality: Criticality(Criticality::REJECT),
        value: SuccessfulOutcomeValue::IdRanConfigurationUpdate(
            RanConfigurationUpdateAcknowledge {
                protocol_i_es: RanConfigurationUpdateAcknowledgeProtocolIEs(vec![]),
            },
        ),
    });

    amf.sender.send_pdu(response, &logger).await?;

    Ok(())
}
