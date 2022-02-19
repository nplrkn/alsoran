#![allow(dead_code, unreachable_patterns)]
use super::f1ap::*;
use asn1_codecs_derive::AperCodec;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use miniserde::{Deserialize, Serialize};

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BplmnIdInfoItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: BplmnIdInfoItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BplmnIdInfoItemIeExtensions(pub Vec<BplmnIdInfoItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BandwidthSrSchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastToBeCancelledItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BroadcastToBeCancelledItemIeExtensions(
    pub Vec<BroadcastToBeCancelledItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BroadcastToBeCancelledListEntryValue {
    #[asn(key = 149)]
    IdBroadcastToBeCancelledItem(BroadcastToBeCancelledItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastToBeCancelledListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BroadcastToBeCancelledListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastPniNpnIdListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BroadcastPniNpnIdListItemIeExtensions(
    pub Vec<BroadcastPniNpnIdListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastSnpnIdListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BroadcastSnpnIdListItemIeExtensions(pub Vec<BroadcastSnpnIdListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "48", sz_ub = "48")]
pub struct BitString23(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CnuePagingIdentitychoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CpTransportLayerAddresschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CudurimInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CudurimInformationIeExtensions(pub Vec<CudurimInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CuduRadioInformationTransferProtocolIEsEntryValue {
    #[asn(key = 250)]
    IdCuduRadioInformationType(CuduRadioInformationType),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CuduRadioInformationTransferProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CuduRadioInformationTransferProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CuduRadioInformationTransferProtocolIEs(
    pub Vec<CuduRadioInformationTransferProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CuduRadioInformationTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CUtoDurrcInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 234)]
    IdCgConfig(CgConfig),
    #[asn(key = 173)]
    IdCellGroupConfig(CellGroupConfig),
    #[asn(key = 119)]
    IdHandoverPreparationInformation(HandoverPreparationInformation),
    #[asn(key = 163)]
    IdMeasurementTimingConfiguration(MeasurementTimingConfiguration),
    #[asn(key = 214)]
    IdUeAssistanceInformation(UeAssistanceInformation),
    #[asn(key = 339)]
    IdUeAssistanceInformationEutra(UeAssistanceInformationEutra),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CUtoDurrcInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: CUtoDurrcInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CUtoDurrcInformationIeExtensions(pub Vec<CUtoDurrcInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateSpCellItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidateSpCellItemIeExtensions(pub Vec<CandidateSpCellItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CandidateSpCellListEntryValue {
    #[asn(key = 91)]
    IdCandidateSpCellItem(CandidateSpCellItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateSpCellListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CandidateSpCellListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer24(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CapacityValueIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CapacityValueIeExtensions(pub Vec<CapacityValueIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CausechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellMeasurementResultItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellMeasurementResultItemIeExtensions(
    pub Vec<CellMeasurementResultItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellToReportItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellToReportItemIeExtensions(pub Vec<CellToReportItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceProtocolIEsEntryValue {
    #[asn(key = 379)]
    IdPrivacyIndicator(PrivacyIndicator),
    #[asn(key = 378)]
    IdTraceCollectionEntityIpAddress(TransportLayerAddress),
    #[asn(key = 380)]
    IdTraceCollectionEntityUri(UriAddress),
    #[asn(key = 243)]
    IdTraceId(TraceId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTrafficTraceProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellTrafficTraceProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CellTrafficTraceProtocolIEs(pub Vec<CellTrafficTraceProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTypeIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellTypeIeExtensions(pub Vec<CellTypeIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsBroadcastCancelledItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsBroadcastCancelledItemIeExtensions(
    pub Vec<CellsBroadcastCancelledItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsBroadcastCancelledListEntryValue {
    #[asn(key = 151)]
    IdCellsBroadcastCancelledItem(CellsBroadcastCancelledItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsBroadcastCancelledListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsBroadcastCancelledListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsBroadcastCompletedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsBroadcastCompletedItemIeExtensions(
    pub Vec<CellsBroadcastCompletedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsBroadcastCompletedListEntryValue {
    #[asn(key = 147)]
    IdCellsBroadcastCompletedItem(CellsBroadcastCompletedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsBroadcastCompletedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsBroadcastCompletedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsFailedToBeActivatedListEntryValue {
    #[asn(key = 2)]
    IdCellsFailedToBeActivatedListItem(CellsFailedToBeActivatedListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsFailedToBeActivatedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsFailedToBeActivatedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsFailedToBeActivatedListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsFailedToBeActivatedListItemIeExtensions(
    pub Vec<CellsFailedToBeActivatedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsStatusItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsStatusItemIeExtensions(pub Vec<CellsStatusItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsStatusListEntryValue {
    #[asn(key = 88)]
    IdCellsStatusItem(CellsStatusItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsStatusListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsStatusListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeBroadcastItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsToBeBroadcastItemIeExtensions(pub Vec<CellsToBeBroadcastItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsToBeBroadcastListEntryValue {
    #[asn(key = 145)]
    IdCellsToBeBroadcastItem(CellsToBeBroadcastItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeBroadcastListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsToBeBroadcastListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsToBeActivatedListEntryValue {
    #[asn(key = 4)]
    IdCellsToBeActivatedListItem(CellsToBeActivatedListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeActivatedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsToBeActivatedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsToBeActivatedListItemIeExtensionsEntryExtensionValue {
    #[asn(key = 179)]
    IdAvailablePlmnList(AvailablePlmnList),
    #[asn(key = 386)]
    IdAvailableSnpnIdList(AvailableSnpnIdList),
    #[asn(key = 197)]
    IdExtendedAvailablePlmnList(ExtendedAvailablePlmnList),
    #[asn(key = 291)]
    IdIabInfoIabDonorCu(IabInfoIabDonorCu),
    #[asn(key = 118)]
    IdGNbCuSystemInformation(GnbCuSystemInformation),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeActivatedListItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: CellsToBeActivatedListItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsToBeActivatedListItemIeExtensions(
    pub Vec<CellsToBeActivatedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsToBeBarredItemIeExtensionsEntryExtensionValue {
    #[asn(key = 298)]
    IdIabBarred(IabBarred),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeBarredItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: CellsToBeBarredItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsToBeBarredItemIeExtensions(pub Vec<CellsToBeBarredItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsToBeBarredListEntryValue {
    #[asn(key = 130)]
    IdCellsToBeBarredItem(CellsToBeBarredItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeBarredListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsToBeBarredListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellsToBeDeactivatedListEntryValue {
    #[asn(key = 6)]
    IdCellsToBeDeactivatedListItem(CellsToBeDeactivatedListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeDeactivatedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellsToBeDeactivatedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellsToBeDeactivatedListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellsToBeDeactivatedListItemIeExtensions(
    pub Vec<CellsToBeDeactivatedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString25(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString26(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString27(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString28(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ChildNodeCellsListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ChildNodeCellsListItemIeExtensions(pub Vec<ChildNodeCellsListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ChildNodesListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ChildNodesListItemIeExtensions(pub Vec<ChildNodesListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompositeAvailableCapacityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompositeAvailableCapacityIeExtensions(
    pub Vec<CompositeAvailableCapacityIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompositeAvailableCapacityGroupIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompositeAvailableCapacityGroupIeExtensions(
    pub Vec<CompositeAvailableCapacityGroupIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConditionalInterDuMobilityInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 433)]
    IdEstimatedArrivalProbability(ChoProbability),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConditionalInterDuMobilityInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ConditionalInterDuMobilityInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ConditionalInterDuMobilityInformationIeExtensions(
    pub Vec<ConditionalInterDuMobilityInformationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConditionalIntraDuMobilityInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 433)]
    IdEstimatedArrivalProbability(ChoProbability),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConditionalIntraDuMobilityInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ConditionalIntraDuMobilityInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ConditionalIntraDuMobilityInformationIeExtensions(
    pub Vec<ConditionalIntraDuMobilityInformationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnosticsIeExtensions(pub Vec<CriticalityDiagnosticsIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIeItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnosticsIeItemIeExtensions(
    pub Vec<CriticalityDiagnosticsIeItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer29(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlPrsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DlPrsIeExtensions(pub Vec<DlPrsIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct BitString30(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct BitString31(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct BitString32(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct BitString33(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BitString34(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct BitString35(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlPrsMutingPatternchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlPrsResourceArpLocationchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlPrsResourceSetArpLocationchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DlUpTnlAddressToUpdateListEntryValue {
    #[asn(key = 305)]
    IdDlUpTnlAddressToUpdateListItem(DlUpTnlAddressToUpdateListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlUpTnlAddressToUpdateListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DlUpTnlAddressToUpdateListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlUpTnlAddressToUpdateListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DlUpTnlAddressToUpdateListItemIeExtensions(
    pub Vec<DlUpTnlAddressToUpdateListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlprsResourceArpIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DlprsResourceArpIeExtensions(pub Vec<DlprsResourceArpIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct DlprsResourceCoordinatesListofDlPrsResourceSetArp(pub Vec<DlprsResourceSetArp>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlprsResourceCoordinatesIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DlprsResourceCoordinatesIeExtensions(pub Vec<DlprsResourceCoordinatesIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DlprsResourceSetArpListofDlPrsResourceArp(pub Vec<DlprsResourceArp>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlprsResourceSetArpIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DlprsResourceSetArpIeExtensions(pub Vec<DlprsResourceSetArpIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DlrrcMessageTransferProtocolIEsEntryValue {
    #[asn(key = 248)]
    IdAdditionalRrmPriorityIndex(AdditionalRrmPriorityIndex),
    #[asn(key = 109)]
    IdExecuteDuplication(ExecuteDuplication),
    #[asn(key = 221)]
    IdPlmnAssistanceInfoForNetShar(PlmnIdentity),
    #[asn(key = 108)]
    IdRatFrequencyPriorityInformation(RatFrequencyPriorityInformation),
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 184)]
    IdRrcDeliveryStatusRequest(RrcDeliveryStatusRequest),
    #[asn(key = 64)]
    IdSrbid(Srbid),
    #[asn(key = 222)]
    IdUeContextNotRetrievable(UeContextNotRetrievable),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 217)]
    IdNewGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 47)]
    IdOldgNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlrrcMessageTransferProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DlrrcMessageTransferProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DlrrcMessageTransferProtocolIEs(pub Vec<DlrrcMessageTransferProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DluptnlInformationToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DluptnlInformationToBeSetupItemIeExtensions(
    pub Vec<DluptnlInformationToBeSetupItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbActivityItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrbActivityItemIeExtensions(pub Vec<DrbActivityItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrbActivityListEntryValue {
    #[asn(key = 99)]
    IdDrbActivityItem(DrbActivityItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbActivityListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrbActivityListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrbInformationIeExtensions(pub Vec<DrbInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrbNotifyItemIeExtensionsEntryExtensionValue {
    #[asn(key = 344)]
    IdCurrentQoSParaSetIndex(QoSParaSetNotifyIndex),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbNotifyItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrbNotifyItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrbNotifyItemIeExtensions(pub Vec<DrbNotifyItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrbNotifyListEntryValue {
    #[asn(key = 136)]
    IdDrbNotifyItem(DrbNotifyItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbNotifyListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrbNotifyListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsFailedToBeModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsFailedToBeModifiedItemIeExtensions(
    pub Vec<DrBsFailedToBeModifiedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsFailedToBeModifiedListEntryValue {
    #[asn(key = 12)]
    IdDrBsFailedToBeModifiedItem(DrBsFailedToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsFailedToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsFailedToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsFailedToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsFailedToBeSetupItemIeExtensions(pub Vec<DrBsFailedToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsFailedToBeSetupListEntryValue {
    #[asn(key = 14)]
    IdDrBsFailedToBeSetupItem(DrBsFailedToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsFailedToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsFailedToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsFailedToBeSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsFailedToBeSetupModItemIeExtensions(
    pub Vec<DrBsFailedToBeSetupModItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsFailedToBeSetupModListEntryValue {
    #[asn(key = 16)]
    IdDrBsFailedToBeSetupModItem(DrBsFailedToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsFailedToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsFailedToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsModifiedItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 344)]
    IdCurrentQoSParaSetIndex(QoSParaSetIndex),
    #[asn(key = 160)]
    IdRlcStatus(RlcStatus),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsModifiedItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsModifiedItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsModifiedItemIeExtensions(pub Vec<DrBsModifiedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsModifiedListEntryValue {
    #[asn(key = 20)]
    IdDrBsModifiedItem(DrBsModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsModifiedConfItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsModifiedConfItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsModifiedConfItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsModifiedConfItemIeExtensions(pub Vec<DrBsModifiedConfItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsModifiedConfListEntryValue {
    #[asn(key = 18)]
    IdDrBsModifiedConfItem(DrBsModifiedConfItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsModifiedConfListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsModifiedConfListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsRequiredToBeModifiedItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 160)]
    IdRlcStatus(RlcStatus),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsRequiredToBeModifiedItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsRequiredToBeModifiedItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsRequiredToBeModifiedItemIeExtensions(
    pub Vec<DrBsRequiredToBeModifiedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsRequiredToBeModifiedListEntryValue {
    #[asn(key = 22)]
    IdDrBsRequiredToBeModifiedItem(DrBsRequiredToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsRequiredToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsRequiredToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsRequiredToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsRequiredToBeReleasedItemIeExtensions(
    pub Vec<DrBsRequiredToBeReleasedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsRequiredToBeReleasedListEntryValue {
    #[asn(key = 24)]
    IdDrBsRequiredToBeReleasedItem(DrBsRequiredToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsRequiredToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsRequiredToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsSetupItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 344)]
    IdCurrentQoSParaSetIndex(QoSParaSetIndex),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSetupItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsSetupItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsSetupItemIeExtensions(pub Vec<DrBsSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsSetupListEntryValue {
    #[asn(key = 26)]
    IdDrBsSetupItem(DrBsSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsSetupModItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 344)]
    IdCurrentQoSParaSetIndex(QoSParaSetIndex),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSetupModItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsSetupModItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsSetupModItemIeExtensions(pub Vec<DrBsSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsSetupModListEntryValue {
    #[asn(key = 28)]
    IdDrBsSetupModItem(DrBsSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeModifiedItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 186)]
    IdBearerTypeChange(BearerTypeChange),
    #[asn(key = 177)]
    IdDcBasedDuplicationActivation(DuplicationActivation),
    #[asn(key = 176)]
    IdDcBasedDuplicationConfigured(DcBasedDuplicationConfigured),
    #[asn(key = 161)]
    IdDlpdcpsnLength(PdcpsnLength),
    #[asn(key = 188)]
    IdDuplicationActivation(DuplicationActivation),
    #[asn(key = 371)]
    IdRlcDuplicationInformation(RlcDuplicationInformation),
    #[asn(key = 187)]
    IdRlcMode(RlcMode),
    #[asn(key = 430)]
    IdTransmissionStopIndicator(TransmissionStopIndicator),
    #[asn(key = 192)]
    IdUlpdcpsnLength(PdcpsnLength),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeModifiedItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsToBeModifiedItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsToBeModifiedItemIeExtensions(pub Vec<DrBsToBeModifiedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeModifiedListEntryValue {
    #[asn(key = 30)]
    IdDrBsToBeModifiedItem(DrBsToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsToBeReleasedItemIeExtensions(pub Vec<DrBsToBeReleasedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeReleasedListEntryValue {
    #[asn(key = 32)]
    IdDrBsToBeReleasedItem(DrBsToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeSetupItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 177)]
    IdDcBasedDuplicationActivation(DuplicationActivation),
    #[asn(key = 176)]
    IdDcBasedDuplicationConfigured(DcBasedDuplicationConfigured),
    #[asn(key = 161)]
    IdDlpdcpsnLength(PdcpsnLength),
    #[asn(key = 371)]
    IdRlcDuplicationInformation(RlcDuplicationInformation),
    #[asn(key = 192)]
    IdUlpdcpsnLength(PdcpsnLength),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeSetupItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsToBeSetupItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsToBeSetupItemIeExtensions(pub Vec<DrBsToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeSetupListEntryValue {
    #[asn(key = 34)]
    IdDrBsToBeSetupItem(DrBsToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeSetupModItemIeExtensionsEntryExtensionValue {
    #[asn(key = 370)]
    IdAdditionalPdcpDuplicationTnlList(AdditionalPdcpDuplicationTnlList),
    #[asn(key = 177)]
    IdDcBasedDuplicationActivation(DuplicationActivation),
    #[asn(key = 176)]
    IdDcBasedDuplicationConfigured(DcBasedDuplicationConfigured),
    #[asn(key = 161)]
    IdDlpdcpsnLength(PdcpsnLength),
    #[asn(key = 371)]
    IdRlcDuplicationInformation(RlcDuplicationInformation),
    #[asn(key = 192)]
    IdUlpdcpsnLength(PdcpsnLength),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeSetupModItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsToBeSetupModItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrBsToBeSetupModItemIeExtensions(pub Vec<DrBsToBeSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToBeSetupModListEntryValue {
    #[asn(key = 36)]
    IdDrBsToBeSetupModItem(DrBsToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DrBsToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrxCycleIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DrxCycleIeExtensions(pub Vec<DrxCycleIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DucurimInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DucurimInformationIeExtensions(pub Vec<DucurimInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DucuRadioInformationTransferProtocolIEsEntryValue {
    #[asn(key = 249)]
    IdDucuRadioInformationType(DucuRadioInformationType),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DucuRadioInformationTransferProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DucuRadioInformationTransferProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DucuRadioInformationTransferProtocolIEs(
    pub Vec<DucuRadioInformationTransferProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DucuRadioInformationTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DufSlotConfigItemchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString36(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DUtoCurrcInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 212)]
    IdDrxConfig(DrxConfig),
    #[asn(key = 191)]
    IdDrxLongCycleStartOffset(DrxLongCycleStartOffset),
    #[asn(key = 238)]
    IdMeasGapSharingConfig(MeasGapSharingConfig),
    #[asn(key = 235)]
    IdPdcchBlindDetectionScg(PdcchBlindDetectionScg),
    #[asn(key = 237)]
    IdPhInfoMcg(PhInfoMcg),
    #[asn(key = 208)]
    IdPhInfoScg(PhInfoScg),
    #[asn(key = 236)]
    IdRequestedPdcchBlindDetectionScg(RequestedPdcchBlindDetectionScg),
    #[asn(key = 209)]
    IdRequestedBandCombinationIndex(RequestedBandCombinationIndex),
    #[asn(key = 210)]
    IdRequestedFeatureSetEntryIndex(RequestedFeatureSetEntryIndex),
    #[asn(key = 211)]
    IdRequestedPMaxFr2(RequestedPMaxFr2),
    #[asn(key = 342)]
    IdSlConfigDedicatedEutraInfo(SlConfigDedicatedEutraInfo),
    #[asn(key = 341)]
    IdSlPhyMacRlcConfig(SlPhyMacRlcConfig),
    #[asn(key = 193)]
    IdSelectedBandCombinationIndex(SelectedBandCombinationIndex),
    #[asn(key = 194)]
    IdSelectedFeatureSetEntryIndex(SelectedFeatureSetEntryIndex),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DUtoCurrcInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DUtoCurrcInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DUtoCurrcInformationIeExtensions(pub Vec<DUtoCurrcInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceProtocolIEsEntryValue {
    #[asn(key = 243)]
    IdTraceId(TraceId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeactivateTraceProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DeactivateTraceProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DeactivateTraceProtocolIEs(pub Vec<DeactivateTraceProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DedicatedSiDeliveryNeededUeItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DedicatedSiDeliveryNeededUeItemIeExtensions(
    pub Vec<DedicatedSiDeliveryNeededUeItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DedicatedSiDeliveryNeededUeListEntryValue {
    #[asn(key = 190)]
    IdDedicatedSiDeliveryNeededUeItem(DedicatedSiDeliveryNeededUeItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DedicatedSiDeliveryNeededUeListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DedicatedSiDeliveryNeededUeListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "127")]
pub struct Integer37(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct Integer38(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Enumerated39(pub u8);
impl Enumerated39 {
    const DELAY_CRITICAL: u8 = 0u8;
    const NON_DELAY_CRITICAL: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Dynamic5QiDescriptorIeExtensionsEntryExtensionValue {
    #[asn(key = 362)]
    IdCnPacketDelayBudgetDownlink(ExtendedPacketDelayBudget),
    #[asn(key = 369)]
    IdCnPacketDelayBudgetUplink(ExtendedPacketDelayBudget),
    #[asn(key = 363)]
    IdExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Dynamic5QiDescriptorIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: Dynamic5QiDescriptorIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Dynamic5QiDescriptorIeExtensions(pub Vec<Dynamic5QiDescriptorIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated40(pub u8);
impl Enumerated40 {
    const GBR: u8 = 0u8;
    const NON_GBR: u8 = 1u8;
    const DELAY_CRITICAL_GRB: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8", extensible = true)]
pub struct Integer41(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DynamicPqiDescriptorIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DynamicPqiDescriptorIeExtensions(pub Vec<DynamicPqiDescriptorIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasuredResultsItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ECidMeasuredResultsItemIeExtensions(pub Vec<ECidMeasuredResultsItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasuredResultsValuechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementQuantitiesEntryValue {
    #[asn(key = 415)]
    IdECidMeasurementQuantitiesItem(ECidMeasurementQuantitiesItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementQuantitiesEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementQuantitiesEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementQuantitiesItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ECidMeasurementQuantitiesItemIeExtensions(
    pub Vec<ECidMeasurementQuantitiesItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementResultIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ECidMeasurementResultIeExtensions(pub Vec<ECidMeasurementResultIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementFailureIndicationProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 412)]
    IdLmfUeMeasurementId(LmfUeMeasurementId),
    #[asn(key = 413)]
    IdRanUeMeasurementId(RanUeMeasurementId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementFailureIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementFailureIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ECidMeasurementFailureIndicationProtocolIEs(
    pub Vec<ECidMeasurementFailureIndicationProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementInitiationFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 412)]
    IdLmfUeMeasurementId(LmfUeMeasurementId),
    #[asn(key = 413)]
    IdRanUeMeasurementId(RanUeMeasurementId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementInitiationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementInitiationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ECidMeasurementInitiationFailureProtocolIEs(
    pub Vec<ECidMeasurementInitiationFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementInitiationRequestProtocolIEsEntryValue {
    #[asn(key = 416)]
    IdECidMeasurementPeriodicity(MeasurementPeriodicity),
    #[asn(key = 414)]
    IdECidMeasurementQuantities(ECidMeasurementQuantities),
    #[asn(key = 424)]
    IdECidReportCharacteristics(ECidReportCharacteristics),
    #[asn(key = 412)]
    IdLmfUeMeasurementId(LmfUeMeasurementId),
    #[asn(key = 413)]
    IdRanUeMeasurementId(RanUeMeasurementId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementInitiationRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementInitiationRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ECidMeasurementInitiationRequestProtocolIEs(
    pub Vec<ECidMeasurementInitiationRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementInitiationResponseProtocolIEsEntryValue {
    #[asn(key = 418)]
    IdCellPortionId(CellPortionId),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 417)]
    IdECidMeasurementResult(ECidMeasurementResult),
    #[asn(key = 412)]
    IdLmfUeMeasurementId(LmfUeMeasurementId),
    #[asn(key = 413)]
    IdRanUeMeasurementId(RanUeMeasurementId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementInitiationResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementInitiationResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ECidMeasurementInitiationResponseProtocolIEs(
    pub Vec<ECidMeasurementInitiationResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementReportProtocolIEsEntryValue {
    #[asn(key = 418)]
    IdCellPortionId(CellPortionId),
    #[asn(key = 417)]
    IdECidMeasurementResult(ECidMeasurementResult),
    #[asn(key = 412)]
    IdLmfUeMeasurementId(LmfUeMeasurementId),
    #[asn(key = 413)]
    IdRanUeMeasurementId(RanUeMeasurementId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementReportProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementReportProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ECidMeasurementReportProtocolIEs(pub Vec<ECidMeasurementReportProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ECidMeasurementTerminationCommandProtocolIEsEntryValue {
    #[asn(key = 412)]
    IdLmfUeMeasurementId(LmfUeMeasurementId),
    #[asn(key = 413)]
    IdRanUeMeasurementId(RanUeMeasurementId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECidMeasurementTerminationCommandProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ECidMeasurementTerminationCommandProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ECidMeasurementTerminationCommandProtocolIEs(
    pub Vec<ECidMeasurementTerminationCommandProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraCoexFddInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraCoexFddInfoIeExtensions(pub Vec<EutraCoexFddInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraCoexTddInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraCoexTddInfoIeExtensions(pub Vec<EutraCoexTddInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraFddInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraFddInfoIeExtensions(pub Vec<EutraFddInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraModeInfochoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "837")]
pub struct Integer42(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct Integer43(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BOOLEAN")]
pub struct Boolean44(pub bool);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "94")]
pub struct Integer45(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer46(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraPrachConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraPrachConfigurationIeExtensions(pub Vec<EutraPrachConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraSpecialSubframeInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraSpecialSubframeInfoIeExtensions(pub Vec<EutraSpecialSubframeInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraTddInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraTddInfoIeExtensions(pub Vec<EutraTddInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraCellsListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutraCellsListItemIeExtensions(pub Vec<EutraCellsListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutranQoSIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EutranQoSIeExtensions(pub Vec<EutranQoSIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EgressBhrlcchItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EgressBhrlcchItemIeExtensions(pub Vec<EgressBhrlcchItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum EndpointIpAddressAndPortIeExtensionsEntryExtensionValue {
    #[asn(key = 230)]
    IdPortNumber(PortNumber),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EndpointIpAddressAndPortIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: EndpointIpAddressAndPortIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EndpointIpAddressAndPortIeExtensions(pub Vec<EndpointIpAddressAndPortIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ErrorIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ErrorIndicationProtocolIEs(pub Vec<ErrorIndicationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExplicitFormatIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExplicitFormatIeExtensions(pub Vec<ExplicitFormatIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedGnbCuNameIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedGnbCuNameIeExtensions(pub Vec<ExtendedGnbCuNameIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedGnbDuNameIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedGnbDuNameIeExtensions(pub Vec<ExtendedGnbDuNameIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedAvailablePlmnItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedAvailablePlmnItemIeExtensions(
    pub Vec<ExtendedAvailablePlmnItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ExtendedServedPlmNsItemIeExtensionsEntryExtensionValue {
    #[asn(key = 390)]
    IdExtendedTaiSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 384)]
    IdNpnSupportInfo(NpnSupportInfo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedServedPlmNsItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ExtendedServedPlmNsItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedServedPlmNsItemIeExtensions(pub Vec<ExtendedServedPlmNsItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1apPdUchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1cTransferPathIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct F1cTransferPathIeExtensions(pub Vec<F1cTransferPathIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum F1RemovalFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1RemovalFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: F1RemovalFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct F1RemovalFailureProtocolIEs(pub Vec<F1RemovalFailureProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum F1RemovalRequestProtocolIEsEntryValue {
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1RemovalRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: F1RemovalRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct F1RemovalRequestProtocolIEs(pub Vec<F1RemovalRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum F1RemovalResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1RemovalResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: F1RemovalResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct F1RemovalResponseProtocolIEs(pub Vec<F1RemovalResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum F1SetupFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 77)]
    IdTimeToWait(TimeToWait),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1SetupFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: F1SetupFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct F1SetupFailureProtocolIEs(pub Vec<F1SetupFailureProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum F1SetupRequestProtocolIEsEntryValue {
    #[asn(key = 281)]
    IdBapAddress(BapAddress),
    #[asn(key = 427)]
    IdExtendedGnbCuName(ExtendedGnbCuName),
    #[asn(key = 171)]
    IdGnbDuRrcVersion(RrcVersion),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 254)]
    IdTransportLayerAddressInfo(TransportLayerAddressInfo),
    #[asn(key = 42)]
    IdGNbDuId(GnbDuId),
    #[asn(key = 45)]
    IdGNbDuName(GnbDuName),
    #[asn(key = 44)]
    IdGNbDuServedCellsList(GnbDuServedCellsList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1SetupRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: F1SetupRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct F1SetupRequestProtocolIEs(pub Vec<F1SetupRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum F1SetupResponseProtocolIEsEntryValue {
    #[asn(key = 281)]
    IdBapAddress(BapAddress),
    #[asn(key = 3)]
    IdCellsToBeActivatedList(CellsToBeActivatedList),
    #[asn(key = 426)]
    IdExtendedGnbDuName(ExtendedGnbDuName),
    #[asn(key = 170)]
    IdGnbCuRrcVersion(RrcVersion),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 254)]
    IdTransportLayerAddressInfo(TransportLayerAddressInfo),
    #[asn(key = 287)]
    IdUlBhNonUpTrafficMapping(UlBhNonUpTrafficMapping),
    #[asn(key = 82)]
    IdGNbCuName(GnbCuName),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct F1SetupResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: F1SetupResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct F1SetupResponseProtocolIEs(pub Vec<F1SetupResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum FddInfoIeExtensionsEntryExtensionValue {
    #[asn(key = 389)]
    IdDlCarrierList(NrCarrierList),
    #[asn(key = 355)]
    IdUlCarrierList(NrCarrierList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FddInfoIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: FddInfoIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FddInfoIeExtensions(pub Vec<FddInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum FlowsMappedToDrbItemIeExtensionsEntryExtensionValue {
    #[asn(key = 183)]
    IdQoSFlowMappingIndication(QoSFlowMappingIndication),
    #[asn(key = 364)]
    IdTscTrafficCharacteristics(TscTrafficCharacteristics),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FlowsMappedToDrbItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: FlowsMappedToDrbItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FlowsMappedToDrbItemIeExtensions(pub Vec<FlowsMappedToDrbItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FlowsMappedToSldrbItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FlowsMappedToSldrbItemIeExtensions(pub Vec<FlowsMappedToSldrbItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1024", extensible = true)]
pub struct Integer47(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "32")]
pub struct FreqBandNrItemSupportedSulBandList(pub Vec<SupportedSulFreqBandItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FreqBandNrItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FreqBandNrItemIeExtensions(pub Vec<FreqBandNrItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FreqDomainLengthchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GbrQoSFlowInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 343)]
    IdAlternativeQoSParaSetList(AlternativeQoSParaSetList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GbrQoSFlowInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: GbrQoSFlowInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GbrQoSFlowInformationIeExtensions(pub Vec<GbrQoSFlowInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GbrQosInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GbrQosInformationIeExtensions(pub Vec<GbrQosInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationFailedToSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbCuTnlAssociationFailedToSetupItemIeExtensions(
    pub Vec<GnbCuTnlAssociationFailedToSetupItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuTnlAssociationFailedToSetupListEntryValue {
    #[asn(key = 135)]
    IdGnbCuTnlAssociationFailedToSetupItem(GnbCuTnlAssociationFailedToSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationFailedToSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbCuTnlAssociationFailedToSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbCuTnlAssociationSetupItemIeExtensions(
    pub Vec<GnbCuTnlAssociationSetupItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuTnlAssociationSetupListEntryValue {
    #[asn(key = 133)]
    IdGnbCuTnlAssociationSetupItem(GnbCuTnlAssociationSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbCuTnlAssociationSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationToAddItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbCuTnlAssociationToAddItemIeExtensions(
    pub Vec<GnbCuTnlAssociationToAddItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuTnlAssociationToAddListEntryValue {
    #[asn(key = 120)]
    IdGnbCuTnlAssociationToAddItem(GnbCuTnlAssociationToAddItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationToAddListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbCuTnlAssociationToAddListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuTnlAssociationToRemoveItemIeExtensionsEntryExtensionValue {
    #[asn(key = 229)]
    IdTnlAssociationTransportLayerAddressgNbdu(CpTransportLayerAddress),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationToRemoveItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: GnbCuTnlAssociationToRemoveItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbCuTnlAssociationToRemoveItemIeExtensions(
    pub Vec<GnbCuTnlAssociationToRemoveItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuTnlAssociationToRemoveListEntryValue {
    #[asn(key = 122)]
    IdGnbCuTnlAssociationToRemoveItem(GnbCuTnlAssociationToRemoveItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationToRemoveListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbCuTnlAssociationToRemoveListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationToUpdateItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbCuTnlAssociationToUpdateItemIeExtensions(
    pub Vec<GnbCuTnlAssociationToUpdateItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuTnlAssociationToUpdateListEntryValue {
    #[asn(key = 124)]
    IdGnbCuTnlAssociationToUpdateItem(GnbCuTnlAssociationToUpdateItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuTnlAssociationToUpdateListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbCuTnlAssociationToUpdateListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbCuSystemInformationSibtypetobeupdatedlist(pub Vec<SibtypetobeupdatedListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbCuSystemInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 239)]
    IdSystemInformationAreaId(SystemInformationAreaId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbCuSystemInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: GnbCuSystemInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbCuSystemInformationIeExtensions(pub Vec<GnbCuSystemInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbDuCellResourceConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbDuCellResourceConfigurationIeExtensions(
    pub Vec<GnbDuCellResourceConfigurationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbDuServedCellsItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbDuServedCellsItemIeExtensions(pub Vec<GnbDuServedCellsItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbDuServedCellsListEntryValue {
    #[asn(key = 43)]
    IdGnbDuServedCellsItem(GnbDuServedCellsItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbDuServedCellsListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbDuServedCellsListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbDuSystemInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 387)]
    IdSib10Message(Sib10Message),
    #[asn(key = 310)]
    IdSib12Message(Sib12Message),
    #[asn(key = 311)]
    IdSib13Message(Sib13Message),
    #[asn(key = 312)]
    IdSib14Message(Sib14Message),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbDuSystemInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: GnbDuSystemInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbDuSystemInformationIeExtensions(pub Vec<GnbDuSystemInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbDuTnlAssociationToRemoveItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbDuTnlAssociationToRemoveItemIeExtensions(
    pub Vec<GnbDuTnlAssociationToRemoveItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbDuTnlAssociationToRemoveListEntryValue {
    #[asn(key = 227)]
    IdGnbDuTnlAssociationToRemoveItem(GnbDuTnlAssociationToRemoveItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbDuTnlAssociationToRemoveListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbDuTnlAssociationToRemoveListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbRxTxTimeDiffIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GnbRxTxTimeDiffIeExtensions(pub Vec<GnbRxTxTimeDiffIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbcuConfigurationUpdateProtocolIEsEntryValue {
    #[asn(key = 281)]
    IdBapAddress(BapAddress),
    #[asn(key = 3)]
    IdCellsToBeActivatedList(CellsToBeActivatedList),
    #[asn(key = 129)]
    IdCellsToBeBarredList(CellsToBeBarredList),
    #[asn(key = 5)]
    IdCellsToBeDeactivatedList(CellsToBeDeactivatedList),
    #[asn(key = 121)]
    IdGnbCuTnlAssociationToAddList(GnbCuTnlAssociationToAddList),
    #[asn(key = 123)]
    IdGnbCuTnlAssociationToRemoveList(GnbCuTnlAssociationToRemoveList),
    #[asn(key = 125)]
    IdGnbCuTnlAssociationToUpdateList(GnbCuTnlAssociationToUpdateList),
    #[asn(key = 244)]
    IdNeighbourCellInformationList(NeighbourCellInformationList),
    #[asn(key = 105)]
    IdProtectedEutraResourcesList(ProtectedEutraResourcesList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 254)]
    IdTransportLayerAddressInfo(TransportLayerAddressInfo),
    #[asn(key = 287)]
    IdUlBhNonUpTrafficMapping(UlBhNonUpTrafficMapping),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbcuConfigurationUpdateProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbcuConfigurationUpdateProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbcuConfigurationUpdateProtocolIEs(pub Vec<GnbcuConfigurationUpdateProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbcuConfigurationUpdateAcknowledgeProtocolIEsEntryValue {
    #[asn(key = 1)]
    IdCellsFailedToBeActivatedList(CellsFailedToBeActivatedList),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 189)]
    IdDedicatedSiDeliveryNeededUeList(DedicatedSiDeliveryNeededUeList),
    #[asn(key = 134)]
    IdGnbCuTnlAssociationFailedToSetupList(GnbCuTnlAssociationFailedToSetupList),
    #[asn(key = 132)]
    IdGnbCuTnlAssociationSetupList(GnbCuTnlAssociationSetupList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 254)]
    IdTransportLayerAddressInfo(TransportLayerAddressInfo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbcuConfigurationUpdateAcknowledgeProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbcuConfigurationUpdateAcknowledgeProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbcuConfigurationUpdateAcknowledgeProtocolIEs(
    pub Vec<GnbcuConfigurationUpdateAcknowledgeProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbcuConfigurationUpdateFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 77)]
    IdTimeToWait(TimeToWait),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbcuConfigurationUpdateFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbcuConfigurationUpdateFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbcuConfigurationUpdateFailureProtocolIEs(
    pub Vec<GnbcuConfigurationUpdateFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduConfigurationUpdateProtocolIEsEntryValue {
    #[asn(key = 89)]
    IdCellsStatusList(CellsStatusList),
    #[asn(key = 189)]
    IdDedicatedSiDeliveryNeededUeList(DedicatedSiDeliveryNeededUeList),
    #[asn(key = 228)]
    IdGnbDuTnlAssociationToRemoveList(GnbDuTnlAssociationToRemoveList),
    #[asn(key = 58)]
    IdServedCellsToAddList(ServedCellsToAddList),
    #[asn(key = 60)]
    IdServedCellsToDeleteList(ServedCellsToDeleteList),
    #[asn(key = 62)]
    IdServedCellsToModifyList(ServedCellsToModifyList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 254)]
    IdTransportLayerAddressInfo(TransportLayerAddressInfo),
    #[asn(key = 42)]
    IdGNbDuId(GnbDuId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduConfigurationUpdateProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduConfigurationUpdateProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduConfigurationUpdateProtocolIEs(pub Vec<GnbduConfigurationUpdateProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduConfigurationUpdateAcknowledgeProtocolIEsEntryValue {
    #[asn(key = 281)]
    IdBapAddress(BapAddress),
    #[asn(key = 3)]
    IdCellsToBeActivatedList(CellsToBeActivatedList),
    #[asn(key = 5)]
    IdCellsToBeDeactivatedList(CellsToBeDeactivatedList),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 254)]
    IdTransportLayerAddressInfo(TransportLayerAddressInfo),
    #[asn(key = 287)]
    IdUlBhNonUpTrafficMapping(UlBhNonUpTrafficMapping),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduConfigurationUpdateAcknowledgeProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduConfigurationUpdateAcknowledgeProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduConfigurationUpdateAcknowledgeProtocolIEs(
    pub Vec<GnbduConfigurationUpdateAcknowledgeProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduConfigurationUpdateFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 77)]
    IdTimeToWait(TimeToWait),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduConfigurationUpdateFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduConfigurationUpdateFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduConfigurationUpdateFailureProtocolIEs(
    pub Vec<GnbduConfigurationUpdateFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduResourceConfigurationProtocolIEsEntryValue {
    #[asn(key = 288)]
    IdActivatedCellsToBeUpdatedList(ActivatedCellsToBeUpdatedList),
    #[asn(key = 289)]
    IdChildNodesList(ChildNodesList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduResourceConfigurationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduResourceConfigurationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduResourceConfigurationProtocolIEs(
    pub Vec<GnbduResourceConfigurationProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduResourceConfigurationAcknowledgeProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduResourceConfigurationAcknowledgeProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduResourceConfigurationAcknowledgeProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduResourceConfigurationAcknowledgeProtocolIEs(
    pub Vec<GnbduResourceConfigurationAcknowledgeProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduResourceConfigurationFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 77)]
    IdTimeToWait(TimeToWait),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduResourceConfigurationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduResourceConfigurationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduResourceConfigurationFailureProtocolIEs(
    pub Vec<GnbduResourceConfigurationFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduResourceCoordinationRequestProtocolIEsEntryValue {
    #[asn(key = 101)]
    IdEutraNrCellResourceCoordinationReqContainer(EutraNrCellResourceCoordinationReqContainer),
    #[asn(key = 213)]
    IdIgnoreResourceCoordinationContainer(IgnoreResourceCoordinationContainer),
    #[asn(key = 106)]
    IdRequestType(RequestType),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduResourceCoordinationRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduResourceCoordinationRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduResourceCoordinationRequestProtocolIEs(
    pub Vec<GnbduResourceCoordinationRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduResourceCoordinationResponseProtocolIEsEntryValue {
    #[asn(key = 102)]
    IdEutraNrCellResourceCoordinationReqAckContainer(
        EutraNrCellResourceCoordinationReqAckContainer,
    ),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduResourceCoordinationResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduResourceCoordinationResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduResourceCoordinationResponseProtocolIEs(
    pub Vec<GnbduResourceCoordinationResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GnbduStatusIndicationProtocolIEsEntryValue {
    #[asn(key = 172)]
    IdGnbduOverloadInformation(GnbduOverloadInformation),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbduStatusIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GnbduStatusIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct GnbduStatusIndicationProtocolIEs(pub Vec<GnbduStatusIndicationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1970049")]
pub struct Integer48(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "985025")]
pub struct Integer49(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "492513")]
pub struct Integer50(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "246257")]
pub struct Integer51(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "123129")]
pub struct Integer52(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "61565")]
pub struct Integer53(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbRxTxTimeDiffMeaschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GtptlaItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GtptlaItemIeExtensions(pub Vec<GtptlaItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GtpTunnelIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GtpTunnelIeExtensions(pub Vec<GtpTunnelIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GeographicalCoordinatesIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GeographicalCoordinatesIeExtensions(pub Vec<GeographicalCoordinatesIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HsnaSlotConfigItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HsnaSlotConfigItemIeExtensions(pub Vec<HsnaSlotConfigItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100", extensible = true)]
pub struct Integer54(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100", extensible = true)]
pub struct Integer55(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HardwareLoadIndicatorIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HardwareLoadIndicatorIeExtensions(pub Vec<HardwareLoadIndicatorIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabAllocatedTnlAddressItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabAllocatedTnlAddressItemIeExtensions(
    pub Vec<IabAllocatedTnlAddressItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabAllocatedTnlAddressListEntryValue {
    #[asn(key = 295)]
    IdIabAllocatedTnlAddressItem(IabAllocatedTnlAddressItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabAllocatedTnlAddressListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabAllocatedTnlAddressListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabDuCellResourceConfigurationFddInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabDuCellResourceConfigurationFddInfoIeExtensions(
    pub Vec<IabDuCellResourceConfigurationFddInfoIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabDuCellResourceConfigurationModeInfochoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabDuCellResourceConfigurationTddInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabDuCellResourceConfigurationTddInfoIeExtensions(
    pub Vec<IabDuCellResourceConfigurationTddInfoIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabInfoIabDuIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabInfoIabDuIeExtensions(pub Vec<IabInfoIabDuIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabInfoIabDonorCuIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabInfoIabDonorCuIeExtensions(pub Vec<IabInfoIabDonorCuIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabMtCellListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabMtCellListItemIeExtensions(pub Vec<IabMtCellListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabStcInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabStcInfoIeExtensions(pub Vec<IabStcInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabStcInfoItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabStcInfoItemIeExtensions(pub Vec<IabStcInfoItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabTnlAddressesToRemoveItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabTnlAddressesToRemoveItemIeExtensions(
    pub Vec<IabTnlAddressesToRemoveItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabTnlAddressesToRemoveListEntryValue {
    #[asn(key = 293)]
    IdIabTnlAddressesToRemoveItem(IabTnlAddressesToRemoveItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabTnlAddressesToRemoveListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabTnlAddressesToRemoveListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabiPv6RequestTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct BitString56(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct BitString57(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct BitString58(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabtnlAddresschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabtnlAddressFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 77)]
    IdTimeToWait(TimeToWait),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabtnlAddressFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabtnlAddressFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct IabtnlAddressFailureProtocolIEs(pub Vec<IabtnlAddressFailureProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabtnlAddressRequestProtocolIEsEntryValue {
    #[asn(key = 292)]
    IdIabTnlAddressesToRemoveList(IabTnlAddressesToRemoveList),
    #[asn(key = 296)]
    IdIabiPv6RequestType(IabiPv6RequestType),
    #[asn(key = 297)]
    IdIaBv4AddressesRequested(IaBv4AddressesRequested),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabtnlAddressRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabtnlAddressRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct IabtnlAddressRequestProtocolIEs(pub Vec<IabtnlAddressRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabtnlAddressResponseProtocolIEsEntryValue {
    #[asn(key = 294)]
    IdIabAllocatedTnlAddressList(IabAllocatedTnlAddressList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabtnlAddressResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabtnlAddressResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct IabtnlAddressResponseProtocolIEs(pub Vec<IabtnlAddressResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct Integer59(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct Integer60(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct Integer61(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct Integer62(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabtnlAddressesRequestedIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IabtnlAddressesRequestedIeExtensions(pub Vec<IabtnlAddressesRequestedIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabupConfigurationUpdateFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 77)]
    IdTimeToWait(TimeToWait),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabupConfigurationUpdateFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabupConfigurationUpdateFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct IabupConfigurationUpdateFailureProtocolIEs(
    pub Vec<IabupConfigurationUpdateFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabupConfigurationUpdateRequestProtocolIEsEntryValue {
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 302)]
    IdUlUpTnlAddressToUpdateList(UlUpTnlAddressToUpdateList),
    #[asn(key = 300)]
    IdUlUpTnlInformationToUpdateList(UlUpTnlInformationToUpdateList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabupConfigurationUpdateRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabupConfigurationUpdateRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct IabupConfigurationUpdateRequestProtocolIEs(
    pub Vec<IabupConfigurationUpdateRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum IabupConfigurationUpdateResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 304)]
    IdDlUpTnlAddressToUpdateList(DlUpTnlAddressToUpdateList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IabupConfigurationUpdateResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: IabupConfigurationUpdateResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct IabupConfigurationUpdateResponseProtocolIEs(
    pub Vec<IabupConfigurationUpdateResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IaBv4AddressesRequestedIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IaBv4AddressesRequestedIeExtensions(pub Vec<IaBv4AddressesRequestedIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BitString63(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IpHeaderInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IpHeaderInformationIeExtensions(pub Vec<IpHeaderInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IPtolayer2TrafficMappingInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IPtolayer2TrafficMappingInfoIeExtensions(
    pub Vec<IPtolayer2TrafficMappingInfoIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IPtolayer2TrafficMappingInfoItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IPtolayer2TrafficMappingInfoItemIeExtensions(
    pub Vec<IPtolayer2TrafficMappingInfoItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImplicitFormatIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ImplicitFormatIeExtensions(pub Vec<ImplicitFormatIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUlrrcMessageTransferProtocolIEsEntryValue {
    #[asn(key = 95)]
    IdCRnti(CRnti),
    #[asn(key = 128)]
    IdDUtoCurrcContainer(DUtoCurrcContainer),
    #[asn(key = 111)]
    IdNrcgi(Nrcgi),
    #[asn(key = 226)]
    IdRanueid(Ranueid),
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 241)]
    IdRrcContainerRrcSetupComplete(RrcContainerRrcSetupComplete),
    #[asn(key = 178)]
    IdSulAccessIndication(SulAccessIndication),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUlrrcMessageTransferProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialUlrrcMessageTransferProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialUlrrcMessageTransferProtocolIEs(
    pub Vec<InitialUlrrcMessageTransferProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessageValue {
    #[asn(key = 32)]
    IdBapMappingConfiguration(BapMappingConfiguration),
    #[asn(key = 31)]
    IdCuduRadioInformationTransfer(CuduRadioInformationTransfer),
    #[asn(key = 12)]
    IdDlrrcMessageTransfer(DlrrcMessageTransfer),
    #[asn(key = 30)]
    IdDucuRadioInformationTransfer(DucuRadioInformationTransfer),
    #[asn(key = 29)]
    IdDeactivateTrace(DeactivateTrace),
    #[asn(key = 53)]
    IdECidMeasurementFailureIndication(ECidMeasurementFailureIndication),
    #[asn(key = 52)]
    IdECidMeasurementInitiation(ECidMeasurementInitiationRequest),
    #[asn(key = 54)]
    IdECidMeasurementReport(ECidMeasurementReport),
    #[asn(key = 55)]
    IdECidMeasurementTermination(ECidMeasurementTerminationCommand),
    #[asn(key = 2)]
    IdErrorIndication(ErrorIndication),
    #[asn(key = 26)]
    IdF1Removal(F1RemovalRequest),
    #[asn(key = 1)]
    IdF1Setup(F1SetupRequest),
    #[asn(key = 33)]
    IdGnbduResourceConfiguration(GnbduResourceConfiguration),
    #[asn(key = 16)]
    IdGnbduResourceCoordination(GnbduResourceCoordinationRequest),
    #[asn(key = 24)]
    IdGnbduStatusIndication(GnbduStatusIndication),
    #[asn(key = 34)]
    IdIabtnlAddressAllocation(IabtnlAddressRequest),
    #[asn(key = 35)]
    IdIabupConfigurationUpdate(IabupConfigurationUpdateRequest),
    #[asn(key = 11)]
    IdInitialUlrrcMessageTransfer(InitialUlrrcMessageTransfer),
    #[asn(key = 27)]
    IdNetworkAccessRateReduction(NetworkAccessRateReduction),
    #[asn(key = 19)]
    IdNotify(Notify),
    #[asn(key = 21)]
    IdPwsCancel(PwsCancelRequest),
    #[asn(key = 23)]
    IdPwsFailureIndication(PwsFailureIndication),
    #[asn(key = 22)]
    IdPwsRestartIndication(PwsRestartIndication),
    #[asn(key = 18)]
    IdPaging(Paging),
    #[asn(key = 50)]
    IdPositioningActivation(PositioningActivationRequest),
    #[asn(key = 42)]
    IdPositioningAssistanceInformationControl(PositioningAssistanceInformationControl),
    #[asn(key = 43)]
    IdPositioningAssistanceInformationFeedback(PositioningAssistanceInformationFeedback),
    #[asn(key = 51)]
    IdPositioningDeactivation(PositioningDeactivation),
    #[asn(key = 49)]
    IdPositioningInformationExchange(PositioningInformationRequest),
    #[asn(key = 56)]
    IdPositioningInformationUpdate(PositioningInformationUpdate),
    #[asn(key = 45)]
    IdPositioningMeasurementAbort(PositioningMeasurementAbort),
    #[asn(key = 41)]
    IdPositioningMeasurementExchange(PositioningMeasurementRequest),
    #[asn(key = 46)]
    IdPositioningMeasurementFailureIndication(PositioningMeasurementFailureIndication),
    #[asn(key = 44)]
    IdPositioningMeasurementReport(PositioningMeasurementReport),
    #[asn(key = 47)]
    IdPositioningMeasurementUpdate(PositioningMeasurementUpdate),
    #[asn(key = 25)]
    IdRrcDeliveryReport(RrcDeliveryReport),
    #[asn(key = 57)]
    IdReferenceTimeInformationReport(ReferenceTimeInformationReport),
    #[asn(key = 58)]
    IdReferenceTimeInformationReportingControl(ReferenceTimeInformationReportingControl),
    #[asn(key = 0)]
    IdReset(Reset),
    #[asn(key = 17)]
    IdSystemInformationDeliveryCommand(SystemInformationDeliveryCommand),
    #[asn(key = 48)]
    IdTrpInformationExchange(TrpInformationRequest),
    #[asn(key = 28)]
    IdTraceStart(TraceStart),
    #[asn(key = 7)]
    IdUeContextModification(UeContextModificationRequest),
    #[asn(key = 8)]
    IdUeContextModificationRequired(UeContextModificationRequired),
    #[asn(key = 6)]
    IdUeContextRelease(UeContextReleaseCommand),
    #[asn(key = 10)]
    IdUeContextReleaseRequest(UeContextReleaseRequest),
    #[asn(key = 5)]
    IdUeContextSetup(UeContextSetupRequest),
    #[asn(key = 15)]
    IdUeInactivityNotification(UeInactivityNotification),
    #[asn(key = 13)]
    IdUlrrcMessageTransfer(UlrrcMessageTransfer),
    #[asn(key = 20)]
    IdWriteReplaceWarning(WriteReplaceWarningRequest),
    #[asn(key = 38)]
    IdAccessAndMobilityIndication(AccessAndMobilityIndication),
    #[asn(key = 39)]
    IdAccessSuccess(AccessSuccess),
    #[asn(key = 40)]
    IdCellTrafficTrace(CellTrafficTrace),
    #[asn(key = 4)]
    IdGNbcuConfigurationUpdate(GnbcuConfigurationUpdate),
    #[asn(key = 3)]
    IdGNbduConfigurationUpdate(GnbduConfigurationUpdate),
    #[asn(key = 14)]
    IdPrivateMessage(PrivateMessage),
    #[asn(key = 37)]
    IdResourceStatusReporting(ResourceStatusUpdate),
    #[asn(key = 36)]
    IdResourceStatusReportingInitiation(ResourceStatusRequest),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated64(pub u8);
impl Enumerated64 {
    const SCS15: u8 = 0u8;
    const SCS30: u8 = 1u8;
    const SCS60: u8 = 2u8;
    const SCS120: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated65(pub u8);
impl Enumerated65 {
    const NORMAL: u8 = 0u8;
    const EXTENDED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "17")]
pub struct Enumerated66(pub u8);
impl Enumerated66 {
    const MS0P5: u8 = 0u8;
    const MS0P625: u8 = 1u8;
    const MS1: u8 = 2u8;
    const MS1P25: u8 = 3u8;
    const MS2: u8 = 4u8;
    const MS2P5: u8 = 5u8;
    const MS3: u8 = 6u8;
    const MS4: u8 = 7u8;
    const MS5: u8 = 8u8;
    const MS10: u8 = 9u8;
    const MS20: u8 = 10u8;
    const MS40: u8 = 11u8;
    const MS60: u8 = 12u8;
    const MS80: u8 = 13u8;
    const MS100: u8 = 14u8;
    const MS120: u8 = 15u8;
    const MS140: u8 = 16u8;
    const MS160: u8 = 17u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntendedTddDlUlConfigIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntendedTddDlUlConfigIeExtensions(pub Vec<IntendedTddDlUlConfigIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated67(pub u8);
impl Enumerated67 {
    const SCS15: u8 = 0u8;
    const SCS30: u8 = 1u8;
    const SCS60: u8 = 2u8;
    const SCS120: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "137")]
pub struct Integer68(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct L139InfoIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct L139InfoIeExtension(pub Vec<L139InfoIeExtensionEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "837")]
pub struct Integer69(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated70(pub u8);
impl Enumerated70 {
    const UNRESTRICTED_SET: u8 = 0u8;
    const RESTRICTED_SET_TYPE_A: u8 = 1u8;
    const RESTRICTED_SET_TYPE_B: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct L839InfoIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct L839InfoIeExtension(pub Vec<L839InfoIeExtensionEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct Integer71(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct Integer72(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct Integer73(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LcsToGcsTranslationAoAIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LcsToGcsTranslationAoAIeExtensions(pub Vec<LcsToGcsTranslationAoAIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct Integer74(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct Integer75(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct Integer76(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct Integer77(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct Integer78(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct Integer79(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LcStoGcsTranslationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LcStoGcsTranslationIeExtensions(pub Vec<LcStoGcsTranslationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LteueSidelinkAggregateMaximumBitrateIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LteueSidelinkAggregateMaximumBitrateIeExtensions(
    pub Vec<LteueSidelinkAggregateMaximumBitrateIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ltev2xServicesAuthorizedIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Ltev2xServicesAuthorizedIeExtensions(pub Vec<Ltev2xServicesAuthorizedIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer80(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer81(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer82(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer83(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationUncertaintyIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationUncertaintyIeExtensions(pub Vec<LocationUncertaintyIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M5ConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M5ConfigurationIeExtensions(pub Vec<M5ConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M6ConfigurationIeExtensions(pub Vec<M6ConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M7ConfigurationIeExtensions(pub Vec<M7ConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MdtConfigurationIeExtensions(pub Vec<MdtConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasuredResultsValuechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementBeamInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MeasurementBeamInfoIeExtensions(pub Vec<MeasurementBeamInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MultiplexingInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MultiplexingInfoIeExtensions(pub Vec<MultiplexingInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgranAllocationAndRetentionPriorityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NgranAllocationAndRetentionPriorityIeExtensions(
    pub Vec<NgranAllocationAndRetentionPriorityIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct Integer84(pub i32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct Integer85(pub i32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-64000", ub = "1280000")]
pub struct Integer86(pub i32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer87(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer88(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct Integer89(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer90(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer91(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer92(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgranHighAccuracyAccessPointPositionIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NgranHighAccuracyAccessPointPositionIeExtensions(
    pub Vec<NgranHighAccuracyAccessPointPositionIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnBroadcastInformationPniNpnIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NpnBroadcastInformationPniNpnIeExtension(
    pub Vec<NpnBroadcastInformationPniNpnIeExtensionEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnBroadcastInformationSnpnIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NpnBroadcastInformationSnpnIeExtension(
    pub Vec<NpnBroadcastInformationSnpnIeExtensionEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnBroadcastInformationchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnSupportInfochoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrCgiListForRestartItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrCgiListForRestartItemIeExtensions(pub Vec<NrCgiListForRestartItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NrCgiListForRestartListEntryValue {
    #[asn(key = 153)]
    IdNrCgiListForRestartItem(NrCgiListForRestartItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrCgiListForRestartListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NrCgiListForRestartListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrModeInfochoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrPrsBeamInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrPrsBeamInformationIeExtensions(pub Vec<NrPrsBeamInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrPrsBeamInformationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrPrsBeamInformationItemIeExtensions(pub Vec<NrPrsBeamInformationItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrcgiIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrcgiIeExtensions(pub Vec<NrcgiIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2199", extensible = true)]
pub struct Integer93(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "275", extensible = true)]
pub struct Integer94(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrCarrierItemIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrCarrierItemIeExtension(pub Vec<NrCarrierItemIeExtensionEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer95(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NrFreqInfoFreqBandListNr(pub Vec<FreqBandNrItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NrFreqInfoIeExtensionsEntryExtensionValue {
    #[asn(key = 356)]
    IdFrequencyShift7p5khz(FrequencyShift7p5khz),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrFreqInfoIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: NrFreqInfoIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrFreqInfoIeExtensions(pub Vec<NrFreqInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrprachConfigIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrprachConfigIeExtension(pub Vec<NrprachConfigIeExtensionEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "274", extensible = true)]
pub struct Integer96(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated97(pub u8);
impl Enumerated97 {
    const ONE: u8 = 0u8;
    const TWO: u8 = 1u8;
    const FOUR: u8 = 2u8;
    const EIGHT: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct Integer98(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct Enumerated99(pub u8);
impl Enumerated99 {
    const ONE_EIGHTH: u8 = 0u8;
    const ONE_FOURTH: u8 = 1u8;
    const ONE_HALF: u8 = 2u8;
    const ONE: u8 = 3u8;
    const TWO: u8 = 4u8;
    const FOUR: u8 = 5u8;
    const EIGHT: u8 = 6u8;
    const SIXTEEN: u8 = 7u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct Integer100(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrprachConfigItemIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrprachConfigItemIeExtension(pub Vec<NrprachConfigItemIeExtensionEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrueSidelinkAggregateMaximumBitrateIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NrueSidelinkAggregateMaximumBitrateIeExtensions(
    pub Vec<NrueSidelinkAggregateMaximumBitrateIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Nrv2xServicesAuthorizedIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Nrv2xServicesAuthorizedIeExtensions(pub Vec<Nrv2xServicesAuthorizedIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NeighbourCellInformationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NeighbourCellInformationItemIeExtensions(
    pub Vec<NeighbourCellInformationItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NeighbourCellInformationListEntryValue {
    #[asn(key = 255)]
    IdNeighbourCellInformationItem(NeighbourCellInformationItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NeighbourCellInformationListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NeighbourCellInformationListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NetworkAccessRateReductionProtocolIEsEntryValue {
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 225)]
    IdUacAssistanceInfo(UacAssistanceInfo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NetworkAccessRateReductionProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NetworkAccessRateReductionProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NetworkAccessRateReductionProtocolIEs(
    pub Vec<NetworkAccessRateReductionProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct Integer101(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "127")]
pub struct Integer102(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NonDynamic5QiDescriptorIeExtensionsEntryExtensionValue {
    #[asn(key = 362)]
    IdCnPacketDelayBudgetDownlink(ExtendedPacketDelayBudget),
    #[asn(key = 369)]
    IdCnPacketDelayBudgetUplink(ExtendedPacketDelayBudget),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamic5QiDescriptorIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: NonDynamic5QiDescriptorIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NonDynamic5QiDescriptorIeExtensions(pub Vec<NonDynamic5QiDescriptorIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct Integer103(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8", extensible = true)]
pub struct Integer104(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamicPqiDescriptorIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NonDynamicPqiDescriptorIeExtensions(pub Vec<NonDynamicPqiDescriptorIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NotificationInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NotificationInformationIeExtensions(pub Vec<NotificationInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NotifyProtocolIEsEntryValue {
    #[asn(key = 137)]
    IdDrbNotifyList(DrbNotifyList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NotifyProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NotifyProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NotifyProtocolIEs(pub Vec<NotifyProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "13", extensible = true)]
pub struct Integer105(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "13", extensible = true)]
pub struct Integer106(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NumDlulSymbolsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NumDlulSymbolsIeExtensions(pub Vec<NumDlulSymbolsIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5QoSCharacteristicschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5FlowBitRatesIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Pc5FlowBitRatesIeExtensions(pub Vec<Pc5FlowBitRatesIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5QoSParametersIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Pc5QoSParametersIeExtensions(pub Vec<Pc5QoSParametersIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct Integer107(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct Integer108(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "180")]
pub struct Integer109(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct Integer110(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsAngleItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsAngleItemIeExtensions(pub Vec<PrsAngleItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsConfigurationIeExtensions(pub Vec<PrsConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer111(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct Integer112(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer113(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsInformationPosIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsInformationPosIeExtensions(pub Vec<PrsInformationPosIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsMutingIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsMutingIeExtensions(pub Vec<PrsMutingIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated114(pub u8);
impl Enumerated114 {
    const RF1: u8 = 0u8;
    const RF2: u8 = 1u8;
    const RF4: u8 = 2u8;
    const RF8: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsMutingOption1IeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsMutingOption1IeExtensions(pub Vec<PrsMutingOption1IeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsMutingOption2IeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsMutingOption2IeExtensions(pub Vec<PrsMutingOption2IeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer115(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "11", extensible = true)]
pub struct Integer116(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct Integer117(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "12")]
pub struct Integer118(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsResourceItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsResourceItemIeExtensions(pub Vec<PrsResourceItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsResourceQclInfochoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsResourceQclSourcePrsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsResourceQclSourcePrsIeExtensions(pub Vec<PrsResourceQclSourcePrsIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007")]
pub struct Integer119(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsResourceQclSourceSsbIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsResourceQclSourceSsbIeExtensions(pub Vec<PrsResourceQclSourceSsbIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated120(pub u8);
impl Enumerated120 {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ60: u8 = 2u8;
    const K_HZ120: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "63")]
pub struct Integer121(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2176")]
pub struct Integer122(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer123(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated124(pub u8);
impl Enumerated124 {
    const N2: u8 = 0u8;
    const N4: u8 = 1u8;
    const N6: u8 = 2u8;
    const N12: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated125(pub u8);
impl Enumerated125 {
    const NORMAL: u8 = 0u8;
    const EXTENDED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "19")]
pub struct Enumerated126(pub u8);
impl Enumerated126 {
    const N4: u8 = 0u8;
    const N5: u8 = 1u8;
    const N8: u8 = 2u8;
    const N10: u8 = 3u8;
    const N16: u8 = 4u8;
    const N20: u8 = 5u8;
    const N32: u8 = 6u8;
    const N40: u8 = 7u8;
    const N64: u8 = 8u8;
    const N80: u8 = 9u8;
    const N160: u8 = 10u8;
    const N320: u8 = 11u8;
    const N640: u8 = 12u8;
    const N1280: u8 = 13u8;
    const N2560: u8 = 14u8;
    const N5120: u8 = 15u8;
    const N10240: u8 = 16u8;
    const N20480: u8 = 17u8;
    const N40960: u8 = 18u8;
    const N81920: u8 = 19u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "81919", extensible = true)]
pub struct Integer127(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct Enumerated128(pub u8);
impl Enumerated128 {
    const RF1: u8 = 0u8;
    const RF2: u8 = 1u8;
    const RF4: u8 = 2u8;
    const RF6: u8 = 3u8;
    const RF8: u8 = 4u8;
    const RF16: u8 = 5u8;
    const RF32: u8 = 6u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct Enumerated129(pub u8);
impl Enumerated129 {
    const TG1: u8 = 0u8;
    const TG2: u8 = 1u8;
    const TG4: u8 = 2u8;
    const TG8: u8 = 3u8;
    const TG16: u8 = 4u8;
    const TG32: u8 = 5u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated130(pub u8);
impl Enumerated130 {
    const N2: u8 = 0u8;
    const N4: u8 = 1u8;
    const N6: u8 = 2u8;
    const N12: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-60", ub = "50")]
pub struct Integer131(pub i8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrsResourceSetItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrsResourceSetItemIeExtensions(pub Vec<PrsResourceSetItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailedNrCgiItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PwsFailedNrCgiItemIeExtensions(pub Vec<PwsFailedNrCgiItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsFailedNrCgiListEntryValue {
    #[asn(key = 155)]
    IdPwsFailedNrCgiItem(PwsFailedNrCgiItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailedNrCgiListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsFailedNrCgiListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsCancelRequestProtocolIEsEntryValue {
    #[asn(key = 148)]
    IdBroadcastToBeCancelledList(BroadcastToBeCancelledList),
    #[asn(key = 157)]
    IdCancelAllWarningMessagesIndicator(CancelAllWarningMessagesIndicator),
    #[asn(key = 220)]
    IdNotificationInformation(NotificationInformation),
    #[asn(key = 142)]
    IdNumberofBroadcastRequest(NumberofBroadcastRequest),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsCancelRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsCancelRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PwsCancelRequestProtocolIEs(pub Vec<PwsCancelRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsCancelResponseProtocolIEsEntryValue {
    #[asn(key = 150)]
    IdCellsBroadcastCancelledList(CellsBroadcastCancelledList),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsCancelResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsCancelResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PwsCancelResponseProtocolIEs(pub Vec<PwsCancelResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsFailureIndicationProtocolIEsEntryValue {
    #[asn(key = 154)]
    IdPwsFailedNrCgiList(PwsFailedNrCgiList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailureIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsFailureIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PwsFailureIndicationProtocolIEs(pub Vec<PwsFailureIndicationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsRestartIndicationProtocolIEsEntryValue {
    #[asn(key = 152)]
    IdNrCgiListForRestartList(NrCgiListForRestartList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsRestartIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsRestartIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PwsRestartIndicationProtocolIEs(pub Vec<PwsRestartIndicationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString132(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsSystemInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 231)]
    IdAdditionalSibMessageList(AdditionalSibMessageList),
    #[asn(key = 220)]
    IdNotificationInformation(NotificationInformation),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsSystemInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PwsSystemInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PwsSystemInformationIeExtensions(pub Vec<PwsSystemInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PacketErrorRateIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PacketErrorRateIeExtensions(pub Vec<PacketErrorRateIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingProtocolIEsEntryValue {
    #[asn(key = 113)]
    IdPagingCellList(PagingCellList),
    #[asn(key = 114)]
    IdPagingDrx(PagingDrx),
    #[asn(key = 127)]
    IdPagingIdentity(PagingIdentity),
    #[asn(key = 216)]
    IdPagingOrigin(PagingOrigin),
    #[asn(key = 115)]
    IdPagingPriority(PagingPriority),
    #[asn(key = 117)]
    IdUeIdentityIndexValue(UeIdentityIndexValue),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PagingProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PagingProtocolIEs(pub Vec<PagingProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingCellItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingCellItemIeExtensions(pub Vec<PagingCellItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingCellListEntryValue {
    #[asn(key = 112)]
    IdPagingCellItem(PagingCellItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingCellListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PagingCellListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingIdentitychoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathlossReferenceInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathlossReferenceInfoIeExtensions(pub Vec<PathlossReferenceInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathlossReferenceSignalchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PeriodicityListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PeriodicityListItemIeExtensions(pub Vec<PeriodicityListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "5")]
pub struct Integer133(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosMeasurementQuantitiesItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosMeasurementQuantitiesItemIeExtensions(
    pub Vec<PosMeasurementQuantitiesItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosMeasurementResultItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosMeasurementResultItemIeExtensions(pub Vec<PosMeasurementResultItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PosMeasurementResultListItemIeExtensionsEntryExtensionValue {
    #[asn(key = 111)]
    IdNrcgi(Nrcgi),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosMeasurementResultListItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PosMeasurementResultListItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosMeasurementResultListItemIeExtensions(
    pub Vec<PosMeasurementResultListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosResourceSetTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3")]
pub struct Integer134(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosResourceSetTypeApIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosResourceSetTypeApIeExtensions(pub Vec<PosResourceSetTypeApIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated135(pub u8);
impl Enumerated135 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosResourceSetTypePrIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosResourceSetTypePrIeExtensions(pub Vec<PosResourceSetTypePrIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated136(pub u8);
impl Enumerated136 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosResourceSetTypeSpIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosResourceSetTypeSpIeExtensions(pub Vec<PosResourceSetTypeSpIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "13")]
pub struct Integer137(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct Enumerated138(pub u8);
impl Enumerated138 {
    const N1: u8 = 0u8;
    const N2: u8 = 1u8;
    const N4: u8 = 2u8;
    const N8: u8 = 3u8;
    const N12: u8 = 4u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "268")]
pub struct Integer139(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer140(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Enumerated141(pub u8);
impl Enumerated141 {
    const NEITHER: u8 = 0u8;
    const GROUP_HOPPING: u8 = 1u8;
    const SEQUENCE_HOPPING: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct Integer142(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosSrsResourceItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosSrsResourceItemIeExtensions(pub Vec<PosSrsResourceItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct Integer143(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosSrsResourceSetItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosSrsResourceSetItemIeExtensions(pub Vec<PosSrsResourceSetItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningActivationFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningActivationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningActivationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningActivationFailureProtocolIEs(
    pub Vec<PositioningActivationFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningActivationRequestProtocolIEsEntryValue {
    #[asn(key = 404)]
    IdActivationTime(RelativeTime1900),
    #[asn(key = 403)]
    IdSrsType(SrsType),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningActivationRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningActivationRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningActivationRequestProtocolIEs(
    pub Vec<PositioningActivationRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningActivationResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 421)]
    IdSlotNumber(SlotNumber),
    #[asn(key = 420)]
    IdSystemFrameNumber(SystemFrameNumber),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningActivationResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningActivationResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningActivationResponseProtocolIEs(
    pub Vec<PositioningActivationResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningAssistanceInformationControlProtocolIEsEntryValue {
    #[asn(key = 392)]
    IdPosAssistanceInformation(PosAssistanceInformation),
    #[asn(key = 393)]
    IdPosBroadcast(PosBroadcast),
    #[asn(key = 406)]
    IdPositioningBroadcastCells(PositioningBroadcastCells),
    #[asn(key = 394)]
    IdRoutingId(RoutingId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningAssistanceInformationControlProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningAssistanceInformationControlProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningAssistanceInformationControlProtocolIEs(
    pub Vec<PositioningAssistanceInformationControlProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningAssistanceInformationFeedbackProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 395)]
    IdPosAssistanceInformationFailureList(PosAssistanceInformationFailureList),
    #[asn(key = 406)]
    IdPositioningBroadcastCells(PositioningBroadcastCells),
    #[asn(key = 394)]
    IdRoutingId(RoutingId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningAssistanceInformationFeedbackProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningAssistanceInformationFeedbackProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningAssistanceInformationFeedbackProtocolIEs(
    pub Vec<PositioningAssistanceInformationFeedbackProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningDeactivationProtocolIEsEntryValue {
    #[asn(key = 405)]
    IdAbortTransmission(AbortTransmission),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningDeactivationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningDeactivationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningDeactivationProtocolIEs(pub Vec<PositioningDeactivationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningInformationFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningInformationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningInformationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningInformationFailureProtocolIEs(
    pub Vec<PositioningInformationFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningInformationRequestProtocolIEsEntryValue {
    #[asn(key = 391)]
    IdRequestedSrsTransmissionCharacteristics(RequestedSrsTransmissionCharacteristics),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningInformationRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningInformationRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningInformationRequestProtocolIEs(
    pub Vec<PositioningInformationRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningInformationResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 419)]
    IdSfnInitialisationTime(RelativeTime1900),
    #[asn(key = 407)]
    IdSrsConfiguration(SrsConfiguration),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningInformationResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningInformationResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningInformationResponseProtocolIEs(
    pub Vec<PositioningInformationResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningInformationUpdateProtocolIEsEntryValue {
    #[asn(key = 419)]
    IdSfnInitialisationTime(RelativeTime1900),
    #[asn(key = 407)]
    IdSrsConfiguration(SrsConfiguration),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningInformationUpdateProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningInformationUpdateProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningInformationUpdateProtocolIEs(
    pub Vec<PositioningInformationUpdateProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementAbortProtocolIEsEntryValue {
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementAbortProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementAbortProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementAbortProtocolIEs(
    pub Vec<PositioningMeasurementAbortProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementFailureProtocolIEs(
    pub Vec<PositioningMeasurementFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementFailureIndicationProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementFailureIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementFailureIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementFailureIndicationProtocolIEs(
    pub Vec<PositioningMeasurementFailureIndicationProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementReportProtocolIEsEntryValue {
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 397)]
    IdPosMeasurementResultList(PosMeasurementResultList),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementReportProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementReportProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementReportProtocolIEs(
    pub Vec<PositioningMeasurementReportProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementRequestProtocolIEsEntryValue {
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 423)]
    IdMeasurementBeamInfoRequest(MeasurementBeamInfoRequest),
    #[asn(key = 409)]
    IdPosMeasurementPeriodicity(MeasurementPeriodicity),
    #[asn(key = 396)]
    IdPosMeasurementQuantities(PosMeasurementQuantities),
    #[asn(key = 408)]
    IdPosReportCharacteristics(PosReportCharacteristics),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 419)]
    IdSfnInitialisationTime(RelativeTime1900),
    #[asn(key = 407)]
    IdSrsConfiguration(SrsConfiguration),
    #[asn(key = 421)]
    IdSlotNumber(SlotNumber),
    #[asn(key = 420)]
    IdSystemFrameNumber(SystemFrameNumber),
    #[asn(key = 422)]
    IdTrpMeasurementRequestList(TrpMeasurementRequestList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementRequestProtocolIEs(
    pub Vec<PositioningMeasurementRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 397)]
    IdPosMeasurementResultList(PosMeasurementResultList),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementResponseProtocolIEs(
    pub Vec<PositioningMeasurementResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PositioningMeasurementUpdateProtocolIEsEntryValue {
    #[asn(key = 402)]
    IdLmfMeasurementId(LmfMeasurementId),
    #[asn(key = 411)]
    IdRanMeasurementId(RanMeasurementId),
    #[asn(key = 407)]
    IdSrsConfiguration(SrsConfiguration),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PositioningMeasurementUpdateProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PositioningMeasurementUpdateProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PositioningMeasurementUpdateProtocolIEs(
    pub Vec<PositioningMeasurementUpdateProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PotentialSpCellItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PotentialSpCellItemIeExtensions(pub Vec<PotentialSpCellItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PotentialSpCellListEntryValue {
    #[asn(key = 93)]
    IdPotentialSpCellItem(PotentialSpCellItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PotentialSpCellListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PotentialSpCellListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct Integer144(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct ObjectIdentifier145;

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessagePrivateIEsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrivateMessagePrivateIEs(pub Vec<PrivateMessagePrivateIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProtectedEutraResourcesItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ProtectedEutraResourcesItemIeExtensions(
    pub Vec<ProtectedEutraResourcesItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ProtectedEutraResourcesListEntryValue {
    #[asn(key = 168)]
    IdProtectedEutraResourcesItem(ProtectedEutraResourcesItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProtectedEutraResourcesListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ProtectedEutraResourcesListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSCharacteristicschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated146(pub u8);
impl Enumerated146 {
    const SUBJECT_TO: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QoSFlowLevelQoSParametersIeExtensionsEntryExtensionValue {
    #[asn(key = 180)]
    IdPduSessionId(PduSessionId),
    #[asn(key = 257)]
    IdQosMonitoringRequest(QosMonitoringRequest),
    #[asn(key = 181)]
    IdUlpduSessionAggregateMaximumBitRate(BitRate),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSFlowLevelQoSParametersIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QoSFlowLevelQoSParametersIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QoSFlowLevelQoSParametersIeExtensions(
    pub Vec<QoSFlowLevelQoSParametersIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QoSInformationchoiceExtensionValue {
    #[asn(key = 164)]
    IdDrbInformation(DrbInformation),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSInformationchoiceExtension {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: QoSInformationchoiceExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RachReportInformationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RachReportInformationItemIeExtensions(
    pub Vec<RachReportInformationItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "40", sz_ub = "40")]
pub struct BitString147(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanuePagingIdentityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RanuePagingIdentityIeExtensions(pub Vec<RanuePagingIdentityIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RatFrequencyPriorityInformationchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RlcStatusIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RlcStatusIeExtensions(pub Vec<RlcStatusIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RlcDuplicationInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RlcDuplicationInformationIeExtensions(
    pub Vec<RlcDuplicationInformationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RlcDuplicationStateItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RlcDuplicationStateItemIeExtensions(pub Vec<RlcDuplicationStateItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RlcFailureIndicationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RlcFailureIndicationIeExtensions(pub Vec<RlcFailureIndicationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RlfReportInformationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RlfReportInformationItemIeExtensions(pub Vec<RlfReportInformationItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct BitString148(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RrcVersionIeExtensionsEntryExtensionValue {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RrcVersionIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: RrcVersionIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RrcVersionIeExtensions(pub Vec<RrcVersionIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RrcDeliveryReportProtocolIEsEntryValue {
    #[asn(key = 185)]
    IdRrcDeliveryStatus(RrcDeliveryStatus),
    #[asn(key = 64)]
    IdSrbid(Srbid),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RrcDeliveryReportProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RrcDeliveryReportProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RrcDeliveryReportProtocolIEs(pub Vec<RrcDeliveryReportProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RrcDeliveryStatusIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RrcDeliveryStatusIeExtensions(pub Vec<RrcDeliveryStatusIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RadioResourceStatusIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RadioResourceStatusIeExtensions(pub Vec<RadioResourceStatusIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ReferencePointchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ReferenceSignalchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ReferenceTimeInformationReportProtocolIEsEntryValue {
    #[asn(key = 366)]
    IdTimeReferenceInformation(TimeReferenceInformation),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ReferenceTimeInformationReportProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ReferenceTimeInformationReportProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ReferenceTimeInformationReportProtocolIEs(
    pub Vec<ReferenceTimeInformationReportProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ReferenceTimeInformationReportingControlProtocolIEsEntryValue {
    #[asn(key = 365)]
    IdReportingRequestType(ReportingRequestType),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ReferenceTimeInformationReportingControlProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ReferenceTimeInformationReportingControlProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ReferenceTimeInformationReportingControlProtocolIEs(
    pub Vec<ReferenceTimeInformationReportingControlProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated149(pub u8);
impl Enumerated149 {
    const MM: u8 = 0u8;
    const CM: u8 = 1u8;
    const DM: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct Integer150(pub i32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct Integer151(pub i32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct Integer152(pub i16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelativeCartesianLocationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelativeCartesianLocationIeExtensions(
    pub Vec<RelativeCartesianLocationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated153(pub u8);
impl Enumerated153 {
    const ZERODOT03: u8 = 0u8;
    const ZERODOT3: u8 = 1u8;
    const THREE: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated154(pub u8);
impl Enumerated154 {
    const MM: u8 = 0u8;
    const CM: u8 = 1u8;
    const M: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Integer155(pub i16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Integer156(pub i16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Integer157(pub i16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelativeGeodeticLocationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RelativeGeodeticLocationIeExtensions(pub Vec<RelativeGeodeticLocationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16351")]
pub struct Integer158(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "8176")]
pub struct Integer159(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4088")]
pub struct Integer160(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2044")]
pub struct Integer161(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1022")]
pub struct Integer162(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct Integer163(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RelativePathDelaychoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ReportingRequestTypeIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ReportingRequestTypeIeExtensions(pub Vec<ReportingRequestTypeIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "500", extensible = true)]
pub struct Integer164(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated165(pub u8);
impl Enumerated165 {
    const PERIODIC: u8 = 0u8;
    const SEMI_PERSISTENT: u8 = 1u8;
    const APERIODIC: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RequestedSrsTransmissionCharacteristicsIeExtensionsEntryExtensionValue {
    #[asn(key = 431)]
    IdSrsFrequency(SrsFrequency),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestedSrsTransmissionCharacteristicsIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: RequestedSrsTransmissionCharacteristicsIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RequestedSrsTransmissionCharacteristicsIeExtensions(
    pub Vec<RequestedSrsTransmissionCharacteristicsIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 48)]
    IdResetType(ResetType),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ResetProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetProtocolIEs(pub Vec<ResetProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 81)]
    IdUeAssociatedLogicalF1ConnectionListResAck(UeAssociatedLogicalF1ConnectionListResAck),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ResetAcknowledgeProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeProtocolIEs(pub Vec<ResetAcknowledgeProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResourceCoordinationEutraCellInfoIeExtensionsEntryExtensionValue {
    #[asn(key = 233)]
    IdIgnorePrachConfiguration(IgnorePrachConfiguration),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceCoordinationEutraCellInfoIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ResourceCoordinationEutraCellInfoIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceCoordinationEutraCellInfoIeExtensions(
    pub Vec<ResourceCoordinationEutraCellInfoIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceCoordinationTransferInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceCoordinationTransferInformationIeExtensions(
    pub Vec<ResourceCoordinationTransferInformationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceSetTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3")]
pub struct Integer166(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32")]
pub struct Integer167(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceSetTypeAperiodicIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceSetTypeAperiodicIeExtensions(pub Vec<ResourceSetTypeAperiodicIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated168(pub u8);
impl Enumerated168 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceSetTypePeriodicIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceSetTypePeriodicIeExtensions(pub Vec<ResourceSetTypePeriodicIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated169(pub u8);
impl Enumerated169 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceSetTypeSemiPersistentIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceSetTypeSemiPersistentIeExtensions(
    pub Vec<ResourceSetTypeSemiPersistentIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResourceStatusFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 345)]
    IdGNbcuMeasurementId(GnbcuMeasurementId),
    #[asn(key = 346)]
    IdGNbduMeasurementId(GnbduMeasurementId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceStatusFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ResourceStatusFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResourceStatusFailureProtocolIEs(pub Vec<ResourceStatusFailureProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResourceStatusRequestProtocolIEsEntryValue {
    #[asn(key = 349)]
    IdCellToReportList(CellToReportList),
    #[asn(key = 347)]
    IdRegistrationRequest(RegistrationRequest),
    #[asn(key = 348)]
    IdReportCharacteristics(ReportCharacteristics),
    #[asn(key = 352)]
    IdReportingPeriodicity(ReportingPeriodicity),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 345)]
    IdGNbcuMeasurementId(GnbcuMeasurementId),
    #[asn(key = 346)]
    IdGNbduMeasurementId(GnbduMeasurementId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceStatusRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ResourceStatusRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResourceStatusRequestProtocolIEs(pub Vec<ResourceStatusRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResourceStatusResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 345)]
    IdGNbcuMeasurementId(GnbcuMeasurementId),
    #[asn(key = 346)]
    IdGNbduMeasurementId(GnbduMeasurementId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceStatusResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ResourceStatusResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResourceStatusResponseProtocolIEs(pub Vec<ResourceStatusResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ResourceStatusUpdateProtocolIEsEntryValue {
    #[asn(key = 350)]
    IdCellMeasurementResultList(CellMeasurementResultList),
    #[asn(key = 351)]
    IdHardwareLoadIndicator(HardwareLoadIndicator),
    #[asn(key = 353)]
    IdTnlCapacityIndicator(TnlCapacityIndicator),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
    #[asn(key = 345)]
    IdGNbcuMeasurementId(GnbcuMeasurementId),
    #[asn(key = 346)]
    IdGNbduMeasurementId(GnbduMeasurementId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceStatusUpdateProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ResourceStatusUpdateProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResourceStatusUpdateProtocolIEs(pub Vec<ResourceStatusUpdateProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated170(pub u8);
impl Enumerated170 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypeAperiodicIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceTypeAperiodicIeExtensions(pub Vec<ResourceTypeAperiodicIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32")]
pub struct Integer171(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypeAperiodicPosIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceTypeAperiodicPosIeExtensions(pub Vec<ResourceTypeAperiodicPosIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "16")]
pub struct Enumerated172(pub u8);
impl Enumerated172 {
    const SLOT1: u8 = 0u8;
    const SLOT2: u8 = 1u8;
    const SLOT4: u8 = 2u8;
    const SLOT5: u8 = 3u8;
    const SLOT8: u8 = 4u8;
    const SLOT10: u8 = 5u8;
    const SLOT16: u8 = 6u8;
    const SLOT20: u8 = 7u8;
    const SLOT32: u8 = 8u8;
    const SLOT40: u8 = 9u8;
    const SLOT64: u8 = 10u8;
    const SLOT80: u8 = 11u8;
    const SLOT160: u8 = 12u8;
    const SLOT320: u8 = 13u8;
    const SLOT640: u8 = 14u8;
    const SLOT1280: u8 = 15u8;
    const SLOT2560: u8 = 16u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2559", extensible = true)]
pub struct Integer173(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypePeriodicIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceTypePeriodicIeExtensions(pub Vec<ResourceTypePeriodicIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "20")]
pub struct Enumerated174(pub u8);
impl Enumerated174 {
    const SLOT1: u8 = 0u8;
    const SLOT2: u8 = 1u8;
    const SLOT4: u8 = 2u8;
    const SLOT5: u8 = 3u8;
    const SLOT8: u8 = 4u8;
    const SLOT10: u8 = 5u8;
    const SLOT16: u8 = 6u8;
    const SLOT20: u8 = 7u8;
    const SLOT32: u8 = 8u8;
    const SLOT40: u8 = 9u8;
    const SLOT64: u8 = 10u8;
    const SLOT80: u8 = 11u8;
    const SLOT160: u8 = 12u8;
    const SLOT320: u8 = 13u8;
    const SLOT640: u8 = 14u8;
    const SLOT1280: u8 = 15u8;
    const SLOT2560: u8 = 16u8;
    const SLOT5120: u8 = 17u8;
    const SLOT10240: u8 = 18u8;
    const SLOT40960: u8 = 19u8;
    const SLOT81920: u8 = 20u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "81919", extensible = true)]
pub struct Integer175(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypePeriodicPosIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceTypePeriodicPosIeExtensions(pub Vec<ResourceTypePeriodicPosIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypePoschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "16")]
pub struct Enumerated176(pub u8);
impl Enumerated176 {
    const SLOT1: u8 = 0u8;
    const SLOT2: u8 = 1u8;
    const SLOT4: u8 = 2u8;
    const SLOT5: u8 = 3u8;
    const SLOT8: u8 = 4u8;
    const SLOT10: u8 = 5u8;
    const SLOT16: u8 = 6u8;
    const SLOT20: u8 = 7u8;
    const SLOT32: u8 = 8u8;
    const SLOT40: u8 = 9u8;
    const SLOT64: u8 = 10u8;
    const SLOT80: u8 = 11u8;
    const SLOT160: u8 = 12u8;
    const SLOT320: u8 = 13u8;
    const SLOT640: u8 = 14u8;
    const SLOT1280: u8 = 15u8;
    const SLOT2560: u8 = 16u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2559", extensible = true)]
pub struct Integer177(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypeSemiPersistentIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceTypeSemiPersistentIeExtensions(
    pub Vec<ResourceTypeSemiPersistentIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "20")]
pub struct Enumerated178(pub u8);
impl Enumerated178 {
    const SLOT1: u8 = 0u8;
    const SLOT2: u8 = 1u8;
    const SLOT4: u8 = 2u8;
    const SLOT5: u8 = 3u8;
    const SLOT8: u8 = 4u8;
    const SLOT10: u8 = 5u8;
    const SLOT16: u8 = 6u8;
    const SLOT20: u8 = 7u8;
    const SLOT32: u8 = 8u8;
    const SLOT40: u8 = 9u8;
    const SLOT64: u8 = 10u8;
    const SLOT80: u8 = 11u8;
    const SLOT160: u8 = 12u8;
    const SLOT320: u8 = 13u8;
    const SLOT640: u8 = 14u8;
    const SLOT1280: u8 = 15u8;
    const SLOT2560: u8 = 16u8;
    const SLOT5120: u8 = 17u8;
    const SLOT10240: u8 = 18u8;
    const SLOT40960: u8 = 19u8;
    const SLOT81920: u8 = 20u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "81919", extensible = true)]
pub struct Integer179(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResourceTypeSemiPersistentPosIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResourceTypeSemiPersistentPosIeExtensions(
    pub Vec<ResourceTypeSemiPersistentPosIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2199", extensible = true)]
pub struct Integer180(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated181(pub u8);
impl Enumerated181 {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ60: u8 = 2u8;
    const K_HZ120: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "275", extensible = true)]
pub struct Integer182(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScsSpecificCarrierIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ScsSpecificCarrierIeExtensions(pub Vec<ScsSpecificCarrierIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellFailedtoSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SCellFailedtoSetupItemIeExtensions(pub Vec<SCellFailedtoSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellFailedtoSetupListEntryValue {
    #[asn(key = 84)]
    IdSCellFailedtoSetupItem(SCellFailedtoSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellFailedtoSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SCellFailedtoSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellFailedtoSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SCellFailedtoSetupModItemIeExtensions(
    pub Vec<SCellFailedtoSetupModItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellFailedtoSetupModListEntryValue {
    #[asn(key = 86)]
    IdSCellFailedtoSetupModItem(SCellFailedtoSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellFailedtoSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SCellFailedtoSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellToBeRemovedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SCellToBeRemovedItemIeExtensions(pub Vec<SCellToBeRemovedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellToBeRemovedListEntryValue {
    #[asn(key = 51)]
    IdSCellToBeRemovedItem(SCellToBeRemovedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellToBeRemovedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SCellToBeRemovedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellToBeSetupItemIeExtensionsEntryExtensionValue {
    #[asn(key = 182)]
    IdServingCellMo(ServingCellMo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellToBeSetupItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SCellToBeSetupItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SCellToBeSetupItemIeExtensions(pub Vec<SCellToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellToBeSetupListEntryValue {
    #[asn(key = 53)]
    IdSCellToBeSetupItem(SCellToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SCellToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellToBeSetupModItemIeExtensionsEntryExtensionValue {
    #[asn(key = 182)]
    IdServingCellMo(ServingCellMo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellToBeSetupModItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SCellToBeSetupModItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SCellToBeSetupModItemIeExtensions(pub Vec<SCellToBeSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SCellToBeSetupModListEntryValue {
    #[asn(key = 55)]
    IdSCellToBeSetupModItem(SCellToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SCellToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SCellToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct BitString183(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SfnOffsetIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SfnOffsetIeExtensions(pub Vec<SfnOffsetIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SItypeItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SItypeItemIeExtensions(pub Vec<SItypeItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsFailedToBeModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsFailedToBeModifiedItemIeExtensions(
    pub Vec<SldrBsFailedToBeModifiedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsFailedToBeModifiedListEntryValue {
    #[asn(key = 313)]
    IdSldrBsFailedToBeModifiedItem(SldrBsFailedToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsFailedToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsFailedToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsFailedToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsFailedToBeSetupItemIeExtensions(
    pub Vec<SldrBsFailedToBeSetupItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsFailedToBeSetupListEntryValue {
    #[asn(key = 315)]
    IdSldrBsFailedToBeSetupItem(SldrBsFailedToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsFailedToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsFailedToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsFailedToBeSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsFailedToBeSetupModItemIeExtensions(
    pub Vec<SldrBsFailedToBeSetupModItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsFailedToBeSetupModListEntryValue {
    #[asn(key = 336)]
    IdSldrBsFailedToBeSetupModItem(SldrBsFailedToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsFailedToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsFailedToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsModifiedItemIeExtensions(pub Vec<SldrBsModifiedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsModifiedListEntryValue {
    #[asn(key = 317)]
    IdSldrBsModifiedItem(SldrBsModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsModifiedConfItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsModifiedConfItemIeExtensions(pub Vec<SldrBsModifiedConfItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsModifiedConfListEntryValue {
    #[asn(key = 338)]
    IdSldrBsModifiedConfItem(SldrBsModifiedConfItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsModifiedConfListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsModifiedConfListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsRequiredToBeModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsRequiredToBeModifiedItemIeExtensions(
    pub Vec<SldrBsRequiredToBeModifiedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsRequiredToBeModifiedListEntryValue {
    #[asn(key = 319)]
    IdSldrBsRequiredToBeModifiedItem(SldrBsRequiredToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsRequiredToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsRequiredToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsRequiredToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsRequiredToBeReleasedItemIeExtensions(
    pub Vec<SldrBsRequiredToBeReleasedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsRequiredToBeReleasedListEntryValue {
    #[asn(key = 321)]
    IdSldrBsRequiredToBeReleasedItem(SldrBsRequiredToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsRequiredToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsRequiredToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsSetupItemIeExtensions(pub Vec<SldrBsSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsSetupListEntryValue {
    #[asn(key = 323)]
    IdSldrBsSetupItem(SldrBsSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsSetupModItemIeExtensions(pub Vec<SldrBsSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsSetupModListEntryValue {
    #[asn(key = 335)]
    IdSldrBsSetupModItem(SldrBsSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsToBeModifiedItemIeExtensions(pub Vec<SldrBsToBeModifiedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsToBeModifiedListEntryValue {
    #[asn(key = 325)]
    IdSldrBsToBeModifiedItem(SldrBsToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsToBeReleasedItemIeExtensions(pub Vec<SldrBsToBeReleasedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsToBeReleasedListEntryValue {
    #[asn(key = 327)]
    IdSldrBsToBeReleasedItem(SldrBsToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsToBeSetupItemIeExtensions(pub Vec<SldrBsToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsToBeSetupListEntryValue {
    #[asn(key = 329)]
    IdSldrBsToBeSetupItem(SldrBsToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SldrBsToBeSetupModItemIeExtensions(pub Vec<SldrBsToBeSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SldrBsToBeSetupModListEntryValue {
    #[asn(key = 331)]
    IdSldrBsToBeSetupModItem(SldrBsToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SldrBsToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SldrBsToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct OctetString184(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct OctetString185(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SnssaiIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SnssaiIeExtensions(pub Vec<SnssaiIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SnssaiItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SnssaiItemIeExtensions(pub Vec<SnssaiItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer186(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer187(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SnssaiAvailableCapacityItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SnssaiAvailableCapacityItemIeExtensions(
    pub Vec<SnssaiAvailableCapacityItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsFailedToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsFailedToBeSetupItemIeExtensions(pub Vec<SrBsFailedToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsFailedToBeSetupListEntryValue {
    #[asn(key = 65)]
    IdSrBsFailedToBeSetupItem(SrBsFailedToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsFailedToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsFailedToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsFailedToBeSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsFailedToBeSetupModItemIeExtensions(
    pub Vec<SrBsFailedToBeSetupModItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsFailedToBeSetupModListEntryValue {
    #[asn(key = 67)]
    IdSrBsFailedToBeSetupModItem(SrBsFailedToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsFailedToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsFailedToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsModifiedItemIeExtensions(pub Vec<SrBsModifiedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsModifiedListEntryValue {
    #[asn(key = 207)]
    IdSrBsModifiedItem(SrBsModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsRequiredToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsRequiredToBeReleasedItemIeExtensions(
    pub Vec<SrBsRequiredToBeReleasedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsRequiredToBeReleasedListEntryValue {
    #[asn(key = 69)]
    IdSrBsRequiredToBeReleasedItem(SrBsRequiredToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsRequiredToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsRequiredToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsSetupItemIeExtensions(pub Vec<SrBsSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsSetupListEntryValue {
    #[asn(key = 203)]
    IdSrBsSetupItem(SrBsSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsSetupModItemIeExtensions(pub Vec<SrBsSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsSetupModListEntryValue {
    #[asn(key = 205)]
    IdSrBsSetupModItem(SrBsSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsToBeReleasedItemIeExtensions(pub Vec<SrBsToBeReleasedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsToBeReleasedListEntryValue {
    #[asn(key = 71)]
    IdSrBsToBeReleasedItem(SrBsToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsToBeSetupItemIeExtensionsEntryExtensionValue {
    #[asn(key = 372)]
    IdAdditionalDuplicationIndication(AdditionalDuplicationIndication),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsToBeSetupItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SrBsToBeSetupItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsToBeSetupItemIeExtensions(pub Vec<SrBsToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsToBeSetupListEntryValue {
    #[asn(key = 73)]
    IdSrBsToBeSetupItem(SrBsToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsToBeSetupModItemIeExtensionsEntryExtensionValue {
    #[asn(key = 372)]
    IdAdditionalDuplicationIndication(AdditionalDuplicationIndication),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsToBeSetupModItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SrBsToBeSetupModItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrBsToBeSetupModItemIeExtensions(pub Vec<SrBsToBeSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SrBsToBeSetupModListEntryValue {
    #[asn(key = 75)]
    IdSrBsToBeSetupModItem(SrBsToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrBsToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SrBsToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer188(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsCarrierListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsCarrierListItemIeExtensions(pub Vec<SrsCarrierListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsConfigIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsConfigIeExtensions(pub Vec<SrsConfigIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsConfigurationIeExtensions(pub Vec<SrsConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Enumerated189(pub u8);
impl Enumerated189 {
    const PORT1: u8 = 0u8;
    const PORTS2: u8 = 1u8;
    const PORTS4: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "13")]
pub struct Integer190(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Enumerated191(pub u8);
impl Enumerated191 {
    const N1: u8 = 0u8;
    const N2: u8 = 1u8;
    const N4: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Enumerated192(pub u8);
impl Enumerated192 {
    const N1: u8 = 0u8;
    const N2: u8 = 1u8;
    const N4: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "67")]
pub struct Integer193(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "268")]
pub struct Integer194(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer195(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct Integer196(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct Integer197(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Enumerated198(pub u8);
impl Enumerated198 {
    const NEITHER: u8 = 0u8;
    const GROUP_HOPPING: u8 = 1u8;
    const SEQUENCE_HOPPING: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct Integer199(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsResourceIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsResourceIeExtensions(pub Vec<SrsResourceIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsResourceSetIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsResourceSetIeExtensions(pub Vec<SrsResourceSetIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct Integer200(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsResourceSetItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsResourceSetItemIeExtensions(pub Vec<SrsResourceSetItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsResourceTriggerIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SrsResourceTriggerIeExtensions(pub Vec<SrsResourceTriggerIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SrsTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbIeExtensions(pub Vec<SsbIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct BitString201(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct BitString202(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct BitString203(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbPositionsInBurstchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer204(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct Enumerated205(pub u8);
impl Enumerated205 {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ60: u8 = 2u8;
    const K_HZ120: u8 = 3u8;
    const K_HZ240: u8 = 4u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-60", ub = "50")]
pub struct Integer206(pub i8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct Enumerated207(pub u8);
impl Enumerated207 {
    const MS5: u8 = 0u8;
    const MS10: u8 = 1u8;
    const MS20: u8 = 2u8;
    const MS40: u8 = 3u8;
    const MS80: u8 = 4u8;
    const MS160: u8 = 5u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct Integer208(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct Integer209(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbTfConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbTfConfigurationIeExtensions(pub Vec<SsbTfConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct BitString210(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct BitString211(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct BitString212(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbTransmissionBitmapchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer213(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer214(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbAreaCapacityValueItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbAreaCapacityValueItemIeExtensions(pub Vec<SsbAreaCapacityValueItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer215(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer216(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer217(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer218(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer219(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer220(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer221(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer222(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer223(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbAreaRadioResourceStatusItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbAreaRadioResourceStatusItemIeExtensions(
    pub Vec<SsbAreaRadioResourceStatusItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbInformationIeExtensions(pub Vec<SsbInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbInformationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbInformationItemIeExtensions(pub Vec<SsbInformationItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct Integer224(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SsbToReportItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SsbToReportItemIeExtensions(pub Vec<SsbToReportItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer225(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SulInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 354)]
    IdCarrierList(NrCarrierList),
    #[asn(key = 356)]
    IdFrequencyShift7p5khz(FrequencyShift7p5khz),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SulInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SulInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SulInformationIeExtensions(pub Vec<SulInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-3841", ub = "3841", extensible = true)]
pub struct Integer226(pub i16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "246", extensible = true)]
pub struct Integer227(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SearchWindowInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SearchWindowInformationIeExtensions(pub Vec<SearchWindowInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SemipersistentSrsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SemipersistentSrsIeExtensions(pub Vec<SemipersistentSrsIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString228(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedCellInformationIeExtensionsEntryExtensionValue {
    #[asn(key = 251)]
    IdAggressorgNbSetId(AggressorgNbSetId),
    #[asn(key = 223)]
    IdBplmnIdInfoList(BplmnIdInfoList),
    #[asn(key = 201)]
    IdCellDirection(CellDirection),
    #[asn(key = 232)]
    IdCellType(CellType),
    #[asn(key = 425)]
    IdConfiguredTacIndication(ConfiguredTacIndication),
    #[asn(key = 196)]
    IdExtendedServedPlmNsList(ExtendedServedPlmNsList),
    #[asn(key = 290)]
    IdIabInfoIabDu(IabInfoIabDu),
    #[asn(key = 383)]
    IdNpnBroadcastInformation(NpnBroadcastInformation),
    #[asn(key = 358)]
    IdNrprachConfig(NrprachConfig),
    #[asn(key = 139)]
    IdRanac(Ranac),
    #[asn(key = 429)]
    IdSfnOffset(SfnOffset),
    #[asn(key = 357)]
    IdSsbPositionsInBurst(SsbPositionsInBurst),
    #[asn(key = 252)]
    IdVictimgNbSetId(VictimgNbSetId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellInformationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ServedCellInformationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedCellInformationIeExtensions(pub Vec<ServedCellInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellsToAddItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedCellsToAddItemIeExtensions(pub Vec<ServedCellsToAddItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedCellsToAddListEntryValue {
    #[asn(key = 57)]
    IdServedCellsToAddItem(ServedCellsToAddItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellsToAddListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ServedCellsToAddListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellsToDeleteItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedCellsToDeleteItemIeExtensions(pub Vec<ServedCellsToDeleteItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedCellsToDeleteListEntryValue {
    #[asn(key = 59)]
    IdServedCellsToDeleteItem(ServedCellsToDeleteItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellsToDeleteListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ServedCellsToDeleteListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellsToModifyItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedCellsToModifyItemIeExtensions(pub Vec<ServedCellsToModifyItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedCellsToModifyListEntryValue {
    #[asn(key = 61)]
    IdServedCellsToModifyItem(ServedCellsToModifyItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedCellsToModifyListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ServedCellsToModifyListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedEutraCellsInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedEutraCellsInformationIeExtensions(
    pub Vec<ServedEutraCellsInformationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedPlmNsItemIeExtensionsEntryExtensionValue {
    #[asn(key = 390)]
    IdExtendedTaiSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 384)]
    IdNpnSupportInfo(NpnSupportInfo),
    #[asn(key = 131)]
    IdTaiSliceSupportList(SliceSupportList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedPlmNsItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ServedPlmNsItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedPlmNsItemIeExtensions(pub Vec<ServedPlmNsItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated229(pub u8);
impl Enumerated229 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServiceStatusIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServiceStatusIeExtensions(pub Vec<ServiceStatusIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "2", ub = "32", extensible = true)]
pub struct Integer230(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString231(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "31", extensible = true)]
pub struct Integer232(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SibtypetobeupdatedListItemIeExtensionsEntryExtensionValue {
    #[asn(key = 240)]
    IdAreaScope(AreaScope),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SibtypetobeupdatedListItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SibtypetobeupdatedListItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SibtypetobeupdatedListItemIeExtensions(
    pub Vec<SibtypetobeupdatedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceAvailableCapacityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceAvailableCapacityIeExtensions(pub Vec<SliceAvailableCapacityIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceAvailableCapacityItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceAvailableCapacityItemIeExtensions(
    pub Vec<SliceAvailableCapacityItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceSupportItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceSupportItemIeExtensions(pub Vec<SliceSupportItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceToReportItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceToReportItemIeExtensions(pub Vec<SliceToReportItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "5119", extensible = true)]
pub struct Integer233(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SlotConfigurationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SlotConfigurationItemIeExtensions(pub Vec<SlotConfigurationItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SpatialDirectionInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SpatialDirectionInformationIeExtensions(
    pub Vec<SpatialDirectionInformationIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SpatialRelationInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SpatialRelationInfoIeExtensions(pub Vec<SpatialRelationInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SpatialRelationPoschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SpatialRelationforResourceIdItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SpatialRelationforResourceIdItemIeExtensions(
    pub Vec<SpatialRelationforResourceIdItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 32)]
    IdBapMappingConfiguration(BapMappingConfigurationAcknowledge),
    #[asn(key = 52)]
    IdECidMeasurementInitiation(ECidMeasurementInitiationResponse),
    #[asn(key = 26)]
    IdF1Removal(F1RemovalResponse),
    #[asn(key = 1)]
    IdF1Setup(F1SetupResponse),
    #[asn(key = 33)]
    IdGnbduResourceConfiguration(GnbduResourceConfigurationAcknowledge),
    #[asn(key = 16)]
    IdGnbduResourceCoordination(GnbduResourceCoordinationResponse),
    #[asn(key = 34)]
    IdIabtnlAddressAllocation(IabtnlAddressResponse),
    #[asn(key = 35)]
    IdIabupConfigurationUpdate(IabupConfigurationUpdateResponse),
    #[asn(key = 21)]
    IdPwsCancel(PwsCancelResponse),
    #[asn(key = 50)]
    IdPositioningActivation(PositioningActivationResponse),
    #[asn(key = 49)]
    IdPositioningInformationExchange(PositioningInformationResponse),
    #[asn(key = 41)]
    IdPositioningMeasurementExchange(PositioningMeasurementResponse),
    #[asn(key = 0)]
    IdReset(ResetAcknowledge),
    #[asn(key = 48)]
    IdTrpInformationExchange(TrpInformationResponse),
    #[asn(key = 7)]
    IdUeContextModification(UeContextModificationResponse),
    #[asn(key = 8)]
    IdUeContextModificationRequired(UeContextModificationConfirm),
    #[asn(key = 6)]
    IdUeContextRelease(UeContextReleaseComplete),
    #[asn(key = 5)]
    IdUeContextSetup(UeContextSetupResponse),
    #[asn(key = 20)]
    IdWriteReplaceWarning(WriteReplaceWarningResponse),
    #[asn(key = 4)]
    IdGNbcuConfigurationUpdate(GnbcuConfigurationUpdateAcknowledge),
    #[asn(key = 3)]
    IdGNbduConfigurationUpdate(GnbduConfigurationUpdateAcknowledge),
    #[asn(key = 36)]
    IdResourceStatusReportingInitiation(ResourceStatusResponse),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1024", extensible = true)]
pub struct Integer234(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedSulFreqBandItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SupportedSulFreqBandItemIeExtensions(pub Vec<SupportedSulFreqBandItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null235;

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null236;

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SymbolAllocInSlotchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SystemInformationDeliveryCommandProtocolIEsEntryValue {
    #[asn(key = 156)]
    IdConfirmedUeid(GnbDuUeF1apId),
    #[asn(key = 111)]
    IdNrcgi(Nrcgi),
    #[asn(key = 116)]
    IdSItypeList(SItypeList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SystemInformationDeliveryCommandProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SystemInformationDeliveryCommandProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SystemInformationDeliveryCommandProtocolIEs(
    pub Vec<SystemInformationDeliveryCommandProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TddInfoIeExtensionsEntryExtensionValue {
    #[asn(key = 354)]
    IdCarrierList(NrCarrierList),
    #[asn(key = 256)]
    IdIntendedTddDlUlConfig(IntendedTddDlUlConfig),
    #[asn(key = 361)]
    IdTddUlDlConfigCommonNr(TddUlDlConfigCommonNr),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TddInfoIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: TddInfoIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TddInfoIeExtensions(pub Vec<TddInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16777216", extensible = true)]
pub struct Integer237(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100", extensible = true)]
pub struct Integer238(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16777216", extensible = true)]
pub struct Integer239(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100", extensible = true)]
pub struct Integer240(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TnlCapacityIndicatorIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TnlCapacityIndicatorIeExtensions(pub Vec<TnlCapacityIndicatorIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrpMeasurementRequestItemIeExtensionsEntryExtensionValue {
    #[asn(key = 111)]
    IdNrcgi(Nrcgi),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpMeasurementRequestItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: TrpMeasurementRequestItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpMeasurementRequestItemIeExtensions(
    pub Vec<TrpMeasurementRequestItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpInformationIeExtensions(pub Vec<TrpInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrpInformationFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TrpInformationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TrpInformationFailureProtocolIEs(pub Vec<TrpInformationFailureProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpInformationItemIeExtensions(pub Vec<TrpInformationItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrpInformationListTrpRespEntryValue {
    #[asn(key = 401)]
    IdTrpInformationItem(TrpInformationItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationListTrpRespEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TrpInformationListTrpRespEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrpInformationRequestProtocolIEsEntryValue {
    #[asn(key = 398)]
    IdTrpInformationTypeListTrpReq(TrpInformationTypeListTrpReq),
    #[asn(key = 410)]
    IdTrpList(TrpList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TrpInformationRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TrpInformationRequestProtocolIEs(pub Vec<TrpInformationRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrpInformationResponseProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 400)]
    IdTrpInformationListTrpResp(TrpInformationListTrpResp),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TrpInformationResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TrpInformationResponseProtocolIEs(pub Vec<TrpInformationResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TrpInformationTypeListTrpReqEntryValue {
    #[asn(key = 399)]
    IdTrpInformationTypeItem(TrpInformationTypeItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationTypeListTrpReqEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TrpInformationTypeListTrpReqEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer241(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpInformationTypeResponseItemchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpListItemIeExtensions(pub Vec<TrpListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpMeasurementQualityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpMeasurementQualityIeExtensions(pub Vec<TrpMeasurementQualityIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpMeasurementQualityItemchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpPositionDefinitionTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpPositionDirectIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpPositionDirectIeExtensions(pub Vec<TrpPositionDirectIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpPositionDirectAccuracychoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpPositionReferencedIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpPositionReferencedIeExtensions(pub Vec<TrpPositionReferencedIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrpReferencePointTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TscAssistanceInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TscAssistanceInformationIeExtensions(pub Vec<TscAssistanceInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TscTrafficCharacteristicsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TscTrafficCharacteristicsIeExtensions(
    pub Vec<TscTrafficCharacteristicsIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetCellListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetCellListItemIeExtensions(pub Vec<TargetCellListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TimeReferenceInformationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TimeReferenceInformationIeExtensions(pub Vec<TimeReferenceInformationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TimeStampIeExtensionEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TimeStampIeExtension(pub Vec<TimeStampIeExtensionEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct Integer242(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct Integer243(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct Integer244(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct Integer245(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TimeStampSlotIndexchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct Integer246(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated247(pub u8);
impl Enumerated247 {
    const M0DOT1: u8 = 0u8;
    const M1: u8 = 1u8;
    const M10: u8 = 2u8;
    const M30: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TimingMeasurementQualityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TimingMeasurementQualityIeExtensions(pub Vec<TimingMeasurementQualityIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationIeExtensionsEntryExtensionValue {
    #[asn(key = 380)]
    IdTraceCollectionEntityUri(UriAddress),
    #[asn(key = 381)]
    IdMdtConfiguration(MdtConfiguration),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceActivationIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: TraceActivationIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceActivationIeExtensions(pub Vec<TraceActivationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceStartProtocolIEsEntryValue {
    #[asn(key = 242)]
    IdTraceActivation(TraceActivation),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceStartProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TraceStartProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TraceStartProtocolIEs(pub Vec<TraceStartProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TrafficMappingInfochoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionBandwidthIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransmissionBandwidthIeExtensions(pub Vec<TransmissionBandwidthIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct Integer248(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct Integer249(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombn2 {
    pub comb_offset_n2: Integer248,
    pub cyclic_shift_n2: Integer249,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct Integer250(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "11")]
pub struct Integer251(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombn4 {
    pub comb_offset_n4: Integer250,
    pub cyclic_shift_n4: Integer251,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct Integer252(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct Integer253(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombPosn2 {
    pub comb_offset_n2: Integer252,
    pub cyclic_shift_n2: Integer253,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct Integer254(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "11")]
pub struct Integer255(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombPosn4 {
    pub comb_offset_n4: Integer254,
    pub cyclic_shift_n4: Integer255,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct Integer256(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "5")]
pub struct Integer257(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombPosn8 {
    pub comb_offset_n8: Integer256,
    pub cyclic_shift_n8: Integer257,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransmissionCombPoschoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransportLayerAddressInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransportLayerAddressInfoIeExtensions(
    pub Vec<TransportLayerAddressInfoIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransportUpLayerAddressInfoToAddItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransportUpLayerAddressInfoToAddItemIeExtensions(
    pub Vec<TransportUpLayerAddressInfoToAddItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TransportUpLayerAddressInfoToRemoveItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TransportUpLayerAddressInfoToRemoveItemIeExtensions(
    pub Vec<TransportUpLayerAddressInfoToRemoveItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UacAssistanceInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UacAssistanceInfoIeExtensions(pub Vec<UacAssistanceInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UacCategoryTypechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "32", ub = "63", extensible = true)]
pub struct Integer258(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BitString259(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UacOperatorDefinedIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UacOperatorDefinedIeExtensions(pub Vec<UacOperatorDefinedIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UacplmnItemIeExtensionsEntryExtensionValue {
    #[asn(key = 385)]
    IdNid(Nid),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UacplmnItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UacplmnItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UacplmnItemIeExtensions(pub Vec<UacplmnItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UacTypeItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UacTypeItemIeExtensions(pub Vec<UacTypeItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAssociatedLogicalF1ConnectionItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UeAssociatedLogicalF1ConnectionItemIeExtensions(
    pub Vec<UeAssociatedLogicalF1ConnectionItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeAssociatedLogicalF1ConnectionListResEntryValue {
    #[asn(key = 80)]
    IdUeAssociatedLogicalF1ConnectionItem(UeAssociatedLogicalF1ConnectionItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAssociatedLogicalF1ConnectionListResEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeAssociatedLogicalF1ConnectionListResEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeAssociatedLogicalF1ConnectionListResAckEntryValue {
    #[asn(key = 80)]
    IdUeAssociatedLogicalF1ConnectionItem(UeAssociatedLogicalF1ConnectionItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAssociatedLogicalF1ConnectionListResAckEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeAssociatedLogicalF1ConnectionListResAckEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationConfirmProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 19)]
    IdDrBsModifiedConfList(DrBsModifiedConfList),
    #[asn(key = 109)]
    IdExecuteDuplication(ExecuteDuplication),
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 49)]
    IdResourceCoordinationTransferContainer(ResourceCoordinationTransferContainer),
    #[asn(key = 195)]
    IdResourceCoordinationTransferInformation(ResourceCoordinationTransferInformation),
    #[asn(key = 337)]
    IdSldrBsModifiedConfList(SldrBsModifiedConfList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationConfirmProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationConfirmProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextModificationConfirmProtocolIEs(
    pub Vec<UeContextModificationConfirmProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 376)]
    IdRequestedTargetCellGlobalId(Nrcgi),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextModificationFailureProtocolIEs(
    pub Vec<UeContextModificationFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationRefuseProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationRefuseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationRefuseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextModificationRefuseProtocolIEs(
    pub Vec<UeContextModificationRefuseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationRequestProtocolIEsEntryValue {
    #[asn(key = 248)]
    IdAdditionalRrmPriorityIndex(AdditionalRrmPriorityIndex),
    #[asn(key = 263)]
    IdBhChannelsToBeModifiedList(BhChannelsToBeModifiedList),
    #[asn(key = 265)]
    IdBhChannelsToBeReleasedList(BhChannelsToBeReleasedList),
    #[asn(key = 267)]
    IdBhChannelsToBeSetupModList(BhChannelsToBeSetupModList),
    #[asn(key = 9)]
    IdCUtoDurrcInformation(CUtoDurrcInformation),
    #[asn(key = 374)]
    IdConditionalIntraDuMobilityInformation(ConditionalIntraDuMobilityInformation),
    #[asn(key = 31)]
    IdDrBsToBeModifiedList(DrBsToBeModifiedList),
    #[asn(key = 33)]
    IdDrBsToBeReleasedList(DrBsToBeReleasedList),
    #[asn(key = 37)]
    IdDrBsToBeSetupModList(DrBsToBeSetupModList),
    #[asn(key = 159)]
    IdDrxConfigurationIndicator(DrxConfigurationIndicator),
    #[asn(key = 38)]
    IdDrxCycle(DrxCycle),
    #[asn(key = 109)]
    IdExecuteDuplication(ExecuteDuplication),
    #[asn(key = 428)]
    IdF1cTransferPath(F1cTransferPath),
    #[asn(key = 94)]
    IdFullConfiguration(FullConfiguration),
    #[asn(key = 158)]
    IdGnbDuUeAmbrUl(BitRate),
    #[asn(key = 162)]
    IdGnbDuConfigurationQuery(GnbDuConfigurationQuery),
    #[asn(key = 97)]
    IdInactivityMonitoringRequest(InactivityMonitoringRequest),
    #[asn(key = 309)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 307)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 253)]
    IdLowerLayerPresenceStatusChange(LowerLayerPresenceStatusChange),
    #[asn(key = 308)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 215)]
    IdNeedforGap(NeedforGap),
    #[asn(key = 340)]
    IdPc5LinkAmbr(BitRate),
    #[asn(key = 108)]
    IdRatFrequencyPriorityInformation(RatFrequencyPriorityInformation),
    #[asn(key = 174)]
    IdRlcFailureIndication(RlcFailureIndication),
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 184)]
    IdRrcDeliveryStatusRequest(RrcDeliveryStatusRequest),
    #[asn(key = 87)]
    IdRrcReconfigurationCompleteIndicator(RrcReconfigurationCompleteIndicator),
    #[asn(key = 49)]
    IdResourceCoordinationTransferContainer(ResourceCoordinationTransferContainer),
    #[asn(key = 195)]
    IdResourceCoordinationTransferInformation(ResourceCoordinationTransferInformation),
    #[asn(key = 432)]
    IdScgIndicator(ScgIndicator),
    #[asn(key = 52)]
    IdSCellToBeRemovedList(SCellToBeRemovedList),
    #[asn(key = 56)]
    IdSCellToBeSetupModList(SCellToBeSetupModList),
    #[asn(key = 326)]
    IdSldrBsToBeModifiedList(SldrBsToBeModifiedList),
    #[asn(key = 328)]
    IdSldrBsToBeReleasedList(SldrBsToBeReleasedList),
    #[asn(key = 332)]
    IdSldrBsToBeSetupModList(SldrBsToBeSetupModList),
    #[asn(key = 72)]
    IdSrBsToBeReleasedList(SrBsToBeReleasedList),
    #[asn(key = 76)]
    IdSrBsToBeSetupModList(SrBsToBeSetupModList),
    #[asn(key = 107)]
    IdServCellIndex(ServCellIndex),
    #[asn(key = 182)]
    IdServingCellMo(ServingCellMo),
    #[asn(key = 63)]
    IdSpCellId(Nrcgi),
    #[asn(key = 96)]
    IdSpCellUlConfigured(CellUlConfigured),
    #[asn(key = 79)]
    IdTransmissionActionIndicator(TransmissionActionIndicator),
    #[asn(key = 175)]
    IdUplinkTxDirectCurrentListInformation(UplinkTxDirectCurrentListInformation),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextModificationRequestProtocolIEs(
    pub Vec<UeContextModificationRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationRequiredProtocolIEsEntryValue {
    #[asn(key = 277)]
    IdBhChannelsRequiredToBeReleasedList(BhChannelsRequiredToBeReleasedList),
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 23)]
    IdDrBsRequiredToBeModifiedList(DrBsRequiredToBeModifiedList),
    #[asn(key = 25)]
    IdDrBsRequiredToBeReleasedList(DrBsRequiredToBeReleasedList),
    #[asn(key = 39)]
    IdDUtoCurrcInformation(DUtoCurrcInformation),
    #[asn(key = 49)]
    IdResourceCoordinationTransferContainer(ResourceCoordinationTransferContainer),
    #[asn(key = 320)]
    IdSldrBsRequiredToBeModifiedList(SldrBsRequiredToBeModifiedList),
    #[asn(key = 322)]
    IdSldrBsRequiredToBeReleasedList(SldrBsRequiredToBeReleasedList),
    #[asn(key = 70)]
    IdSrBsRequiredToBeReleasedList(SrBsRequiredToBeReleasedList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 375)]
    IdTargetCellsToCancel(TargetCellList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationRequiredProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationRequiredProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextModificationRequiredProtocolIEs(
    pub Vec<UeContextModificationRequiredProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationResponseProtocolIEsEntryValue {
    #[asn(key = 198)]
    IdAssociatedSCellList(AssociatedSCellList),
    #[asn(key = 269)]
    IdBhChannelsFailedToBeModifiedList(BhChannelsFailedToBeModifiedList),
    #[asn(key = 271)]
    IdBhChannelsFailedToBeSetupModList(BhChannelsFailedToBeSetupModList),
    #[asn(key = 273)]
    IdBhChannelsModifiedList(BhChannelsModifiedList),
    #[asn(key = 275)]
    IdBhChannelsSetupModList(BhChannelsSetupModList),
    #[asn(key = 95)]
    IdCRnti(CRnti),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 13)]
    IdDrBsFailedToBeModifiedList(DrBsFailedToBeModifiedList),
    #[asn(key = 17)]
    IdDrBsFailedToBeSetupModList(DrBsFailedToBeSetupModList),
    #[asn(key = 21)]
    IdDrBsModifiedList(DrBsModifiedList),
    #[asn(key = 29)]
    IdDrBsSetupModList(DrBsSetupModList),
    #[asn(key = 39)]
    IdDUtoCurrcInformation(DUtoCurrcInformation),
    #[asn(key = 94)]
    IdFullConfiguration(FullConfiguration),
    #[asn(key = 98)]
    IdInactivityMonitoringResponse(InactivityMonitoringResponse),
    #[asn(key = 49)]
    IdResourceCoordinationTransferContainer(ResourceCoordinationTransferContainer),
    #[asn(key = 85)]
    IdSCellFailedtoSetupModList(SCellFailedtoSetupModList),
    #[asn(key = 314)]
    IdSldrBsFailedToBeModifiedList(SldrBsFailedToBeModifiedList),
    #[asn(key = 334)]
    IdSldrBsFailedToBeSetupModList(SldrBsFailedToBeSetupModList),
    #[asn(key = 318)]
    IdSldrBsModifiedList(SldrBsModifiedList),
    #[asn(key = 333)]
    IdSldrBsSetupModList(SldrBsSetupModList),
    #[asn(key = 68)]
    IdSrBsFailedToBeSetupModList(SrBsFailedToBeSetupModList),
    #[asn(key = 206)]
    IdSrBsModifiedList(SrBsModifiedList),
    #[asn(key = 204)]
    IdSrBsSetupModList(SrBsSetupModList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 376)]
    IdRequestedTargetCellGlobalId(Nrcgi),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextModificationResponseProtocolIEs(
    pub Vec<UeContextModificationResponseProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseCommandProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 109)]
    IdExecuteDuplication(ExecuteDuplication),
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 184)]
    IdRrcDeliveryStatusRequest(RrcDeliveryStatusRequest),
    #[asn(key = 64)]
    IdSrbid(Srbid),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 47)]
    IdOldgNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 375)]
    IdTargetCellsToCancel(TargetCellList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseCommandProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseCommandProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextReleaseCommandProtocolIEs(pub Vec<UeContextReleaseCommandProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseCompleteProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseCompleteProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseCompleteProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextReleaseCompleteProtocolIEs(pub Vec<UeContextReleaseCompleteProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseRequestProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 375)]
    IdTargetCellsToCancel(TargetCellList),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextReleaseRequestProtocolIEs(pub Vec<UeContextReleaseRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSetupFailureProtocolIEsEntryValue {
    #[asn(key = 0)]
    IdCause(Cause),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 92)]
    IdPotentialSpCellList(PotentialSpCellList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 376)]
    IdRequestedTargetCellGlobalId(Nrcgi),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSetupFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSetupFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextSetupFailureProtocolIEs(pub Vec<UeContextSetupFailureProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSetupRequestProtocolIEsEntryValue {
    #[asn(key = 248)]
    IdAdditionalRrmPriorityIndex(AdditionalRrmPriorityIndex),
    #[asn(key = 258)]
    IdBhChannelsToBeSetupList(BhChannelsToBeSetupList),
    #[asn(key = 9)]
    IdCUtoDurrcInformation(CUtoDurrcInformation),
    #[asn(key = 90)]
    IdCandidateSpCellList(CandidateSpCellList),
    #[asn(key = 373)]
    IdConditionalInterDuMobilityInformation(ConditionalInterDuMobilityInformation),
    #[asn(key = 282)]
    IdConfiguredBapAddress(BapAddress),
    #[asn(key = 35)]
    IdDrBsToBeSetupList(DrBsToBeSetupList),
    #[asn(key = 38)]
    IdDrxCycle(DrxCycle),
    #[asn(key = 428)]
    IdF1cTransferPath(F1cTransferPath),
    #[asn(key = 158)]
    IdGnbDuUeAmbrUl(BitRate),
    #[asn(key = 97)]
    IdInactivityMonitoringRequest(InactivityMonitoringRequest),
    #[asn(key = 309)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 307)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 377)]
    IdManagementBasedMdtplmnList(MdtplmnList),
    #[asn(key = 126)]
    IdMaskedImeisv(MaskedImeisv),
    #[asn(key = 308)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 340)]
    IdPc5LinkAmbr(BitRate),
    #[asn(key = 226)]
    IdRanueid(Ranueid),
    #[asn(key = 108)]
    IdRatFrequencyPriorityInformation(RatFrequencyPriorityInformation),
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 184)]
    IdRrcDeliveryStatusRequest(RrcDeliveryStatusRequest),
    #[asn(key = 49)]
    IdResourceCoordinationTransferContainer(ResourceCoordinationTransferContainer),
    #[asn(key = 195)]
    IdResourceCoordinationTransferInformation(ResourceCoordinationTransferInformation),
    #[asn(key = 54)]
    IdSCellToBeSetupList(SCellToBeSetupList),
    #[asn(key = 330)]
    IdSldrBsToBeSetupList(SldrBsToBeSetupList),
    #[asn(key = 74)]
    IdSrBsToBeSetupList(SrBsToBeSetupList),
    #[asn(key = 107)]
    IdServCellIndex(ServCellIndex),
    #[asn(key = 182)]
    IdServingCellMo(ServingCellMo),
    #[asn(key = 382)]
    IdServingNid(Nid),
    #[asn(key = 165)]
    IdServingPlmn(PlmnIdentity),
    #[asn(key = 63)]
    IdSpCellId(Nrcgi),
    #[asn(key = 96)]
    IdSpCellUlConfigured(CellUlConfigured),
    #[asn(key = 242)]
    IdTraceActivation(TraceActivation),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 217)]
    IdNewGNbCuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSetupRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSetupRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextSetupRequestProtocolIEs(pub Vec<UeContextSetupRequestProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSetupResponseProtocolIEsEntryValue {
    #[asn(key = 279)]
    IdBhChannelsFailedToBeSetupList(BhChannelsFailedToBeSetupList),
    #[asn(key = 260)]
    IdBhChannelsSetupList(BhChannelsSetupList),
    #[asn(key = 95)]
    IdCRnti(CRnti),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 15)]
    IdDrBsFailedToBeSetupList(DrBsFailedToBeSetupList),
    #[asn(key = 27)]
    IdDrBsSetupList(DrBsSetupList),
    #[asn(key = 39)]
    IdDUtoCurrcInformation(DUtoCurrcInformation),
    #[asn(key = 94)]
    IdFullConfiguration(FullConfiguration),
    #[asn(key = 98)]
    IdInactivityMonitoringResponse(InactivityMonitoringResponse),
    #[asn(key = 49)]
    IdResourceCoordinationTransferContainer(ResourceCoordinationTransferContainer),
    #[asn(key = 83)]
    IdSCellFailedtoSetupList(SCellFailedtoSetupList),
    #[asn(key = 316)]
    IdSldrBsFailedToBeSetupList(SldrBsFailedToBeSetupList),
    #[asn(key = 324)]
    IdSldrBsSetupList(SldrBsSetupList),
    #[asn(key = 66)]
    IdSrBsFailedToBeSetupList(SrBsFailedToBeSetupList),
    #[asn(key = 202)]
    IdSrBsSetupList(SrBsSetupList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 376)]
    IdRequestedTargetCellGlobalId(Nrcgi),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSetupResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSetupResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeContextSetupResponseProtocolIEs(pub Vec<UeContextSetupResponseProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BitString260(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeIdentityIndexValuechoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeInactivityNotificationProtocolIEsEntryValue {
    #[asn(key = 100)]
    IdDrbActivityList(DrbActivityList),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeInactivityNotificationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeInactivityNotificationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UeInactivityNotificationProtocolIEs(pub Vec<UeInactivityNotificationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct Integer261(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1799")]
pub struct Integer262(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlAoAIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlAoAIeExtensions(pub Vec<UlAoAIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlBhNonUpTrafficMappingIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlBhNonUpTrafficMappingIeExtensions(pub Vec<UlBhNonUpTrafficMappingIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlBhNonUpTrafficMappingItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlBhNonUpTrafficMappingItemIeExtensions(
    pub Vec<UlBhNonUpTrafficMappingItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlRtoaMeasurementIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlRtoaMeasurementIeExtensions(pub Vec<UlRtoaMeasurementIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1970049")]
pub struct Integer263(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "985025")]
pub struct Integer264(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "492513")]
pub struct Integer265(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "246257")]
pub struct Integer266(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "123129")]
pub struct Integer267(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "61565")]
pub struct Integer268(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlRtoaMeasurementItemchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UlUpTnlAddressToUpdateListEntryValue {
    #[asn(key = 303)]
    IdUlUpTnlAddressToUpdateListItem(UlUpTnlAddressToUpdateListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlUpTnlAddressToUpdateListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UlUpTnlAddressToUpdateListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlUpTnlAddressToUpdateListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlUpTnlAddressToUpdateListItemIeExtensions(
    pub Vec<UlUpTnlAddressToUpdateListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UlUpTnlInformationToUpdateListEntryValue {
    #[asn(key = 301)]
    IdUlUpTnlInformationToUpdateListItem(UlUpTnlInformationToUpdateListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlUpTnlInformationToUpdateListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UlUpTnlInformationToUpdateListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlUpTnlInformationToUpdateListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlUpTnlInformationToUpdateListItemIeExtensions(
    pub Vec<UlUpTnlInformationToUpdateListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlConfigurationIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UlConfigurationIeExtensions(pub Vec<UlConfigurationIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UlrrcMessageTransferProtocolIEsEntryValue {
    #[asn(key = 50)]
    IdRrcContainer(RrcContainer),
    #[asn(key = 64)]
    IdSrbid(Srbid),
    #[asn(key = 224)]
    IdSelectedPlmnid(PlmnIdentity),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
    #[asn(key = 219)]
    IdNewGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlrrcMessageTransferProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UlrrcMessageTransferProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UlrrcMessageTransferProtocolIEs(pub Vec<UlrrcMessageTransferProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UluptnlInformationToBeSetupItemIeExtensionsEntryExtensionValue {
    #[asn(key = 280)]
    IdBhInfo(BhInfo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UluptnlInformationToBeSetupItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UluptnlInformationToBeSetupItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UluptnlInformationToBeSetupItemIeExtensions(
    pub Vec<UluptnlInformationToBeSetupItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 32)]
    IdBapMappingConfiguration(BapMappingConfigurationFailure),
    #[asn(key = 52)]
    IdECidMeasurementInitiation(ECidMeasurementInitiationFailure),
    #[asn(key = 26)]
    IdF1Removal(F1RemovalFailure),
    #[asn(key = 1)]
    IdF1Setup(F1SetupFailure),
    #[asn(key = 33)]
    IdGnbduResourceConfiguration(GnbduResourceConfigurationFailure),
    #[asn(key = 34)]
    IdIabtnlAddressAllocation(IabtnlAddressFailure),
    #[asn(key = 35)]
    IdIabupConfigurationUpdate(IabupConfigurationUpdateFailure),
    #[asn(key = 50)]
    IdPositioningActivation(PositioningActivationFailure),
    #[asn(key = 49)]
    IdPositioningInformationExchange(PositioningInformationFailure),
    #[asn(key = 41)]
    IdPositioningMeasurementExchange(PositioningMeasurementFailure),
    #[asn(key = 48)]
    IdTrpInformationExchange(TrpInformationFailure),
    #[asn(key = 7)]
    IdUeContextModification(UeContextModificationFailure),
    #[asn(key = 8)]
    IdUeContextModificationRequired(UeContextModificationRefuse),
    #[asn(key = 5)]
    IdUeContextSetup(UeContextSetupFailure),
    #[asn(key = 4)]
    IdGNbcuConfigurationUpdate(GnbcuConfigurationUpdateFailure),
    #[asn(key = 3)]
    IdGNbduConfigurationUpdate(GnbduConfigurationUpdateFailure),
    #[asn(key = 36)]
    IdResourceStatusReportingInitiation(ResourceStatusFailure),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VictimgNbSetIdIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct VictimgNbSetIdIeExtensions(pub Vec<VictimgNbSetIdIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestProtocolIEsEntryValue {
    #[asn(key = 144)]
    IdCellsToBeBroadcastList(CellsToBeBroadcastList),
    #[asn(key = 142)]
    IdNumberofBroadcastRequest(NumberofBroadcastRequest),
    #[asn(key = 140)]
    IdPwsSystemInformation(PwsSystemInformation),
    #[asn(key = 141)]
    IdRepetitionPeriod(RepetitionPeriod),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningRequestProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningRequestProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct WriteReplaceWarningRequestProtocolIEs(
    pub Vec<WriteReplaceWarningRequestProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningResponseProtocolIEsEntryValue {
    #[asn(key = 146)]
    IdCellsBroadcastCompletedList(CellsBroadcastCompletedList),
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 189)]
    IdDedicatedSiDeliveryNeededUeList(DedicatedSiDeliveryNeededUeList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningResponseProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningResponseProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct WriteReplaceWarningResponseProtocolIEs(
    pub Vec<WriteReplaceWarningResponseProtocolIEsEntry>,
);
