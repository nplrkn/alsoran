use crate::pdu::*;
use crate::top_pdu::*;
use crate::ProcedureCode;
use asn1_codecs::aper::AperCodec;
use asn1_codecs::aper::AperCodecData;
use asn1_codecs::aper::AperCodecError;
use async_trait::async_trait;

// trait NgapProcedure {
//     type Request: AperCodec + IntoPdu<NgapPdu> + Send + Sync + 'static;
//     type Success: AperCodec<Output = Self::Success>;
// }

// These are xxap-common? (from net?)
trait Procedure<T: AperCodec> {
    type Request: AperCodec + IntoPdu<T> + Send + Sync + 'static;
    type Success: AperCodec<Output = Self::Success>;
}

trait IntoPdu<P> {
    fn to_pdu_and_procedure_code(self) -> (P, ProcedureCode);
}

#[async_trait]
trait RequestProvider<T: AperCodec, P: Procedure<T>> {
    async fn request(&self, r: P::Request) -> Result<P::Success, AperCodecError>;
}

// These stay here
//trait NgapProcedure: Procedure<NgapPdu> {}

// #[async_trait]
// trait RequestProvider<P: NgapProcedure> {
//     async fn request(&self, r: P::Request) -> Result<P::Success, AperCodecError>;
// }

// Autogen this
impl IntoPdu<NgapPdu> for NgSetupRequest {
    fn to_pdu_and_procedure_code(self) -> (NgapPdu, ProcedureCode) {
        todo!()
    }
}

// Autogen this
struct NgSetupRequestProcedure {}
impl Procedure<NgapPdu> for NgSetupRequestProcedure {
    type Request = NgSetupRequest;
    type Success = NgSetupResponse;
}

#[async_trait]
impl<T: AperCodec, P: Procedure<T>> RequestProvider<T, P> for Connection {
    async fn request(&self, r: P::Request) -> Result<P::Success, AperCodecError> {
        let (pdu, procedure_code) = r.to_pdu_and_procedure_code();
        let mut d = AperCodecData::new();
        pdu.encode(&mut d)?;
        let bytes = d.into_bytes();

        // we can implement this on top of existing transaction layer as well.

        // imagine we send down TNLA and register a catcher and get back whether it is successful or not.
        let successful = true;
        let open_type_bytes = bytes;

        let mut d = AperCodecData::from_slice(&open_type_bytes);
        if successful {
            P::Success::decode(&mut d)
        } else {
            P::Success::decode(&mut d) // unsuccessful response type
        }
    }
}

trait NgapRequestProvider<P: NgapProcedure>: RequestProvider<NgapPdu, P> {}
impl<P: NgapProcedure> NgapRequestProvider<P> for Connection {}
//impl NgapProcedure for NgSetupRequestProcedure {}

async fn foo(c: Connection) -> Result<(), AperCodecError> {
    let ng_setup: NgSetupRequest = {};
    let _rsp =
        <Connection as RequestProvider<NgapPdu, NgSetupRequestProcedure>>::request(&c, ng_setup)
            .await?;
    Ok(())
}
