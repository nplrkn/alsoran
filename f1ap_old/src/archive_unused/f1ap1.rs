#![allow(dead_code, unreachable_patterns)]
use super::f1ap2::*;
use asn1_codecs_derive::AperCodec;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use miniserde::{Deserialize, Serialize};

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum AbortTransmission {
    #[asn(key = 0, extended = false)]
    SrsResourceSetId(SrsResourceSetId),
    #[asn(key = 1, extended = false)]
    ReleaseAll(Null2),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(AbortTransmissionchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AccessAndMobilityIndication {
    pub protocol_i_es: AccessAndMobilityIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AccessPointPosition {
    pub latitude_sign: Enumerated3,
    pub latitude: Integer4,
    pub longitude: Integer5,
    pub direction_of_altitude: Enumerated6,
    pub altitude: Integer7,
    pub uncertainty_semi_major: Integer8,
    pub uncertainty_semi_minor: Integer9,
    pub orientation_of_major_axis: Integer10,
    pub uncertainty_altitude: Integer11,
    pub confidence: Integer12,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AccessPointPositionIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AccessSuccess {
    pub protocol_i_es: AccessSuccessProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct ActivatedCellsToBeUpdatedList(pub Vec<ActivatedCellsToBeUpdatedListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ActivatedCellsToBeUpdatedListItem {
    pub nrcgi: Nrcgi,
    pub iab_du_cell_resource_configuration_mode_info: IabDuCellResourceConfigurationModeInfo,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ActivatedCellsToBeUpdatedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct ActiveUlbwp {
    pub location_and_bandwidth: Integer13,
    pub subcarrier_spacing: Enumerated14,
    pub cyclic_prefix: Enumerated15,
    pub tx_direct_current_location: Integer16,
    #[asn(optional_idx = 0)]
    pub shift7dot5k_hz: Option<Enumerated17>,
    pub srs_config: SrsConfig,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ActiveUlbwpIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct AdditionalDuplicationIndication(pub u8);
impl AdditionalDuplicationIndication {
    const THREE: u8 = 0u8;
    const FOUR: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AdditionalPdcpDuplicationTnlItem {
    pub additional_pdcp_duplication_uptnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AdditionalPdcpDuplicationTnlItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct AdditionalPdcpDuplicationTnlList(pub Vec<AdditionalPdcpDuplicationTnlItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct AdditionalPathItem {
    pub relative_path_delay: RelativePathDelay,
    #[asn(optional_idx = 0)]
    pub path_quality: Option<TrpMeasurementQuality>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AdditionalPathItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct AdditionalPathList(pub Vec<AdditionalPathItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct AdditionalRrmPriorityIndex(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct AdditionalSibMessageList(pub Vec<AdditionalSibMessageListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AdditionalSibMessageListItem {
    pub additional_sib: OctetString18,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AdditionalSibMessageListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct AggressorCellList(pub Vec<AggressorCellListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AggressorCellListItem {
    pub aggressor_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AggressorCellListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AggressorgNbSetId {
    pub aggressorg_nb_set_id: GnbSetId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AggressorgNbSetIdIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationAndRetentionPriority {
    pub priority_level: PriorityLevel,
    pub pre_emption_capability: PreEmptionCapability,
    pub pre_emption_vulnerability: PreEmptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationAndRetentionPriorityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct AlternativeQoSParaSetItem {
    pub alternative_qo_s_para_set_index: QoSParaSetIndex,
    #[asn(optional_idx = 0)]
    pub guaranteed_flow_bit_rate_dl: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub guaranteed_flow_bit_rate_ul: Option<BitRate>,
    #[asn(optional_idx = 2)]
    pub packet_delay_budget: Option<PacketDelayBudget>,
    #[asn(optional_idx = 3)]
    pub packet_error_rate: Option<PacketErrorRate>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<AlternativeQoSParaSetItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AlternativeQoSParaSetList(pub Vec<AlternativeQoSParaSetItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct AngleMeasurementQuality {
    pub azimuth_quality: Integer19,
    #[asn(optional_idx = 0)]
    pub zenith_quality: Option<Integer20>,
    pub resolution: Enumerated21,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AngleMeasurementQualityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AperiodicSrs {
    pub aperiodic: Enumerated22,
    #[asn(optional_idx = 0)]
    pub srs_resource_trigger: Option<SrsResourceTrigger>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AperiodicSrsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3")]
pub struct AperiodicSrsResourceTrigger(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct AperiodicSrsResourceTriggerList(pub Vec<AperiodicSrsResourceTrigger>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AreaScope(pub u8);
impl AreaScope {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AssociatedSCellItem {
    pub s_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AssociatedSCellItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AssociatedSCellList(pub Vec<AssociatedSCellListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "6")]
pub struct AvailablePlmnList(pub Vec<AvailablePlmnListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AvailablePlmnListItem {
    pub plmn_identity: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AvailablePlmnListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct AvailableSnpnIdList(pub Vec<AvailableSnpnIdListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AvailableSnpnIdListItem {
    pub plmn_identity: PlmnIdentity,
    pub available_nid_list: BroadcastNidList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AvailableSnpnIdListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct AveragingWindow(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BapAddress(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BapCtrlPduChannel(pub u8);
impl BapCtrlPduChannel {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BapMappingConfiguration {
    pub protocol_i_es: BapMappingConfigurationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BapMappingConfigurationAcknowledge {
    pub protocol_i_es: BapMappingConfigurationAcknowledgeProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BapMappingConfigurationFailure {
    pub protocol_i_es: BapMappingConfigurationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BapPathId(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BapRoutingId {
    pub bap_address: BapAddress,
    pub bap_path_id: BapPathId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BapRoutingIdIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct BaPlayerBhrlCchannelMappingInfo {
    #[asn(optional_idx = 0)]
    pub ba_player_bhrl_cchannel_mapping_info_to_add: Option<BaPlayerBhrlCchannelMappingInfoList>,
    #[asn(optional_idx = 1)]
    pub ba_player_bhrl_cchannel_mapping_info_to_remove: Option<MappingInformationtoRemove>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BaPlayerBhrlCchannelMappingInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct BaPlayerBhrlCchannelMappingInfoItem {
    pub mapping_information_index: MappingInformationIndex,
    #[asn(optional_idx = 0)]
    pub prior_hop_bap_address: Option<BapAddress>,
    #[asn(optional_idx = 1)]
    pub ingressb_hrlc_channel_id: Option<BhrlcChannelId>,
    #[asn(optional_idx = 2)]
    pub next_hop_bap_address: Option<BapAddress>,
    #[asn(optional_idx = 3)]
    pub egressb_hrlc_channel_id: Option<BhrlcChannelId>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<BaPlayerBhrlCchannelMappingInfoItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "67108864"
)]
pub struct BaPlayerBhrlCchannelMappingInfoList(pub Vec<BaPlayerBhrlCchannelMappingInfoItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct BhRoutingInformationAddedList(pub Vec<BhRoutingInformationAddedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhRoutingInformationAddedListItem {
    pub bap_routing_id: BapRoutingId,
    pub next_hop_bap_address: BapAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhRoutingInformationAddedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct BhRoutingInformationRemovedList(pub Vec<BhRoutingInformationRemovedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhRoutingInformationRemovedListItem {
    pub bap_routing_id: BapRoutingId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhRoutingInformationRemovedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct BhChannelsFailedToBeModifiedItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<BhChannelsFailedToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsFailedToBeModifiedList(pub Vec<BhChannelsFailedToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct BhChannelsFailedToBeSetupItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<BhChannelsFailedToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsFailedToBeSetupList(pub Vec<BhChannelsFailedToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct BhChannelsFailedToBeSetupModItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<BhChannelsFailedToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsFailedToBeSetupModList(pub Vec<BhChannelsFailedToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhChannelsModifiedItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhChannelsModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsModifiedList(pub Vec<BhChannelsModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhChannelsRequiredToBeReleasedItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhChannelsRequiredToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsRequiredToBeReleasedList(pub Vec<BhChannelsRequiredToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhChannelsSetupItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhChannelsSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsSetupList(pub Vec<BhChannelsSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhChannelsSetupModItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhChannelsSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsSetupModList(pub Vec<BhChannelsSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct BhChannelsToBeModifiedItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    pub bh_qo_s_information: BhQoSInformation,
    #[asn(optional_idx = 0)]
    pub rl_cmode: Option<RlcMode>,
    #[asn(optional_idx = 1)]
    pub bap_ctrl_pdu_channel: Option<BapCtrlPduChannel>,
    #[asn(optional_idx = 2)]
    pub traffic_mapping_info: Option<TrafficMappingInfo>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<BhChannelsToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsToBeModifiedList(pub Vec<BhChannelsToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct BhChannelsToBeReleasedItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BhChannelsToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsToBeReleasedList(pub Vec<BhChannelsToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct BhChannelsToBeSetupItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    pub bh_qo_s_information: BhQoSInformation,
    pub rl_cmode: RlcMode,
    #[asn(optional_idx = 0)]
    pub bap_ctrl_pdu_channel: Option<BapCtrlPduChannel>,
    #[asn(optional_idx = 1)]
    pub traffic_mapping_info: Option<TrafficMappingInfo>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BhChannelsToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsToBeSetupList(pub Vec<BhChannelsToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct BhChannelsToBeSetupModItem {
    pub bhrlc_channel_id: BhrlcChannelId,
    pub bh_qo_s_information: BhQoSInformation,
    pub rl_cmode: RlcMode,
    #[asn(optional_idx = 0)]
    pub bap_ctrl_pdu_channel: Option<BapCtrlPduChannel>,
    #[asn(optional_idx = 1)]
    pub traffic_mapping_info: Option<TrafficMappingInfo>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BhChannelsToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct BhChannelsToBeSetupModList(pub Vec<BhChannelsToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct BhInfo {
    #[asn(optional_idx = 0)]
    pub ba_prouting_id: Option<BapRoutingId>,
    #[asn(optional_idx = 1)]
    pub egress_bhrlcch_list: Option<EgressBhrlcchList>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BhInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum BhQoSInformation {
    #[asn(key = 0, extended = false)]
    BhrlcchQoS(QoSFlowLevelQoSParameters),
    #[asn(key = 1, extended = false)]
    EutranbhrlcchQoS(EutranQoS),
    #[asn(key = 2, extended = false)]
    CpTrafficType(CpTrafficType),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(BhQoSInformationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BhrlcChannelId(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct BplmnIdInfoItem {
    pub plmn_identity_list: AvailablePlmnList,
    #[asn(optional_idx = 0)]
    pub extended_plmn_identity_list: Option<ExtendedAvailablePlmnList>,
    #[asn(optional_idx = 1)]
    pub five_gs_tac: Option<FiveGsTac>,
    pub nr_cell_id: NrCellIdentity,
    #[asn(optional_idx = 2)]
    pub ranac: Option<Ranac>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<BplmnIdInfoItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BplmnIdInfoList(pub Vec<BplmnIdInfoItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum BandwidthSrs {
    #[asn(key = 0, extended = false)]
    Fr1(Fr1Bandwidth),
    #[asn(key = 1, extended = false)]
    Fr2(Fr2Bandwidth),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(BandwidthSrSchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BearerTypeChange(pub u8);
impl BearerTypeChange {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4000000000000", extensible = true)]
pub struct BitRate(pub u64);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastToBeCancelledItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastToBeCancelledItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct BroadcastToBeCancelledList(pub Vec<BroadcastToBeCancelledListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastCagList(pub Vec<Cagid>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastNidList(pub Vec<Nid>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastPniNpnIdList(pub Vec<BroadcastPniNpnIdListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastPniNpnIdListItem {
    pub plmn_identity: PlmnIdentity,
    pub broadcast_cag_list: BroadcastCagList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastPniNpnIdListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastSnpnIdList(pub Vec<BroadcastSnpnIdListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastSnpnIdListItem {
    pub plmn_identity: PlmnIdentity,
    pub broadcast_nid_list: BroadcastNidList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastSnpnIdListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct BurstArrivalTime(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535", extensible = true)]
pub struct CRnti(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct Cagid(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CgConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CgConfigInfo(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100")]
pub struct ChoProbability(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ChOtriggerInterDu(pub u8);
impl ChOtriggerInterDu {
    const CHO_INITIATION: u8 = 0u8;
    const CHO_REPLACE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ChOtriggerIntraDu(pub u8);
impl ChOtriggerIntraDu {
    const CHO_INITIATION: u8 = 0u8;
    const CHO_REPLACE: u8 = 1u8;
    const CHO_CANCEL: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CnuePagingIdentity {
    #[asn(key = 0, extended = false)]
    FiveGSTmsi(BitString23),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(CnuePagingIdentitychoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CpTransportLayerAddress {
    #[asn(key = 0, extended = false)]
    EndpointIpAddress(TransportLayerAddress),
    #[asn(key = 1, extended = false)]
    EndpointIpAddressAndPort(EndpointIpAddressAndPort),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(CpTransportLayerAddresschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3", extensible = true)]
pub struct CpTrafficType(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct CudurimInformation {
    pub victimg_nb_set_id: GnbSetId,
    pub rimrs_detection_status: RimrsDetectionStatus,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CudurimInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CuduRadioInformationTransfer {
    pub protocol_i_es: CuduRadioInformationTransferProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CuduRadioInformationType {
    #[asn(key = 0, extended = false)]
    Rim(CudurimInformation),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(CuduRadioInformationTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct CUtoDurrcInformation {
    #[asn(optional_idx = 0)]
    pub cg_config_info: Option<CgConfigInfo>,
    #[asn(optional_idx = 1)]
    pub ue_capability_rat_container_list: Option<UeCapabilityRatContainerList>,
    #[asn(optional_idx = 2)]
    pub meas_config: Option<MeasConfig>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<CUtoDurrcInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CancelAllWarningMessagesIndicator(pub u8);
impl CancelAllWarningMessagesIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateSpCellItem {
    pub candidate_sp_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateSpCellItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct CandidateSpCellList(pub Vec<CandidateSpCellListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct CapacityValue {
    pub capacity_value: Integer24,
    #[asn(optional_idx = 0)]
    pub ssb_area_capacity_value_list: Option<SsbAreaCapacityValueList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CapacityValueIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    RadioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    Transport(CauseTransport),
    #[asn(key = 2, extended = false)]
    Protocol(CauseProtocol),
    #[asn(key = 3, extended = false)]
    Misc(CauseMisc),
    #[asn(key = 4, extended = false)]
    ChoiceExtension(CausechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct CauseMisc(pub u8);
impl CauseMisc {
    const CONTROL_PROCESSING_OVERLOAD: u8 = 0u8;
    const NOT_ENOUGH_USER_PLANE_PROCESSING_RESOURCES: u8 = 1u8;
    const HARDWARE_FAILURE: u8 = 2u8;
    const OM_INTERVENTION: u8 = 3u8;
    const UNSPECIFIED: u8 = 4u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct CauseProtocol(pub u8);
impl CauseProtocol {
    const TRANSFER_SYNTAX_ERROR: u8 = 0u8;
    const ABSTRACT_SYNTAX_ERROR_REJECT: u8 = 1u8;
    const ABSTRACT_SYNTAX_ERROR_IGNORE_AND_NOTIFY: u8 = 2u8;
    const MESSAGE_NOT_COMPATIBLE_WITH_RECEIVER_STATE: u8 = 3u8;
    const SEMANTIC_ERROR: u8 = 4u8;
    const ABSTRACT_SYNTAX_ERROR_FALSELY_CONSTRUCTED_MESSAGE: u8 = 5u8;
    const UNSPECIFIED: u8 = 6u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "10")]
pub struct CauseRadioNetwork(pub u8);
impl CauseRadioNetwork {
    const UNSPECIFIED: u8 = 0u8;
    const RL_FAILURE_RLC: u8 = 1u8;
    const UNKNOWN_OR_ALREADY_ALLOCATED_GNB_CU_UE_F1AP_ID: u8 = 2u8;
    const UNKNOWN_OR_ALREADY_ALLOCATED_GNB_DU_UE_F1AP_ID: u8 = 3u8;
    const UNKNOWN_OR_INCONSISTENT_PAIR_OF_UE_F1AP_ID: u8 = 4u8;
    const INTERACTION_WITH_OTHER_PROCEDURE: u8 = 5u8;
    const NOT_SUPPORTED_QCI_VALUE: u8 = 6u8;
    const ACTION_DESIRABLE_FOR_RADIO_REASONS: u8 = 7u8;
    const NO_RADIO_RESOURCES_AVAILABLE: u8 = 8u8;
    const PROCEDURE_CANCELLED: u8 = 9u8;
    const NORMAL_RELEASE: u8 = 10u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CauseTransport(pub u8);
impl CauseTransport {
    const UNSPECIFIED: u8 = 0u8;
    const TRANSPORT_RESOURCE_UNAVAILABLE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct CellDirection(pub u8);
impl CellDirection {
    const DL_ONLY: u8 = 0u8;
    const UL_ONLY: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct CellPortionId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CellBarred(pub u8);
impl CellBarred {
    const BARRED: u8 = 0u8;
    const NOT_BARRED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100", extensible = true)]
pub struct CellCapacityClassValue(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CellGroupConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct CellMeasurementResultItem {
    pub cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub radio_resource_status: Option<RadioResourceStatus>,
    #[asn(optional_idx = 1)]
    pub composite_available_capacity_group: Option<CompositeAvailableCapacityGroup>,
    #[asn(optional_idx = 2)]
    pub slice_available_capacity: Option<SliceAvailableCapacity>,
    #[asn(optional_idx = 3)]
    pub numberof_active_u_es: Option<NumberofActiveUEs>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<CellMeasurementResultItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellMeasurementResultList(pub Vec<CellMeasurementResultItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CellSize(pub u8);
impl CellSize {
    const VERYSMALL: u8 = 0u8;
    const SMALL: u8 = 1u8;
    const MEDIUM: u8 = 2u8;
    const LARGE: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct CellToReportItem {
    pub cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ssb_to_report_list: Option<SsbToReportList>,
    #[asn(optional_idx = 1)]
    pub slice_to_report_list: Option<SliceToReportList>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<CellToReportItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellToReportList(pub Vec<CellToReportItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellTrafficTrace {
    pub protocol_i_es: CellTrafficTraceProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellType {
    pub cell_size: CellSize,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellTypeIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CellUlConfigured(pub u8);
impl CellUlConfigured {
    const NONE: u8 = 0u8;
    const UL: u8 = 1u8;
    const SUL: u8 = 2u8;
    const UL_AND_SUL: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellsBroadcastCancelledItem {
    pub nrcgi: Nrcgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsBroadcastCancelledItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsBroadcastCancelledList(pub Vec<CellsBroadcastCancelledListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellsBroadcastCompletedItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsBroadcastCompletedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsBroadcastCompletedList(pub Vec<CellsBroadcastCompletedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsFailedToBeActivatedList(pub Vec<CellsFailedToBeActivatedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellsFailedToBeActivatedListItem {
    pub nrcgi: Nrcgi,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsFailedToBeActivatedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellsStatusItem {
    pub nrcgi: Nrcgi,
    pub service_status: ServiceStatus,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsStatusItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "512"
)]
pub struct CellsStatusList(pub Vec<CellsStatusListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellsToBeBroadcastItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsToBeBroadcastItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsToBeBroadcastList(pub Vec<CellsToBeBroadcastListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsToBeActivatedList(pub Vec<CellsToBeActivatedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CellsToBeActivatedListItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub nrpci: Option<Nrpci>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CellsToBeActivatedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct CellsToBeBarredItem {
    pub nrcgi: Nrcgi,
    pub cell_barred: CellBarred,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsToBeBarredItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsToBeBarredList(pub Vec<CellsToBeBarredListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct CellsToBeDeactivatedList(pub Vec<CellsToBeDeactivatedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellsToBeDeactivatedListItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellsToBeDeactivatedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct ChildNodeCellsList(pub Vec<ChildNodeCellsListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 10)]
pub struct ChildNodeCellsListItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub iab_du_cell_resource_configuration_mode_info:
        Option<IabDuCellResourceConfigurationModeInfo>,
    #[asn(optional_idx = 1)]
    pub iab_stc_info: Option<IabStcInfo>,
    #[asn(optional_idx = 2)]
    pub rach_config_common: Option<RachConfigCommon>,
    #[asn(optional_idx = 3)]
    pub rach_config_common_iab: Option<RachConfigCommonIab>,
    #[asn(optional_idx = 4)]
    pub csi_rs_configuration: Option<OctetString25>,
    #[asn(optional_idx = 5)]
    pub sr_configuration: Option<OctetString26>,
    #[asn(optional_idx = 6)]
    pub pdcch_config_sib1: Option<OctetString27>,
    #[asn(optional_idx = 7)]
    pub scs_common: Option<OctetString28>,
    #[asn(optional_idx = 8)]
    pub multiplexing_info: Option<MultiplexingInfo>,
    #[asn(optional_idx = 9)]
    pub ie_extensions: Option<ChildNodeCellsListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct ChildNodesList(pub Vec<ChildNodesListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct ChildNodesListItem {
    pub gnb_cu_ue_f1ap_id: GnbCuUeF1apId,
    pub gnb_du_ue_f1ap_id: GnbDuUeF1apId,
    #[asn(optional_idx = 0)]
    pub child_node_cells_list: Option<ChildNodeCellsList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ChildNodesListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct CompositeAvailableCapacity {
    #[asn(optional_idx = 0)]
    pub cell_capacity_class_value: Option<CellCapacityClassValue>,
    pub capacity_value: CapacityValue,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CompositeAvailableCapacityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct CompositeAvailableCapacityGroup {
    pub composite_available_capacity_downlink: CompositeAvailableCapacity,
    pub composite_available_capacity_uplink: CompositeAvailableCapacity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompositeAvailableCapacityGroupIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ConditionalInterDuMobilityInformation {
    pub cho_trigger: ChOtriggerInterDu,
    #[asn(optional_idx = 0)]
    pub targetg_nb_duuef1apid: Option<GnbDuUeF1apId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ConditionalInterDuMobilityInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ConditionalIntraDuMobilityInformation {
    pub cho_trigger: ChOtriggerIntraDu,
    #[asn(optional_idx = 0)]
    pub target_cells_tocancel: Option<TargetCellList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ConditionalIntraDuMobilityInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct ConfiguredEpsTac(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ConfiguredTacIndication(pub u8);
impl ConfiguredTacIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "511", extensible = true)]
pub struct CoordinateId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(pub u8);
impl Criticality {
    pub const REJECT: u8 = 0u8;
    pub const IGNORE: u8 = 1u8;
    pub const NOTIFY: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct CriticalityDiagnostics {
    #[asn(optional_idx = 0)]
    pub procedure_code: Option<ProcedureCode>,
    #[asn(optional_idx = 1)]
    pub triggering_message: Option<TriggeringMessage>,
    #[asn(optional_idx = 2)]
    pub procedure_criticality: Option<Criticality>,
    #[asn(optional_idx = 3)]
    pub transaction_id: Option<TransactionId>,
    #[asn(optional_idx = 4)]
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIeList>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<CriticalityDiagnosticsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnosticsIeItem {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIeId,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CriticalityDiagnosticsIeItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CriticalityDiagnosticsIeList(pub Vec<CriticalityDiagnosticsIeItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DcBasedDuplicationConfigured(pub u8);
impl DcBasedDuplicationConfigured {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct DlPrs {
    pub prsid: Integer29,
    pub dl_prs_resource_set_id: PrsResourceSetId,
    #[asn(optional_idx = 0)]
    pub dl_prs_resource_id: Option<PrsResourceId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DlPrsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum DlPrsMutingPattern {
    #[asn(key = 0, extended = false)]
    Two(BitString30),
    #[asn(key = 1, extended = false)]
    Four(BitString31),
    #[asn(key = 2, extended = false)]
    Six(BitString32),
    #[asn(key = 3, extended = false)]
    Eight(BitString33),
    #[asn(key = 4, extended = false)]
    Sixteen(BitString34),
    #[asn(key = 5, extended = false)]
    ThirtyTwo(BitString35),
    #[asn(key = 6, extended = false)]
    ChoiceExtension(DlPrsMutingPatternchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DlPrsResourceArpLocation {
    #[asn(key = 0, extended = false)]
    RelativeGeodeticLocation(RelativeGeodeticLocation),
    #[asn(key = 1, extended = false)]
    RelativeCartesianLocation(RelativeCartesianLocation),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(DlPrsResourceArpLocationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DlPrsResourceSetArpLocation {
    #[asn(key = 0, extended = false)]
    RelativeGeodeticLocation(RelativeGeodeticLocation),
    #[asn(key = 1, extended = false)]
    RelativeCartesianLocation(RelativeCartesianLocation),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(DlPrsResourceSetArpLocationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct DlUpTnlAddressToUpdateList(pub Vec<DlUpTnlAddressToUpdateListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DlUpTnlAddressToUpdateListItem {
    pub old_ip_adress: TransportLayerAddress,
    pub new_ip_adress: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DlUpTnlAddressToUpdateListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct DlprsResourceArp {
    pub dl_prs_resource_id: PrsResourceId,
    pub dl_prs_resource_arp_location: DlPrsResourceArpLocation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DlprsResourceArpIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct DlprsResourceCoordinates {
    pub listof_dl_prs_resource_set_arp: DlprsResourceCoordinatesListofDlPrsResourceSetArp,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DlprsResourceCoordinatesIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct DlprsResourceSetArp {
    pub dl_prs_resource_set_id: PrsResourceSetId,
    pub dl_prs_resource_set_arp_location: DlPrsResourceSetArpLocation,
    pub listof_dl_prs_resource_arp: DlprsResourceSetArpListofDlPrsResourceArp,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DlprsResourceSetArpIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DlrrcMessageTransfer {
    pub protocol_i_es: DlrrcMessageTransferProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DluptnlInformationToBeSetupItem {
    pub dluptnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DluptnlInformationToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct DluptnlInformationToBeSetupList(pub Vec<DluptnlInformationToBeSetupItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DrbActivity(pub u8);
impl DrbActivity {
    const ACTIVE: u8 = 0u8;
    const NOT_ACTIVE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrbActivityItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub drb_activity: Option<DrbActivity>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrbActivityItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrbActivityList(pub Vec<DrbActivityListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct DrbInformation {
    pub drb_qo_s: QoSFlowLevelQoSParameters,
    pub snssai: Snssai,
    #[asn(optional_idx = 0)]
    pub notification_control: Option<NotificationControl>,
    pub flows_mapped_to_drb_list: FlowsMappedToDrbList,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrbInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrbNotifyItem {
    pub drbid: Drbid,
    pub notification_cause: NotificationCause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrbNotifyItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrbNotifyList(pub Vec<DrbNotifyListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32", extensible = true)]
pub struct Drbid(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrBsFailedToBeModifiedItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrBsFailedToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsFailedToBeModifiedList(pub Vec<DrBsFailedToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrBsFailedToBeSetupItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrBsFailedToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsFailedToBeSetupList(pub Vec<DrBsFailedToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrBsFailedToBeSetupModItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrBsFailedToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsFailedToBeSetupModList(pub Vec<DrBsFailedToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrBsModifiedItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub lcid: Option<Lcid>,
    pub dluptnl_information_to_be_setup_list: DluptnlInformationToBeSetupList,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrBsModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsModifiedList(pub Vec<DrBsModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsModifiedConfItem {
    pub drbid: Drbid,
    pub uluptnl_information_to_be_setup_list: UluptnlInformationToBeSetupList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrBsModifiedConfItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsModifiedConfList(pub Vec<DrBsModifiedConfListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsRequiredToBeModifiedItem {
    pub drbid: Drbid,
    pub dluptnl_information_to_be_setup_list: DluptnlInformationToBeSetupList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrBsRequiredToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsRequiredToBeModifiedList(pub Vec<DrBsRequiredToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsRequiredToBeReleasedItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrBsRequiredToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsRequiredToBeReleasedList(pub Vec<DrBsRequiredToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrBsSetupItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub lcid: Option<Lcid>,
    pub dluptnl_information_to_be_setup_list: DluptnlInformationToBeSetupList,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrBsSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsSetupList(pub Vec<DrBsSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrBsSetupModItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub lcid: Option<Lcid>,
    pub dluptnl_information_to_be_setup_list: DluptnlInformationToBeSetupList,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<DrBsSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsSetupModList(pub Vec<DrBsSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DrBsToBeModifiedItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub qo_s_information: Option<QoSInformation>,
    pub uluptnl_information_to_be_setup_list: UluptnlInformationToBeSetupList,
    #[asn(optional_idx = 1)]
    pub ul_configuration: Option<UlConfiguration>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DrBsToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsToBeModifiedList(pub Vec<DrBsToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsToBeReleasedItem {
    pub drbid: Drbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrBsToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsToBeReleasedList(pub Vec<DrBsToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DrBsToBeSetupItem {
    pub drbid: Drbid,
    pub qo_s_information: QoSInformation,
    pub uluptnl_information_to_be_setup_list: UluptnlInformationToBeSetupList,
    pub rlc_mode: RlcMode,
    #[asn(optional_idx = 0)]
    pub ul_configuration: Option<UlConfiguration>,
    #[asn(optional_idx = 1)]
    pub duplication_activation: Option<DuplicationActivation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DrBsToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsToBeSetupList(pub Vec<DrBsToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DrBsToBeSetupModItem {
    pub drbid: Drbid,
    pub qo_s_information: QoSInformation,
    pub uluptnl_information_to_be_setup_list: UluptnlInformationToBeSetupList,
    pub rlc_mode: RlcMode,
    #[asn(optional_idx = 0)]
    pub ul_configuration: Option<UlConfiguration>,
    #[asn(optional_idx = 1)]
    pub duplication_activation: Option<DuplicationActivation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DrBsToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DrBsToBeSetupModList(pub Vec<DrBsToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct DrxConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "10239")]
pub struct DrxLongCycleStartOffset(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DrxConfigurationIndicator(pub u8);
impl DrxConfigurationIndicator {
    const RELEASE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DrxCycle {
    pub long_drx_cycle_length: LongDrxCycleLength,
    #[asn(optional_idx = 0)]
    pub short_drx_cycle_length: Option<ShortDrxCycleLength>,
    #[asn(optional_idx = 1)]
    pub short_drx_cycle_timer: Option<ShortDrxCycleTimer>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DrxCycleIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct Dscp(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "64")]
pub struct DsInformationList(pub Vec<Dscp>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DuRxMtRx(pub u8);
impl DuRxMtRx {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DuRxMtTx(pub u8);
impl DuRxMtTx {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DuTxMtRx(pub u8);
impl DuTxMtRx {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct DuTxMtTx(pub u8);
impl DuTxMtTx {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct DucurimInformation {
    pub victimg_nb_set_id: GnbSetId,
    pub rimrs_detection_status: RimrsDetectionStatus,
    pub aggressor_cell_list: AggressorCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DucurimInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DucuRadioInformationTransfer {
    pub protocol_i_es: DucuRadioInformationTransferProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum DucuRadioInformationType {
    #[asn(key = 0, extended = false)]
    Rim(DucurimInformation),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(DucuRadioInformationTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DufSlotConfigItem {
    #[asn(key = 0, extended = false)]
    ExplicitFormat(ExplicitFormat),
    #[asn(key = 1, extended = false)]
    ImplicitFormat(ImplicitFormat),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(DufSlotConfigItemchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "320"
)]
pub struct DufSlotConfigList(pub Vec<DufSlotConfigItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "254")]
pub struct DufSlotformatIndex(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct DufTransmissionPeriodicity(pub u8);
impl DufTransmissionPeriodicity {
    const MS0P5: u8 = 0u8;
    const MS0P625: u8 = 1u8;
    const MS1: u8 = 2u8;
    const MS1P25: u8 = 3u8;
    const MS2: u8 = 4u8;
    const MS2P5: u8 = 5u8;
    const MS5: u8 = 6u8;
    const MS10: u8 = 7u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct DUtoCurrcContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DUtoCurrcInformation {
    pub cell_group_config: CellGroupConfig,
    #[asn(optional_idx = 0)]
    pub meas_gap_config: Option<MeasGapConfig>,
    #[asn(optional_idx = 1)]
    pub requested_p_max_fr1: Option<OctetString36>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DUtoCurrcInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DeactivateTrace {
    pub protocol_i_es: DeactivateTraceProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DedicatedSiDeliveryNeededUeItem {
    pub gnb_cu_ue_f1ap_id: GnbCuUeF1apId,
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DedicatedSiDeliveryNeededUeItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct DedicatedSiDeliveryNeededUeList(pub Vec<DedicatedSiDeliveryNeededUeListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DuplicationActivation(pub u8);
impl DuplicationActivation {
    const ACTIVE: u8 = 0u8;
    const INACTIVE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DuplicationIndication(pub u8);
impl DuplicationIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DuplicationState(pub u8);
impl DuplicationState {
    const ACTIVE: u8 = 0u8;
    const INACTIVE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct Dynamic5QiDescriptor {
    pub qo_s_priority_level: Integer37,
    pub packet_delay_budget: PacketDelayBudget,
    pub packet_error_rate: PacketErrorRate,
    #[asn(optional_idx = 0)]
    pub five_qi: Option<Integer38>,
    #[asn(optional_idx = 1)]
    pub delay_critical: Option<Enumerated39>,
    #[asn(optional_idx = 2)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 3)]
    pub max_data_burst_volume: Option<MaxDataBurstVolume>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<Dynamic5QiDescriptorIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct DynamicPqiDescriptor {
    #[asn(optional_idx = 0)]
    pub resource_type: Option<Enumerated40>,
    pub qo_s_priority_level: Integer41,
    pub packet_delay_budget: PacketDelayBudget,
    pub packet_error_rate: PacketErrorRate,
    #[asn(optional_idx = 1)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 2)]
    pub max_data_burst_volume: Option<MaxDataBurstVolume>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<DynamicPqiDescriptorIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ECidMeasuredResultsItem {
    pub e_cid_measured_results_value: ECidMeasuredResultsValue,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ECidMeasuredResultsItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct ECidMeasuredResultsList(pub Vec<ECidMeasuredResultsItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ECidMeasuredResultsValue {
    #[asn(key = 0, extended = false)]
    ValueAngleofArrivalNr(UlAoA),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(ECidMeasuredResultsValuechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct ECidMeasurementQuantities(pub Vec<ECidMeasurementQuantitiesEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ECidMeasurementQuantitiesItem {
    pub e_ci_dmeasurement_quantities_value: ECidMeasurementQuantitiesValue,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ECidMeasurementQuantitiesItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ECidMeasurementQuantitiesValue(pub u8);
impl ECidMeasurementQuantitiesValue {
    const DEFAULT: u8 = 0u8;
    const ANGLE_OF_ARRIVAL_NR: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct ECidMeasurementResult {
    #[asn(optional_idx = 0)]
    pub geographical_coordinates: Option<GeographicalCoordinates>,
    #[asn(optional_idx = 1)]
    pub measured_results_list: Option<ECidMeasuredResultsList>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ECidMeasurementResultIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ECidReportCharacteristics(pub u8);
impl ECidReportCharacteristics {
    const ON_DEMAND: u8 = 0u8;
    const PERIODIC: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECidMeasurementFailureIndication {
    pub protocol_i_es: ECidMeasurementFailureIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECidMeasurementInitiationFailure {
    pub protocol_i_es: ECidMeasurementInitiationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECidMeasurementInitiationRequest {
    pub protocol_i_es: ECidMeasurementInitiationRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECidMeasurementInitiationResponse {
    pub protocol_i_es: ECidMeasurementInitiationResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECidMeasurementReport {
    pub protocol_i_es: ECidMeasurementReportProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECidMeasurementTerminationCommand {
    pub protocol_i_es: ECidMeasurementTerminationCommandProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct EutraCellId(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct EutraCoexFddInfo {
    #[asn(optional_idx = 0)]
    pub ul_earfcn: Option<ExtendedEarfcn>,
    pub dl_earfcn: ExtendedEarfcn,
    #[asn(optional_idx = 1)]
    pub ul_transmission_bandwidth: Option<EutraTransmissionBandwidth>,
    pub dl_transmission_bandwidth: EutraTransmissionBandwidth,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<EutraCoexFddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum EutraCoexModeInfo {
    #[asn(key = 0, extended = false)]
    Fdd(EutraCoexFddInfo),
    #[asn(key = 1, extended = false)]
    Tdd(EutraCoexTddInfo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EutraCoexTddInfo {
    pub earfcn: ExtendedEarfcn,
    pub transmission_bandwidth: EutraTransmissionBandwidth,
    pub subframe_assignment: EutraSubframeAssignment,
    pub special_subframe_info: EutraSpecialSubframeInfo,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraCoexTddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EutraCyclicPrefixDl(pub u8);
impl EutraCyclicPrefixDl {
    const NORMAL: u8 = 0u8;
    const EXTENDED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EutraCyclicPrefixUl(pub u8);
impl EutraCyclicPrefixUl {
    const NORMAL: u8 = 0u8;
    const EXTENDED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EutraFddInfo {
    pub ul_offset_to_point_a: OffsetToPointA,
    pub dl_offset_to_point_a: OffsetToPointA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraFddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum EutraModeInfo {
    #[asn(key = 0, extended = false)]
    Eutrafdd(EutraFddInfo),
    #[asn(key = 1, extended = false)]
    Eutratdd(EutraTddInfo),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(EutraModeInfochoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EutraNrCellResourceCoordinationReqContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EutraNrCellResourceCoordinationReqAckContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EutraPrachConfiguration {
    pub root_sequence_index: Integer42,
    pub zero_correlation_index: Integer43,
    pub high_speed_flag: Boolean44,
    pub prach_freq_offset: Integer45,
    #[asn(optional_idx = 0)]
    pub prach_config_index: Option<Integer46>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EutraPrachConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EutraSpecialSubframeInfo {
    pub special_subframe_patterns: EutraSpecialSubframePatterns,
    pub cyclic_prefix_dl: EutraCyclicPrefixDl,
    pub cyclic_prefix_ul: EutraCyclicPrefixUl,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraSpecialSubframeInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "10")]
pub struct EutraSpecialSubframePatterns(pub u8);
impl EutraSpecialSubframePatterns {
    const SSP0: u8 = 0u8;
    const SSP1: u8 = 1u8;
    const SSP2: u8 = 2u8;
    const SSP3: u8 = 3u8;
    const SSP4: u8 = 4u8;
    const SSP5: u8 = 5u8;
    const SSP6: u8 = 6u8;
    const SSP7: u8 = 7u8;
    const SSP8: u8 = 8u8;
    const SSP9: u8 = 9u8;
    const SSP10: u8 = 10u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct EutraSubframeAssignment(pub u8);
impl EutraSubframeAssignment {
    const SA0: u8 = 0u8;
    const SA1: u8 = 1u8;
    const SA2: u8 = 2u8;
    const SA3: u8 = 3u8;
    const SA4: u8 = 4u8;
    const SA5: u8 = 5u8;
    const SA6: u8 = 6u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EutraTddInfo {
    pub offset_to_point_a: OffsetToPointA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraTddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct EutraTransmissionBandwidth(pub u8);
impl EutraTransmissionBandwidth {
    const BW6: u8 = 0u8;
    const BW15: u8 = 1u8;
    const BW25: u8 = 2u8;
    const BW50: u8 = 3u8;
    const BW75: u8 = 4u8;
    const BW100: u8 = 5u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct EutraCellsList(pub Vec<EutraCellsListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EutraCellsListItem {
    pub eutra_cell_id: EutraCellId,
    pub served_eutra_cells_information: ServedEutraCellsInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraCellsListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EutranQoS {
    pub qci: Qci,
    pub allocation_and_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GbrQosInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EutranQoSIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EgressBhrlcchItem {
    pub next_hop_bap_address: BapAddress,
    pub bhrlc_channel_id: BhrlcChannelId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EgressBhrlcchItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct EgressBhrlcchList(pub Vec<EgressBhrlcchItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EndpointIpAddressAndPort {
    pub endpoint_ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EndpointIpAddressAndPortIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct EventType(pub u8);
impl EventType {
    const ON_DEMAND: u8 = 0u8;
    const PERIODIC: u8 = 1u8;
    const STOP: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ExecuteDuplication(pub u8);
impl ExecuteDuplication {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct ExplicitFormat {
    pub permutation: Permutation,
    #[asn(optional_idx = 0)]
    pub noof_downlink_symbols: Option<NoofDownlinkSymbols>,
    #[asn(optional_idx = 1)]
    pub noof_uplink_symbols: Option<NoofUplinkSymbols>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExplicitFormatIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedGnbCuName {
    #[asn(optional_idx = 0)]
    pub gnb_cu_name_visible_string: Option<GnbCuNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub gnb_cu_name_utf8_string: Option<GnbCuNameUtf8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedGnbCuNameIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedGnbDuName {
    #[asn(optional_idx = 0)]
    pub gnb_du_name_visible_string: Option<GnbDuNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub gnb_du_name_utf8_string: Option<GnbDuNameUtf8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedGnbDuNameIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ExtendedAvailablePlmnItem {
    pub plmn_identity: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ExtendedAvailablePlmnItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "6")]
pub struct ExtendedAvailablePlmnList(pub Vec<ExtendedAvailablePlmnItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct ExtendedEarfcn(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65535", extensible = true)]
pub struct ExtendedPacketDelayBudget(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ExtendedServedPlmNsItem {
    pub plmn_identity: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub tai_slice_support_list: Option<SliceSupportList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ExtendedServedPlmNsItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "6")]
pub struct ExtendedServedPlmNsList(pub Vec<ExtendedServedPlmNsItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedSliceSupportList(pub Vec<SliceSupportItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum F1apPdu {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(F1apPdUchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct F1cPathNsa(pub u8);
impl F1cPathNsa {
    const LTE: u8 = 0u8;
    const NR: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct F1cTransferPath {
    pub f1c_path_nsa: F1cPathNsa,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<F1cTransferPathIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct F1RemovalFailure {
    pub protocol_i_es: F1RemovalFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct F1RemovalRequest {
    pub protocol_i_es: F1RemovalRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct F1RemovalResponse {
    pub protocol_i_es: F1RemovalResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct F1SetupFailure {
    pub protocol_i_es: F1SetupFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct F1SetupRequest {
    pub protocol_i_es: F1SetupRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct F1SetupResponse {
    pub protocol_i_es: F1SetupResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FddInfo {
    pub ul_nr_freq_info: NrFreqInfo,
    pub dl_nr_freq_info: NrFreqInfo,
    pub ul_transmission_bandwidth: TransmissionBandwidth,
    pub dl_transmission_bandwidth: TransmissionBandwidth,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct Fr1Bandwidth(pub u8);
impl Fr1Bandwidth {
    const BW5: u8 = 0u8;
    const BW10: u8 = 1u8;
    const BW20: u8 = 2u8;
    const BW40: u8 = 3u8;
    const BW50: u8 = 4u8;
    const BW80: u8 = 5u8;
    const BW100: u8 = 6u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Fr2Bandwidth(pub u8);
impl Fr2Bandwidth {
    const BW50: u8 = 0u8;
    const BW100: u8 = 1u8;
    const BW200: u8 = 2u8;
    const BW400: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct FiveGsTac(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FlowsMappedToDrbItem {
    pub qo_s_flow_identifier: QoSFlowIdentifier,
    pub qo_s_flow_level_qo_s_parameters: QoSFlowLevelQoSParameters,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FlowsMappedToDrbItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct FlowsMappedToDrbList(pub Vec<FlowsMappedToDrbItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FlowsMappedToSldrbItem {
    pub pc5_qo_s_flow_identifier: Pc5QoSFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FlowsMappedToSldrbItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "2048"
)]
pub struct FlowsMappedToSldrbList(pub Vec<FlowsMappedToSldrbItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FreqBandNrItem {
    pub freq_band_indicator_nr: Integer47,
    pub supported_sul_band_list: FreqBandNrItemSupportedSulBandList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FreqBandNrItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum FreqDomainLength {
    #[asn(key = 0, extended = false)]
    L839(L839Info),
    #[asn(key = 1, extended = false)]
    L139(L139Info),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(FreqDomainLengthchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct FrequencyShift7p5khz(pub u8);
impl FrequencyShift7p5khz {
    const FALSE: u8 = 0u8;
    const TRUE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct FullConfiguration(pub u8);
impl FullConfiguration {
    const FULL: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct GbrQoSFlowInformation {
    pub max_flow_bit_rate_downlink: BitRate,
    pub max_flow_bit_rate_uplink: BitRate,
    pub guaranteed_flow_bit_rate_downlink: BitRate,
    pub guaranteed_flow_bit_rate_uplink: BitRate,
    #[asn(optional_idx = 0)]
    pub max_packet_loss_rate_downlink: Option<MaxPacketLossRate>,
    #[asn(optional_idx = 1)]
    pub max_packet_loss_rate_uplink: Option<MaxPacketLossRate>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<GbrQoSFlowInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GbrQosInformation {
    pub e_rab_maximum_bitrate_dl: BitRate,
    pub e_rab_maximum_bitrate_ul: BitRate,
    pub e_rab_guaranteed_bitrate_dl: BitRate,
    pub e_rab_guaranteed_bitrate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GbrQosInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct GnbCuName(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct GnbCuNameUtf8String(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "VisibleString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct GnbCuNameVisibleString(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GnbCuTnlAssociationFailedToSetupItem {
    pub tnl_association_transport_layer_address: CpTransportLayerAddress,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GnbCuTnlAssociationFailedToSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbCuTnlAssociationFailedToSetupList(pub Vec<GnbCuTnlAssociationFailedToSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GnbCuTnlAssociationSetupItem {
    pub tnl_association_transport_layer_address: CpTransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GnbCuTnlAssociationSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbCuTnlAssociationSetupList(pub Vec<GnbCuTnlAssociationSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GnbCuTnlAssociationToAddItem {
    pub tnl_association_transport_layer_address: CpTransportLayerAddress,
    pub tnl_association_usage: TnlAssociationUsage,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GnbCuTnlAssociationToAddItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbCuTnlAssociationToAddList(pub Vec<GnbCuTnlAssociationToAddListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GnbCuTnlAssociationToRemoveItem {
    pub tnl_association_transport_layer_address: CpTransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GnbCuTnlAssociationToRemoveItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbCuTnlAssociationToRemoveList(pub Vec<GnbCuTnlAssociationToRemoveListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct GnbCuTnlAssociationToUpdateItem {
    pub tnl_association_transport_layer_address: CpTransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TnlAssociationUsage>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GnbCuTnlAssociationToUpdateItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbCuTnlAssociationToUpdateList(pub Vec<GnbCuTnlAssociationToUpdateListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct GnbCuUeF1apId(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GnbCuSystemInformation {
    pub sibtypetobeupdatedlist: GnbCuSystemInformationSibtypetobeupdatedlist,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GnbCuSystemInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct GnbDuCellResourceConfiguration {
    pub subcarrier_spacing: SubcarrierSpacing,
    #[asn(optional_idx = 0)]
    pub duf_transmission_periodicity: Option<DufTransmissionPeriodicity>,
    #[asn(optional_idx = 1)]
    pub duf_slot_config_list: Option<DufSlotConfigList>,
    pub hsna_transmission_periodicity: HsnaTransmissionPeriodicity,
    #[asn(optional_idx = 2)]
    pub hnsa_slot_config_list: Option<HsnaSlotConfigList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<GnbDuCellResourceConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "68719476735")]
pub struct GnbDuId(pub u64);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct GnbDuName(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct GnbDuNameUtf8String(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "VisibleString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct GnbDuNameVisibleString(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GnbDuServedCellsItem {
    pub served_cell_information: ServedCellInformation,
    #[asn(optional_idx = 0)]
    pub gnb_du_system_information: Option<GnbDuSystemInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GnbDuServedCellsItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct GnbDuServedCellsList(pub Vec<GnbDuServedCellsListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GnbDuSystemInformation {
    pub mib_message: MibMessage,
    pub sib1_message: Sib1Message,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GnbDuSystemInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct GnbDuTnlAssociationToRemoveItem {
    pub tnl_association_transport_layer_address: CpTransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub tnl_association_transport_layer_addressg_nbcu: Option<CpTransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GnbDuTnlAssociationToRemoveItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct GnbDuTnlAssociationToRemoveList(pub Vec<GnbDuTnlAssociationToRemoveListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct GnbDuUeF1apId(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GnbDuConfigurationQuery(pub u8);
impl GnbDuConfigurationQuery {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct GnbRxTxTimeDiff {
    pub rx_tx_time_diff: GnbRxTxTimeDiffMeas,
    #[asn(optional_idx = 0)]
    pub additional_path_list: Option<AdditionalPathList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GnbRxTxTimeDiffIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbcuConfigurationUpdate {
    pub protocol_i_es: GnbcuConfigurationUpdateProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbcuConfigurationUpdateAcknowledge {
    pub protocol_i_es: GnbcuConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbcuConfigurationUpdateFailure {
    pub protocol_i_es: GnbcuConfigurationUpdateFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct GnbcuMeasurementId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduConfigurationUpdate {
    pub protocol_i_es: GnbduConfigurationUpdateProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduConfigurationUpdateAcknowledge {
    pub protocol_i_es: GnbduConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduConfigurationUpdateFailure {
    pub protocol_i_es: GnbduConfigurationUpdateFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct GnbduMeasurementId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct GnbduOverloadInformation(pub u8);
impl GnbduOverloadInformation {
    const OVERLOADED: u8 = 0u8;
    const NOT_OVERLOADED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduResourceConfiguration {
    pub protocol_i_es: GnbduResourceConfigurationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduResourceConfigurationAcknowledge {
    pub protocol_i_es: GnbduResourceConfigurationAcknowledgeProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduResourceConfigurationFailure {
    pub protocol_i_es: GnbduResourceConfigurationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduResourceCoordinationRequest {
    pub protocol_i_es: GnbduResourceCoordinationRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduResourceCoordinationResponse {
    pub protocol_i_es: GnbduResourceCoordinationResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GnbduStatusIndication {
    pub protocol_i_es: GnbduStatusIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum GnbRxTxTimeDiffMeas {
    #[asn(key = 0, extended = false)]
    K0(Integer48),
    #[asn(key = 1, extended = false)]
    K1(Integer49),
    #[asn(key = 2, extended = false)]
    K2(Integer50),
    #[asn(key = 3, extended = false)]
    K3(Integer51),
    #[asn(key = 4, extended = false)]
    K4(Integer52),
    #[asn(key = 5, extended = false)]
    K5(Integer53),
    #[asn(key = 6, extended = false)]
    ChoiceExtension(GnbRxTxTimeDiffMeaschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "22")]
pub struct GnbSetId(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GtpTeid(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct GtptlaItem {
    pub gtp_transport_layer_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GtptlaItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GtptlAs(pub Vec<GtptlaItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GtpTunnel {
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GtpTeid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GtpTunnelIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct GeographicalCoordinates {
    pub trp_position_definition_type: TrpPositionDefinitionType,
    #[asn(optional_idx = 0)]
    pub dlprs_resource_coordinates: Option<DlprsResourceCoordinates>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GeographicalCoordinatesIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct HsnaDownlink(pub u8);
impl HsnaDownlink {
    const HARD: u8 = 0u8;
    const SOFT: u8 = 1u8;
    const NOTAVAILABLE: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct HsnaFlexible(pub u8);
impl HsnaFlexible {
    const HARD: u8 = 0u8;
    const SOFT: u8 = 1u8;
    const NOTAVAILABLE: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct HsnaSlotConfigItem {
    #[asn(optional_idx = 0)]
    pub hsna_downlink: Option<HsnaDownlink>,
    #[asn(optional_idx = 1)]
    pub hsna_uplink: Option<HsnaUplink>,
    #[asn(optional_idx = 2)]
    pub hsna_flexible: Option<HsnaFlexible>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<HsnaSlotConfigItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "5120"
)]
pub struct HsnaSlotConfigList(pub Vec<HsnaSlotConfigItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "11")]
pub struct HsnaTransmissionPeriodicity(pub u8);
impl HsnaTransmissionPeriodicity {
    const MS0P5: u8 = 0u8;
    const MS0P625: u8 = 1u8;
    const MS1: u8 = 2u8;
    const MS1P25: u8 = 3u8;
    const MS2: u8 = 4u8;
    const MS2P5: u8 = 5u8;
    const MS5: u8 = 6u8;
    const MS10: u8 = 7u8;
    const MS20: u8 = 8u8;
    const MS40: u8 = 9u8;
    const MS80: u8 = 10u8;
    const MS160: u8 = 11u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct HsnaUplink(pub u8);
impl HsnaUplink {
    const HARD: u8 = 0u8;
    const SOFT: u8 = 1u8;
    const NOTAVAILABLE: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct HandoverPreparationInformation(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HardwareLoadIndicator {
    pub dl_hardware_load_indicator: Integer54,
    pub ul_hardware_load_indicator: Integer55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HardwareLoadIndicatorIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct IabAllocatedTnlAddressItem {
    pub iabtnl_address: IabtnlAddress,
    #[asn(optional_idx = 0)]
    pub iabtnl_address_usage: Option<IabtnlAddressUsage>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<IabAllocatedTnlAddressItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct IabAllocatedTnlAddressList(pub Vec<IabAllocatedTnlAddressListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IabBarred(pub u8);
impl IabBarred {
    const BARRED: u8 = 0u8;
    const NOT_BARRED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IabDuCellResourceConfigurationFddInfo {
    pub gnb_du_cell_resource_configuration_fdd_ul: GnbDuCellResourceConfiguration,
    pub gnb_du_cell_resource_configuration_fdd_dl: GnbDuCellResourceConfiguration,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IabDuCellResourceConfigurationFddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IabDuCellResourceConfigurationModeInfo {
    #[asn(key = 0, extended = false)]
    Fdd(IabDuCellResourceConfigurationFddInfo),
    #[asn(key = 1, extended = false)]
    Tdd(IabDuCellResourceConfigurationTddInfo),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(IabDuCellResourceConfigurationModeInfochoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IabDuCellResourceConfigurationTddInfo {
    pub gnb_du_cell_resourc_configuration_tdd: GnbDuCellResourceConfiguration,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IabDuCellResourceConfigurationTddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct IabInfoIabDu {
    #[asn(optional_idx = 0)]
    pub multiplexing_info: Option<MultiplexingInfo>,
    #[asn(optional_idx = 1)]
    pub iab_stc_info: Option<IabStcInfo>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<IabInfoIabDuIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct IabInfoIabDonorCu {
    #[asn(optional_idx = 0)]
    pub iab_stc_info: Option<IabStcInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<IabInfoIabDonorCuIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct IabMtCellList(pub Vec<IabMtCellListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IabMtCellListItem {
    pub nr_cell_identity: NrCellIdentity,
    pub du_rx_mt_rx: DuRxMtRx,
    pub du_tx_mt_tx: DuTxMtTx,
    pub du_rx_mt_tx: DuRxMtTx,
    pub du_tx_mt_rx: DuTxMtRx,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IabMtCellListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IabStcInfo {
    pub iab_stc_info_list: IabStcInfoList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IabStcInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IabStcInfoItem {
    pub ssb_freq_info: SsbFreqInfo,
    pub ssb_subcarrier_spacing: SsbSubcarrierSpacing,
    pub ssb_transmission_periodicity: SsbTransmissionPeriodicity,
    pub ssb_transmission_timing_offset: SsbTransmissionTimingOffset,
    pub ssb_transmission_bitmap: SsbTransmissionBitmap,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IabStcInfoItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "45")]
pub struct IabStcInfoList(pub Vec<IabStcInfoItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IabTnlAddressesToRemoveItem {
    pub iabtnl_address: IabtnlAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IabTnlAddressesToRemoveItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct IabTnlAddressesToRemoveList(pub Vec<IabTnlAddressesToRemoveListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IabiPv6RequestType {
    #[asn(key = 0, extended = false)]
    IPv6Address(IabtnlAddressesRequested),
    #[asn(key = 1, extended = false)]
    IPv6Prefix(IabtnlAddressesRequested),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(IabiPv6RequestTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum IabtnlAddress {
    #[asn(key = 0, extended = false)]
    IPv4Address(BitString56),
    #[asn(key = 1, extended = false)]
    IPv6Address(BitString57),
    #[asn(key = 2, extended = false)]
    IPv6Prefix(BitString58),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(IabtnlAddresschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct IabtnlAddressFailure {
    pub protocol_i_es: IabtnlAddressFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct IabtnlAddressRequest {
    pub protocol_i_es: IabtnlAddressRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct IabtnlAddressResponse {
    pub protocol_i_es: IabtnlAddressResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct IabtnlAddressUsage(pub u8);
impl IabtnlAddressUsage {
    const F1_C: u8 = 0u8;
    const F1_U: u8 = 1u8;
    const NON_F1: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct IabtnlAddressesRequested {
    #[asn(optional_idx = 0)]
    pub tnl_addresses_or_prefixes_requested_all_traffic: Option<Integer59>,
    #[asn(optional_idx = 1)]
    pub tnl_addresses_or_prefixes_requested_f1_c: Option<Integer60>,
    #[asn(optional_idx = 2)]
    pub tnl_addresses_or_prefixes_requested_f1_u: Option<Integer61>,
    #[asn(optional_idx = 3)]
    pub tnl_addresses_or_prefixes_requested_no_nf1: Option<Integer62>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<IabtnlAddressesRequestedIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct IabupConfigurationUpdateFailure {
    pub protocol_i_es: IabupConfigurationUpdateFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct IabupConfigurationUpdateRequest {
    pub protocol_i_es: IabupConfigurationUpdateRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct IabupConfigurationUpdateResponse {
    pub protocol_i_es: IabupConfigurationUpdateResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IaBv4AddressesRequested {
    pub ia_bv4_addresses_requested: IabtnlAddressesRequested,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IaBv4AddressesRequestedIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct IpHeaderInformation {
    pub destination_iabtnl_address: IabtnlAddress,
    #[asn(optional_idx = 0)]
    pub ds_information_list: Option<DsInformationList>,
    #[asn(optional_idx = 1)]
    pub i_pv6_flow_label: Option<BitString63>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<IpHeaderInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct IPtolayer2TrafficMappingInfo {
    #[asn(optional_idx = 0)]
    pub i_ptolayer2_traffic_mapping_info_to_add: Option<IPtolayer2TrafficMappingInfoList>,
    #[asn(optional_idx = 1)]
    pub i_ptolayer2_traffic_mapping_info_to_remove: Option<MappingInformationtoRemove>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<IPtolayer2TrafficMappingInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IPtolayer2TrafficMappingInfoItem {
    pub mapping_information_index: MappingInformationIndex,
    pub ip_header_information: IpHeaderInformation,
    pub bh_info: BhInfo,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IPtolayer2TrafficMappingInfoItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "67108864"
)]
pub struct IPtolayer2TrafficMappingInfoList(pub Vec<IPtolayer2TrafficMappingInfoItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IgnorePrachConfiguration(pub u8);
impl IgnorePrachConfiguration {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IgnoreResourceCoordinationContainer(pub u8);
impl IgnoreResourceCoordinationContainer {
    const YES: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ImplicitFormat {
    pub duf_slotformat_index: DufSlotformatIndex,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ImplicitFormatIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct InactivityMonitoringRequest(pub u8);
impl InactivityMonitoringRequest {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct InactivityMonitoringResponse(pub u8);
impl InactivityMonitoringResponse {
    const NOT_SUPPORTED: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialUlrrcMessageTransfer {
    pub protocol_i_es: InitialUlrrcMessageTransferProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessageValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct IntendedTddDlUlConfig {
    pub nrscs: Enumerated64,
    pub nrcp: Enumerated65,
    pub nrdlul_tx_periodicity: Enumerated66,
    pub slot_configuration_list: SlotConfigurationList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntendedTddDlUlConfigIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct InterfacesToTrace(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct L139Info {
    pub msg1_scs: Enumerated67,
    #[asn(optional_idx = 0)]
    pub root_sequence_index: Option<Integer68>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<L139InfoIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct L839Info {
    pub root_sequence_index: Integer69,
    pub restricted_set_config: Enumerated70,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<L839InfoIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32", extensible = true)]
pub struct Lcid(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LcsToGcsTranslationAoA {
    pub alpha: Integer71,
    pub beta: Integer72,
    pub gamma: Integer73,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LcsToGcsTranslationAoAIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct LcStoGcsTranslation {
    pub alpha: Integer74,
    #[asn(optional_idx = 0)]
    pub alpha_fine: Option<Integer75>,
    pub beta: Integer76,
    #[asn(optional_idx = 1)]
    pub beta_fine: Option<Integer77>,
    pub gamma: Integer78,
    #[asn(optional_idx = 2)]
    pub gamma_fine: Option<Integer79>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<LcStoGcsTranslationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct LcStoGcsTranslationList(pub Vec<LcStoGcsTranslation>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65536", extensible = true)]
pub struct LmfMeasurementId(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct LmfUeMeasurementId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct LteueSidelinkAggregateMaximumBitrate {
    pub uelte_sidelink_aggregate_maximum_bitrate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LteueSidelinkAggregateMaximumBitrateIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct Ltev2xServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUe>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUe>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Ltev2xServicesAuthorizedIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct LocationUncertainty {
    pub horizontal_uncertainty: Integer80,
    pub horizontal_confidence: Integer81,
    pub vertical_uncertainty: Integer82,
    pub vertical_confidence: Integer83,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LocationUncertaintyIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "19")]
pub struct LongDrxCycleLength(pub u8);
impl LongDrxCycleLength {
    const MS10: u8 = 0u8;
    const MS20: u8 = 1u8;
    const MS32: u8 = 2u8;
    const MS40: u8 = 3u8;
    const MS60: u8 = 4u8;
    const MS64: u8 = 5u8;
    const MS70: u8 = 6u8;
    const MS80: u8 = 7u8;
    const MS128: u8 = 8u8;
    const MS160: u8 = 9u8;
    const MS256: u8 = 10u8;
    const MS320: u8 = 11u8;
    const MS512: u8 = 12u8;
    const MS640: u8 = 13u8;
    const MS1024: u8 = 14u8;
    const MS1280: u8 = 15u8;
    const MS2048: u8 = 16u8;
    const MS2560: u8 = 17u8;
    const MS5120: u8 = 18u8;
    const MS10240: u8 = 19u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct LowerLayerPresenceStatusChange(pub u8);
impl LowerLayerPresenceStatusChange {
    const SUSPEND_LOWER_LAYERS: u8 = 0u8;
    const RESUME_LOWER_LAYERS: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct M2Configuration(pub u8);
impl M2Configuration {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct M5LinksToLog(pub u8);
impl M5LinksToLog {
    const UPLINK: u8 = 0u8;
    const DOWNLINK: u8 = 1u8;
    const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M5Configuration {
    pub m5period: M5period,
    pub m5_links_to_log: M5LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M5ConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct M5period(pub u8);
impl M5period {
    const MS1024: u8 = 0u8;
    const MS2048: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
    const MIN1: u8 = 4u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct M6LinksToLog(pub u8);
impl M6LinksToLog {
    const UPLINK: u8 = 0u8;
    const DOWNLINK: u8 = 1u8;
    const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M6Configuration {
    pub m6report_interval: M6reportInterval,
    pub m6_links_to_log: M6LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M6ConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct M6reportInterval(pub u8);
impl M6reportInterval {
    const MS120: u8 = 0u8;
    const MS240: u8 = 1u8;
    const MS640: u8 = 2u8;
    const MS1024: u8 = 3u8;
    const MS2048: u8 = 4u8;
    const MS5120: u8 = 5u8;
    const MS10240: u8 = 6u8;
    const MS20480: u8 = 7u8;
    const MS40960: u8 = 8u8;
    const MIN1: u8 = 9u8;
    const MIN6: u8 = 10u8;
    const MIN12: u8 = 11u8;
    const MIN30: u8 = 12u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct M7LinksToLog(pub u8);
impl M7LinksToLog {
    const DOWNLINK: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M7Configuration {
    pub m7period: M7period,
    pub m7_links_to_log: M7LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M7ConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60", extensible = true)]
pub struct M7period(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MdtActivation(pub u8);
impl MdtActivation {
    const IMMEDIATE_MDT_ONLY: u8 = 0u8;
    const IMMEDIATE_MDT_AND_TRACE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct MdtConfiguration {
    pub mdt_activation: MdtActivation,
    pub measurements_to_activate: MeasurementsToActivate,
    #[asn(optional_idx = 0)]
    pub m2_configuration: Option<M2Configuration>,
    #[asn(optional_idx = 1)]
    pub m5_configuration: Option<M5Configuration>,
    #[asn(optional_idx = 2)]
    pub m6_configuration: Option<M6Configuration>,
    #[asn(optional_idx = 3)]
    pub m7_configuration: Option<M7Configuration>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<MdtConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MdtplmnList(pub Vec<PlmnIdentity>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MibMessage(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "26", sz_ub = "26")]
pub struct MappingInformationIndex(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "67108864"
)]
pub struct MappingInformationtoRemove(pub Vec<MappingInformationIndex>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct MaskedImeisv(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct MaxDataBurstVolume(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1000")]
pub struct MaxPacketLossRate(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MeasConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MeasGapConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MeasGapSharingConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum MeasuredResultsValue {
    #[asn(key = 0, extended = false)]
    UlAngleOfArrival(UlAoA),
    #[asn(key = 1, extended = false)]
    UlSrsRsrp(UlSrsRsrp),
    #[asn(key = 2, extended = false)]
    UlRtoa(UlRtoaMeasurement),
    #[asn(key = 3, extended = false)]
    GnbRxTxTimeDiff(GnbRxTxTimeDiff),
    #[asn(key = 4, extended = false)]
    ChoiceExtension(MeasuredResultsValuechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct MeasurementBeamInfo {
    #[asn(optional_idx = 0)]
    pub prs_resource_id: Option<PrsResourceId>,
    #[asn(optional_idx = 1)]
    pub prs_resource_set_id: Option<PrsResourceSetId>,
    #[asn(optional_idx = 2)]
    pub ssb_index: Option<SsbIndex>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<MeasurementBeamInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MeasurementBeamInfoRequest(pub u8);
impl MeasurementBeamInfoRequest {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "11")]
pub struct MeasurementPeriodicity(pub u8);
impl MeasurementPeriodicity {
    const MS120: u8 = 0u8;
    const MS240: u8 = 1u8;
    const MS480: u8 = 2u8;
    const MS640: u8 = 3u8;
    const MS1024: u8 = 4u8;
    const MS2048: u8 = 5u8;
    const MS5120: u8 = 6u8;
    const MS10240: u8 = 7u8;
    const MIN1: u8 = 8u8;
    const MIN6: u8 = 9u8;
    const MIN12: u8 = 10u8;
    const MIN30: u8 = 11u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MeasurementTimingConfiguration(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MeasurementsToActivate(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct MessageIdentifier(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct MultiplexingInfo {
    pub iab_mt_cell_list: IabMtCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MultiplexingInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NgranAllocationAndRetentionPriority {
    pub priority_level: PriorityLevel,
    pub pre_emption_capability: PreEmptionCapability,
    pub pre_emption_vulnerability: PreEmptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NgranAllocationAndRetentionPriorityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NgranHighAccuracyAccessPointPosition {
    pub latitude: Integer84,
    pub longitude: Integer85,
    pub altitude: Integer86,
    pub uncertainty_semi_major: Integer87,
    pub uncertainty_semi_minor: Integer88,
    pub orientation_of_major_axis: Integer89,
    pub horizontal_confidence: Integer90,
    pub uncertainty_altitude: Integer91,
    pub vertical_confidence: Integer92,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NgranHighAccuracyAccessPointPositionIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "44", sz_ub = "44")]
pub struct Nid(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NpnBroadcastInformationPniNpn {
    pub broadcast_pni_npn_id_information: BroadcastPniNpnIdList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NpnBroadcastInformationPniNpnIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NpnBroadcastInformationSnpn {
    pub broadcast_snpnid_list: BroadcastSnpnIdList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NpnBroadcastInformationSnpnIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NpnBroadcastInformation {
    #[asn(key = 0, extended = false)]
    SnpnBroadcastInformation(NpnBroadcastInformationSnpn),
    #[asn(key = 1, extended = false)]
    PniNpnBroadcastInformation(NpnBroadcastInformationPniNpn),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(NpnBroadcastInformationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnSupportInfo {
    #[asn(key = 0, extended = false)]
    SnpnInformation(Nid),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(NpnSupportInfochoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrCgiListForRestartItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrCgiListForRestartItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct NrCgiListForRestartList(pub Vec<NrCgiListForRestartListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NrModeInfo {
    #[asn(key = 0, extended = false)]
    Fdd(FddInfo),
    #[asn(key = 1, extended = false)]
    Tdd(TddInfo),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(NrModeInfochoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NrPrsBeamInformation {
    pub nr_prs_beam_information_list: NrPrsBeamInformationList,
    #[asn(optional_idx = 0)]
    pub lc_sto_gcs_translation_list: Option<LcStoGcsTranslationList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NrPrsBeamInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NrPrsBeamInformationItem {
    pub prs_resource_set_id: PrsResourceSetId,
    pub prs_angle_list: PrsAngleList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrPrsBeamInformationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NrPrsBeamInformationList(pub Vec<NrPrsBeamInformationItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Nrcgi {
    pub plmn_identity: PlmnIdentity,
    pub nr_cell_identity: NrCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrcgiIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrCarrierItem {
    pub carrier_scs: Nrscs,
    pub offset_to_carrier: Integer93,
    pub carrier_bandwidth: Integer94,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NrCarrierItemIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "5")]
pub struct NrCarrierList(pub Vec<NrCarrierItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NrCellIdentity(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NrFreqInfo {
    pub nrarfcn: Integer95,
    #[asn(optional_idx = 0)]
    pub sul_information: Option<SulInformation>,
    pub freq_band_list_nr: NrFreqInfoFreqBandListNr,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NrFreqInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "28")]
pub struct Nrnrb(pub u8);
impl Nrnrb {
    const NRB11: u8 = 0u8;
    const NRB18: u8 = 1u8;
    const NRB24: u8 = 2u8;
    const NRB25: u8 = 3u8;
    const NRB31: u8 = 4u8;
    const NRB32: u8 = 5u8;
    const NRB38: u8 = 6u8;
    const NRB51: u8 = 7u8;
    const NRB52: u8 = 8u8;
    const NRB65: u8 = 9u8;
    const NRB66: u8 = 10u8;
    const NRB78: u8 = 11u8;
    const NRB79: u8 = 12u8;
    const NRB93: u8 = 13u8;
    const NRB106: u8 = 14u8;
    const NRB107: u8 = 15u8;
    const NRB121: u8 = 16u8;
    const NRB132: u8 = 17u8;
    const NRB133: u8 = 18u8;
    const NRB135: u8 = 19u8;
    const NRB160: u8 = 20u8;
    const NRB162: u8 = 21u8;
    const NRB189: u8 = 22u8;
    const NRB216: u8 = 23u8;
    const NRB217: u8 = 24u8;
    const NRB245: u8 = 25u8;
    const NRB264: u8 = 26u8;
    const NRB270: u8 = 27u8;
    const NRB273: u8 = 28u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007")]
pub struct Nrpci(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NrprachConfig {
    #[asn(optional_idx = 0)]
    pub ul_prach_config_list: Option<NrprachConfigList>,
    #[asn(optional_idx = 1)]
    pub sul_prach_config_list: Option<NrprachConfigList>,
    #[asn(optional_idx = 2)]
    pub ie_extension: Option<NrprachConfigIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrprachConfigItem {
    pub nrscs: Nrscs,
    pub prach_freq_startfrom_carrier: Integer96,
    pub msg1_fdm: Enumerated97,
    pub parch_config_index: Integer98,
    pub ssb_per_rach_occasion: Enumerated99,
    pub freq_domain_length: FreqDomainLength,
    pub zero_correl_zone_config: Integer100,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NrprachConfigItemIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "16")]
pub struct NrprachConfigList(pub Vec<NrprachConfigItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Nrscs(pub u8);
impl Nrscs {
    const SCS15: u8 = 0u8;
    const SCS30: u8 = 1u8;
    const SCS60: u8 = 2u8;
    const SCS120: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NruerlfReportContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NrueSidelinkAggregateMaximumBitrate {
    pub uenr_sidelink_aggregate_maximum_bitrate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrueSidelinkAggregateMaximumBitrateIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct Nrv2xServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUe>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUe>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Nrv2xServicesAuthorizedIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "191")]
pub struct NzpCsiRsResourceId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NeedforGap(pub u8);
impl NeedforGap {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NeighbourCellInformationItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub intended_tdd_dl_ul_config: Option<IntendedTddDlUlConfig>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NeighbourCellInformationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct NeighbourCellInformationList(pub Vec<NeighbourCellInformationListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NetworkAccessRateReduction {
    pub protocol_i_es: NetworkAccessRateReductionProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct NonDynamic5QiDescriptor {
    pub five_qi: Integer101,
    #[asn(optional_idx = 0)]
    pub qo_s_priority_level: Option<Integer102>,
    #[asn(optional_idx = 1)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 2)]
    pub max_data_burst_volume: Option<MaxDataBurstVolume>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<NonDynamic5QiDescriptorIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct NonDynamicPqiDescriptor {
    pub five_qi: Integer103,
    #[asn(optional_idx = 0)]
    pub qo_s_priority_level: Option<Integer104>,
    #[asn(optional_idx = 1)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 2)]
    pub max_data_burst_volume: Option<MaxDataBurstVolume>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<NonDynamicPqiDescriptorIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NonUpTrafficType(pub u8);
impl NonUpTrafficType {
    const UE_ASSOCIATED: u8 = 0u8;
    const NON_UE_ASSOCIATED: u8 = 1u8;
    const NON_F1: u8 = 2u8;
    const BAP_CONTROL_PDU: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "14")]
pub struct NoofDownlinkSymbols(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "14")]
pub struct NoofUplinkSymbols(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NotificationCause(pub u8);
impl NotificationCause {
    const FULFILLED: u8 = 0u8;
    const NOT_FULFILLED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NotificationControl(pub u8);
impl NotificationControl {
    const ACTIVE: u8 = 0u8;
    const NOT_ACTIVE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NotificationInformation {
    pub message_identifier: MessageIdentifier,
    pub serial_number: SerialNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NotificationInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Notify {
    pub protocol_i_es: NotifyProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct NumDlulSymbols {
    pub num_dl_symbols: Integer105,
    pub num_ul_symbols: Integer106,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NumDlulSymbolsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberOfBroadcasts(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215", extensible = true)]
pub struct NumberofActiveUEs(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberofBroadcastRequest(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2199", extensible = true)]
pub struct OffsetToPointA(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum Pc5QoSCharacteristics {
    #[asn(key = 0, extended = false)]
    NonDynamicPqi(NonDynamicPqiDescriptor),
    #[asn(key = 1, extended = false)]
    DynamicPqi(DynamicPqiDescriptor),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(Pc5QoSCharacteristicschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Pc5FlowBitRates {
    pub guaranteed_flow_bit_rate: BitRate,
    pub maximum_flow_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Pc5FlowBitRatesIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "2048")]
pub struct Pc5QoSFlowIdentifier(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Pc5QoSParameters {
    pub pc5_qo_s_characteristics: Pc5QoSCharacteristics,
    #[asn(optional_idx = 0)]
    pub pc5_qo_s_flow_bit_rates: Option<Pc5FlowBitRates>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Pc5QoSParametersIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct PdcchBlindDetectionScg(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct PdcpSn(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PdcpsnLength(pub u8);
impl PdcpsnLength {
    const TWELVE_BITS: u8 = 0u8;
    const EIGHTEEN_BITS: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PduSessionId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct PerExponent(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct PerScalar(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PlmnIdentity(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct PrsResourceId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct PrsResourceSetId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PrsAngleItem {
    pub nr_prs_azimuth: Integer107,
    pub nr_prs_azimuth_fine: Integer108,
    pub nr_prs_elevation: Integer109,
    pub nr_prs_elevation_fine: Integer110,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PrsAngleItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct PrsAngleList(pub Vec<PrsAngleItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PrsConfiguration {
    pub prs_resource_set_list: PrsResourceSetList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PrsConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PrsInformationPos {
    pub prs_id_pos: Integer111,
    pub prs_resource_set_id_pos: Integer112,
    #[asn(optional_idx = 0)]
    pub prs_resource_id_pos: Option<Integer113>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PrsInformationPosIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PrsMuting {
    pub prs_muting_option1: PrsMutingOption1,
    pub prs_muting_option2: PrsMutingOption2,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PrsMutingIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PrsMutingOption1 {
    pub muting_pattern: DlPrsMutingPattern,
    pub muting_bit_repetition_factor: Enumerated114,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PrsMutingOption1IeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PrsMutingOption2 {
    pub muting_pattern: DlPrsMutingPattern,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PrsMutingOption2IeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PrsResourceItem {
    pub prs_resource_id: PrsResourceId,
    pub sequence_id: Integer115,
    pub re_offset: Integer116,
    pub resource_slot_offset: Integer117,
    pub resource_symbol_offset: Integer118,
    #[asn(optional_idx = 0)]
    pub qcl_info: Option<PrsResourceQclInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PrsResourceItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct PrsResourceList(pub Vec<PrsResourceItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PrsResourceQclInfo {
    #[asn(key = 0, extended = false)]
    QclSourceSsb(PrsResourceQclSourceSsb),
    #[asn(key = 1, extended = false)]
    QclSourcePrs(PrsResourceQclSourcePrs),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(PrsResourceQclInfochoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PrsResourceQclSourcePrs {
    pub qcl_source_prs_resource_set_id: PrsResourceSetId,
    #[asn(optional_idx = 0)]
    pub qcl_source_prs_resource_id: Option<PrsResourceId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PrsResourceQclSourcePrsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PrsResourceQclSourceSsb {
    pub pci_nr: Integer119,
    #[asn(optional_idx = 0)]
    pub ssb_index: Option<SsbIndex>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PrsResourceQclSourceSsbIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PrsResourceSetItem {
    pub prs_resource_set_id: PrsResourceSetId,
    pub subcarrier_spacing: Enumerated120,
    pub pr_sbandwidth: Integer121,
    pub start_prb: Integer122,
    pub point_a: Integer123,
    pub comb_size: Enumerated124,
    pub cp_type: Enumerated125,
    pub resource_set_periodicity: Enumerated126,
    pub resource_set_slot_offset: Integer127,
    pub resource_repetition_factor: Enumerated128,
    pub resource_time_gap: Enumerated129,
    pub resource_numberof_symbols: Enumerated130,
    #[asn(optional_idx = 0)]
    pub prs_muting: Option<PrsMuting>,
    pub prs_resource_transmit_power: Integer131,
    pub prs_resource_list: PrsResourceList,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PrsResourceSetItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct PrsResourceSetList(pub Vec<PrsResourceSetItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PwsFailedNrCgiItem {
    pub nrcgi: Nrcgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PwsFailedNrCgiItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct PwsFailedNrCgiList(pub Vec<PwsFailedNrCgiListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsCancelRequest {
    pub protocol_i_es: PwsCancelRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsCancelResponse {
    pub protocol_i_es: PwsCancelResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsFailureIndication {
    pub protocol_i_es: PwsFailureIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsRestartIndication {
    pub protocol_i_es: PwsRestartIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PwsSystemInformation {
    pub si_btype: SibTypePws,
    pub si_bmessage: OctetString132,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PwsSystemInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1023", extensible = true)]
pub struct PacketDelayBudget(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PacketErrorRate {
    pub per_scalar: PerScalar,
    pub per_exponent: PerExponent,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PacketErrorRateIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Paging {
    pub protocol_i_es: PagingProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PagingCellItem {
    pub nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PagingCellItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct PagingCellList(pub Vec<PagingCellListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct PagingDrx(pub u8);
impl PagingDrx {
    const V32: u8 = 0u8;
    const V64: u8 = 1u8;
    const V128: u8 = 2u8;
    const V256: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PagingIdentity {
    #[asn(key = 0, extended = false)]
    RanuePagingIdentity(RanuePagingIdentity),
    #[asn(key = 1, extended = false)]
    CnuePagingIdentity(CnuePagingIdentity),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(PagingIdentitychoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PagingOrigin(pub u8);
impl PagingOrigin {
    const NON_3GPP: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct PagingPriority(pub u8);
impl PagingPriority {
    const PRIOLEVEL1: u8 = 0u8;
    const PRIOLEVEL2: u8 = 1u8;
    const PRIOLEVEL3: u8 = 2u8;
    const PRIOLEVEL4: u8 = 3u8;
    const PRIOLEVEL5: u8 = 4u8;
    const PRIOLEVEL6: u8 = 5u8;
    const PRIOLEVEL7: u8 = 6u8;
    const PRIOLEVEL8: u8 = 7u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PathlossReferenceInfo {
    pub pathloss_reference_signal: PathlossReferenceSignal,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PathlossReferenceInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PathlossReferenceSignal {
    #[asn(key = 0, extended = false)]
    Ssb(Ssb),
    #[asn(key = 1, extended = false)]
    DlPrs(DlPrs),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(PathlossReferenceSignalchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PedestrianUe(pub u8);
impl PedestrianUe {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "640000", extensible = true)]
pub struct Periodicity(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PeriodicityList(pub Vec<PeriodicityListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PeriodicityListItem {
    pub periodicity_srs: PeriodicitySrs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PeriodicityListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "24")]
pub struct PeriodicitySrs(pub u8);
impl PeriodicitySrs {
    const MS0P125: u8 = 0u8;
    const MS0P25: u8 = 1u8;
    const MS0P5: u8 = 2u8;
    const MS0P625: u8 = 3u8;
    const MS1: u8 = 4u8;
    const MS1P25: u8 = 5u8;
    const MS2: u8 = 6u8;
    const MS2P5: u8 = 7u8;
    const MS4: u8 = 8u8;
    const MS5: u8 = 9u8;
    const MS8: u8 = 10u8;
    const MS10: u8 = 11u8;
    const MS16: u8 = 12u8;
    const MS20: u8 = 13u8;
    const MS32: u8 = 14u8;
    const MS40: u8 = 15u8;
    const MS64: u8 = 16u8;
    const MS80: u8 = 17u8;
    const MS160: u8 = 18u8;
    const MS320: u8 = 19u8;
    const MS640: u8 = 20u8;
    const MS1280: u8 = 21u8;
    const MS2560: u8 = 22u8;
    const MS5120: u8 = 23u8;
    const MS10240: u8 = 24u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Permutation(pub u8);
impl Permutation {
    const DFU: u8 = 0u8;
    const UFD: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct PhInfoMcg(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct PhInfoScg(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct PortNumber(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct PosAssistanceInformation(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct PosAssistanceInformationFailureList(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PosBroadcast(pub u8);
impl PosBroadcast {
    const START: u8 = 0u8;
    const STOP: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "16384"
)]
pub struct PosMeasurementQuantities(pub Vec<PosMeasurementQuantitiesItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PosMeasurementQuantitiesItem {
    pub pos_measurement_type: PosMeasurementType,
    #[asn(optional_idx = 0)]
    pub timing_reporting_granularity_factor: Option<Integer133>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PosMeasurementQuantitiesItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "16384"
)]
pub struct PosMeasurementResult(pub Vec<PosMeasurementResultItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct PosMeasurementResultItem {
    pub measured_results_value: MeasuredResultsValue,
    pub time_stamp: TimeStamp,
    #[asn(optional_idx = 0)]
    pub measurement_quality: Option<TrpMeasurementQuality>,
    #[asn(optional_idx = 1)]
    pub measurement_beam_info: Option<MeasurementBeamInfo>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PosMeasurementResultItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct PosMeasurementResultList(pub Vec<PosMeasurementResultListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PosMeasurementResultListItem {
    pub pos_measurement_result: PosMeasurementResult,
    pub trpid: Trpid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PosMeasurementResultListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct PosMeasurementType(pub u8);
impl PosMeasurementType {
    const GNB_RX_TX: u8 = 0u8;
    const UL_SRS_RSRP: u8 = 1u8;
    const UL_AOA: u8 = 2u8;
    const UL_RTOA: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PosReportCharacteristics(pub u8);
impl PosReportCharacteristics {
    const ONDEMAND: u8 = 0u8;
    const PERIODIC: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum PosResourceSetType {
    #[asn(key = 0, extended = false)]
    Periodic(PosResourceSetTypePr),
    #[asn(key = 1, extended = false)]
    SemiPersistent(PosResourceSetTypeSp),
    #[asn(key = 2, extended = false)]
    Aperiodic(PosResourceSetTypeAp),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(PosResourceSetTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PosResourceSetTypeAp {
    pub srs_resource_trigger_list: Integer134,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PosResourceSetTypeApIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PosResourceSetTypePr {
    pub posperiodic_set: Enumerated135,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PosResourceSetTypePrIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PosResourceSetTypeSp {
    pub possemi_persistent_set: Enumerated136,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PosResourceSetTypeSpIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PosSrsResourceItem {
    pub srs_pos_resource_id: SrsPosResourceId,
    pub transmission_comb_pos: TransmissionCombPos,
    pub start_position: Integer137,
    pub nrof_symbols: Enumerated138,
    pub freq_domain_shift: Integer139,
    pub c_srs: Integer140,
    pub group_or_sequence_hopping: Enumerated141,
    pub resource_type_pos: ResourceTypePos,
    pub sequence_id: Integer142,
    #[asn(optional_idx = 0)]
    pub spatial_relation_pos: Option<SpatialRelationPos>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PosSrsResourceItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct PosSrsResourceList(pub Vec<PosSrsResourceItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PosSrsResourceIdList(pub Vec<SrsPosResourceId>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PosSrsResourceSetItem {
    pub possrs_resource_set_id: Integer143,
    pub poss_rs_resource_id_list: PosSrsResourceIdList,
    pub posresource_set_type: PosResourceSetType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PosSrsResourceSetItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PosSrsResourceSetList(pub Vec<PosSrsResourceSetItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningActivationFailure {
    pub protocol_i_es: PositioningActivationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningActivationRequest {
    pub protocol_i_es: PositioningActivationRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningActivationResponse {
    pub protocol_i_es: PositioningActivationResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningAssistanceInformationControl {
    pub protocol_i_es: PositioningAssistanceInformationControlProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningAssistanceInformationFeedback {
    pub protocol_i_es: PositioningAssistanceInformationFeedbackProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "16384"
)]
pub struct PositioningBroadcastCells(pub Vec<Nrcgi>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningDeactivation {
    pub protocol_i_es: PositioningDeactivationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningInformationFailure {
    pub protocol_i_es: PositioningInformationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningInformationRequest {
    pub protocol_i_es: PositioningInformationRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningInformationResponse {
    pub protocol_i_es: PositioningInformationResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningInformationUpdate {
    pub protocol_i_es: PositioningInformationUpdateProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementAbort {
    pub protocol_i_es: PositioningMeasurementAbortProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementFailure {
    pub protocol_i_es: PositioningMeasurementFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementFailureIndication {
    pub protocol_i_es: PositioningMeasurementFailureIndicationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementReport {
    pub protocol_i_es: PositioningMeasurementReportProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementRequest {
    pub protocol_i_es: PositioningMeasurementRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementResponse {
    pub protocol_i_es: PositioningMeasurementResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningMeasurementUpdate {
    pub protocol_i_es: PositioningMeasurementUpdateProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PotentialSpCellItem {
    pub potential_sp_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PotentialSpCellItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "64")]
pub struct PotentialSpCellList(pub Vec<PotentialSpCellListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PreEmptionCapability(pub u8);
impl PreEmptionCapability {
    const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PreEmptionVulnerability(pub u8);
impl PreEmptionVulnerability {
    const NOT_PRE_EMPTABLE: u8 = 0u8;
    const PRE_EMPTABLE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Presence(pub u8);
impl Presence {
    const OPTIONAL: u8 = 0u8;
    const CONDITIONAL: u8 = 1u8;
    const MANDATORY: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PrimaryPathIndication(pub u8);
impl PrimaryPathIndication {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct PriorityLevel(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PrivacyIndicator(pub u8);
impl PrivacyIndicator {
    const IMMEDIATE_MDT: u8 = 0u8;
    const LOGGED_MDT: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum PrivateIeId {
    #[asn(key = 0, extended = false)]
    Local(Integer144),
    #[asn(key = 1, extended = false)]
    Global(ObjectIdentifier145),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PrivateMessage {
    pub private_i_es: PrivateMessagePrivateIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ProcedureCode(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ProtectedEutraResourcesItem {
    pub spectrum_sharing_group_id: SpectrumSharingGroupId,
    pub eutra_cells_list: EutraCellsList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ProtectedEutraResourcesItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ProtectedEutraResourcesList(pub Vec<ProtectedEutraResourcesListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct ProtectedEutraResourceIndication(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIeId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Qci(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum QoSCharacteristics {
    #[asn(key = 0, extended = false)]
    NonDynamic5qi(NonDynamic5QiDescriptor),
    #[asn(key = 1, extended = false)]
    Dynamic5qi(Dynamic5QiDescriptor),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(QoSCharacteristicschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct QoSFlowIdentifier(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct QoSFlowLevelQoSParameters {
    pub qo_s_characteristics: QoSCharacteristics,
    pub ngra_nallocation_retention_priority: NgranAllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qo_s_flow_information: Option<GbrQoSFlowInformation>,
    #[asn(optional_idx = 1)]
    pub reflective_qo_s_attribute: Option<Enumerated146>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<QoSFlowLevelQoSParametersIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct QoSFlowMappingIndication(pub u8);
impl QoSFlowMappingIndication {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum QoSInformation {
    #[asn(key = 0, extended = false)]
    EutranQoS(EutranQoS),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(QoSInformationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8", extensible = true)]
pub struct QoSParaSetIndex(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "8", extensible = true)]
pub struct QoSParaSetNotifyIndex(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct QosMonitoringRequest(pub u8);
impl QosMonitoringRequest {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RachConfigCommon(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RachConfigCommonIab(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RachReportContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RachReportInformationItem {
    pub rach_report_container: RachReportContainer,
    #[asn(optional_idx = 0)]
    pub ue_assitant_identifier: Option<GnbDuUeF1apId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RachReportInformationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct RachReportInformationList(pub Vec<RachReportInformationItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65536", extensible = true)]
pub struct RanMeasurementId(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct RanUeMeasurementId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Ranac(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct Ranueid(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RanuePagingIdentity {
    pub irnti: BitString147,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RanuePagingIdentityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum RatFrequencyPriorityInformation {
    #[asn(key = 0, extended = false)]
    Endc(SubscriberProfileIDforRfp),
    #[asn(key = 1, extended = false)]
    Ngran(RatFrequencySelectionPriority),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(RatFrequencyPriorityInformationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct RatFrequencySelectionPriority(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RimrsDetectionStatus(pub u8);
impl RimrsDetectionStatus {
    const RS_DETECTED: u8 = 0u8;
    const RS_DISAPPEARED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RlcStatus {
    pub reestablishment_indication: ReestablishmentIndication,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RlcStatusIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct RlcDuplicationInformation {
    pub rlc_duplication_state_list: RlcDuplicationStateList,
    #[asn(optional_idx = 0)]
    pub primary_path_indication: Option<PrimaryPathIndication>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RlcDuplicationInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RlcDuplicationStateItem {
    pub duplication_state: DuplicationState,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RlcDuplicationStateItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct RlcDuplicationStateList(pub Vec<RlcDuplicationStateItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RlcFailureIndication {
    pub assocated_lcid: Lcid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RlcFailureIndicationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RlcMode(pub u8);
impl RlcMode {
    const RLC_AM: u8 = 0u8;
    const RLC_UM_BIDIRECTIONAL: u8 = 1u8;
    const RLC_UM_UNIDIRECTIONAL_UL: u8 = 2u8;
    const RLC_UM_UNIDIRECTIONAL_DL: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RlfReportInformationItem {
    pub nruerlf_report_container: NruerlfReportContainer,
    #[asn(optional_idx = 0)]
    pub ue_assitant_identifier: Option<GnbDuUeF1apId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RlfReportInformationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct RlfReportInformationList(pub Vec<RlfReportInformationItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RrcVersion {
    pub latest_rrc_version: BitString148,

    // Extensions
    pub latest_rrc_version_enhanced: Option<[u8; 3]>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RrcContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RrcContainerRrcSetupComplete(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RrcDeliveryReport {
    pub protocol_i_es: RrcDeliveryReportProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RrcDeliveryStatus {
    pub delivery_status: PdcpSn,
    pub triggering_message: PdcpSn,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RrcDeliveryStatusIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RrcDeliveryStatusRequest(pub u8);
impl RrcDeliveryStatusRequest {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RrcReconfigurationCompleteIndicator(pub u8);
impl RrcReconfigurationCompleteIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RadioResourceStatus {
    pub ssb_area_radio_resource_status_list: SsbAreaRadioResourceStatusList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RadioResourceStatusIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReestablishmentIndication(pub u8);
impl ReestablishmentIndication {
    const REESTABLISHED: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ReferencePoint {
    #[asn(key = 0, extended = false)]
    CoordinateId(CoordinateId),
    #[asn(key = 1, extended = false)]
    ReferencePointCoordinate(AccessPointPosition),
    #[asn(key = 2, extended = false)]
    ReferencePointCoordinateHa(NgranHighAccuracyAccessPointPosition),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(ReferencePointchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct ReferenceSfn(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = false)]
pub enum ReferenceSignal {
    #[asn(key = 0, extended = false)]
    NzpCsiRs(NzpCsiRsResourceId),
    #[asn(key = 1, extended = false)]
    Ssb(Ssb),
    #[asn(key = 2, extended = false)]
    Srs(SrsResourceId),
    #[asn(key = 3, extended = false)]
    PositioningSrs(SrsPosResourceId),
    #[asn(key = 4, extended = false)]
    DlPrs(DlPrs),
    #[asn(key = 5, extended = false)]
    ChoiceExtension(ReferenceSignalchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct ReferenceTime(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ReferenceTimeInformationReport {
    pub protocol_i_es: ReferenceTimeInformationReportProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ReferenceTimeInformationReportingControl {
    pub protocol_i_es: ReferenceTimeInformationReportingControlProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct RegistrationRequest(pub u8);
impl RegistrationRequest {
    const START: u8 = 0u8;
    const STOP: u8 = 1u8;
    const ADD: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RelativeCartesianLocation {
    pub xy_zunit: Enumerated149,
    pub xvalue: Integer150,
    pub yvalue: Integer151,
    pub zvalue: Integer152,
    pub location_uncertainty: LocationUncertainty,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RelativeCartesianLocationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct RelativeGeodeticLocation {
    pub milli_arc_second_units: Enumerated153,
    pub height_units: Enumerated154,
    pub delta_latitude: Integer155,
    pub delta_longitude: Integer156,
    pub delta_height: Integer157,
    pub location_uncertainty: LocationUncertainty,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RelativeGeodeticLocationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum RelativePathDelay {
    #[asn(key = 0, extended = false)]
    K0(Integer158),
    #[asn(key = 1, extended = false)]
    K1(Integer159),
    #[asn(key = 2, extended = false)]
    K2(Integer160),
    #[asn(key = 3, extended = false)]
    K3(Integer161),
    #[asn(key = 4, extended = false)]
    K4(Integer162),
    #[asn(key = 5, extended = false)]
    K5(Integer163),
    #[asn(key = 6, extended = false)]
    ChoiceExtension(RelativePathDelaychoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct RelativeTime1900(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "131071", extensible = true)]
pub struct RepetitionPeriod(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct ReportCharacteristics(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct ReportingPeriodicity(pub u8);
impl ReportingPeriodicity {
    const MS500: u8 = 0u8;
    const MS1000: u8 = 1u8;
    const MS2000: u8 = 2u8;
    const MS5000: u8 = 3u8;
    const MS10000: u8 = 4u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "512", extensible = true)]
pub struct ReportingPeriodicityValue(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct ReportingRequestType {
    pub event_type: EventType,
    #[asn(optional_idx = 0)]
    pub reporting_periodicity_value: Option<ReportingPeriodicityValue>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ReportingRequestTypeIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RequestType(pub u8);
impl RequestType {
    const OFFER: u8 = 0u8;
    const EXECUTION: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RequestedPdcchBlindDetectionScg(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RequestedBandCombinationIndex(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RequestedFeatureSetEntryIndex(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RequestedPMaxFr2(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct RequestedSrsTransmissionCharacteristics {
    #[asn(optional_idx = 0)]
    pub number_of_transmissions: Option<Integer164>,
    pub resource_type: Enumerated165,
    pub bandwidth_srs: BandwidthSrs,
    #[asn(optional_idx = 1)]
    pub srs_resource_set_list: Option<SrsResourceSetList>,
    #[asn(optional_idx = 2)]
    pub ssb_information: Option<SsbInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<RequestedSrsTransmissionCharacteristicsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Reset {
    pub protocol_i_es: ResetProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ResetAll(pub u8);
impl ResetAll {
    const RESET_ALL: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum ResetType {
    #[asn(key = 0, extended = false)]
    F1Interface(ResetAll),
    #[asn(key = 1, extended = false)]
    PartOfF1Interface(UeAssociatedLogicalF1ConnectionListRes),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(ResetTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResourceCoordinationEutraCellInfo {
    pub eutra_mode_info: EutraCoexModeInfo,
    pub eutra_prach_configuration: EutraPrachConfiguration,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceCoordinationEutraCellInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct ResourceCoordinationTransferContainer(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ResourceCoordinationTransferInformation {
    pub me_nb_cell_id: EutraCellId,
    #[asn(optional_idx = 0)]
    pub resource_coordination_eutra_cell_info: Option<ResourceCoordinationEutraCellInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ResourceCoordinationTransferInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ResourceSetType {
    #[asn(key = 0, extended = false)]
    Periodic(ResourceSetTypePeriodic),
    #[asn(key = 1, extended = false)]
    SemiPersistent(ResourceSetTypeSemiPersistent),
    #[asn(key = 2, extended = false)]
    Aperiodic(ResourceSetTypeAperiodic),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(ResourceSetTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceSetTypeAperiodic {
    pub srs_resource_trigger_list: Integer166,
    pub slotoffset: Integer167,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceSetTypeAperiodicIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceSetTypePeriodic {
    pub periodic_set: Enumerated168,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceSetTypePeriodicIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceSetTypeSemiPersistent {
    pub semi_persistent_set: Enumerated169,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceSetTypeSemiPersistentIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResourceStatusFailure {
    pub protocol_i_es: ResourceStatusFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResourceStatusRequest {
    pub protocol_i_es: ResourceStatusRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResourceStatusResponse {
    pub protocol_i_es: ResourceStatusResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResourceStatusUpdate {
    pub protocol_i_es: ResourceStatusUpdateProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ResourceType {
    #[asn(key = 0, extended = false)]
    Periodic(ResourceTypePeriodic),
    #[asn(key = 1, extended = false)]
    SemiPersistent(ResourceTypeSemiPersistent),
    #[asn(key = 2, extended = false)]
    Aperiodic(ResourceTypeAperiodic),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(ResourceTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceTypeAperiodic {
    pub aperiodic_resource_type: Enumerated170,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceTypeAperiodicIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceTypeAperiodicPos {
    pub slot_offset: Integer171,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceTypeAperiodicPosIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceTypePeriodic {
    pub periodicity: Enumerated172,
    pub offset: Integer173,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceTypePeriodicIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceTypePeriodicPos {
    pub periodicity: Enumerated174,
    pub offset: Integer175,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceTypePeriodicPosIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ResourceTypePos {
    #[asn(key = 0, extended = false)]
    Periodic(ResourceTypePeriodicPos),
    #[asn(key = 1, extended = false)]
    SemiPersistent(ResourceTypeSemiPersistentPos),
    #[asn(key = 2, extended = false)]
    Aperiodic(ResourceTypeAperiodicPos),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(ResourceTypePoschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceTypeSemiPersistent {
    pub periodicity: Enumerated176,
    pub offset: Integer177,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceTypeSemiPersistentIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ResourceTypeSemiPersistentPos {
    pub periodicity: Enumerated178,
    pub offset: Integer179,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResourceTypeSemiPersistentPosIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RoutingId(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ScgIndicator(pub u8);
impl ScgIndicator {
    const RELEASED: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ScsSpecificCarrier {
    pub offset_to_carrier: Integer180,
    pub subcarrier_spacing: Enumerated181,
    pub carrier_bandwidth: Integer182,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ScsSpecificCarrierIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SCellFailedtoSetupItem {
    pub s_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SCellFailedtoSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SCellFailedtoSetupList(pub Vec<SCellFailedtoSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SCellFailedtoSetupModItem {
    pub s_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SCellFailedtoSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SCellFailedtoSetupModList(pub Vec<SCellFailedtoSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SCellToBeRemovedItem {
    pub s_cell_id: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SCellToBeRemovedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SCellToBeRemovedList(pub Vec<SCellToBeRemovedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SCellToBeSetupItem {
    pub s_cell_id: Nrcgi,
    pub s_cell_index: SCellIndex,
    #[asn(optional_idx = 0)]
    pub s_cell_ul_configured: Option<CellUlConfigured>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SCellToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SCellToBeSetupList(pub Vec<SCellToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SCellToBeSetupModItem {
    pub s_cell_id: Nrcgi,
    pub s_cell_index: SCellIndex,
    #[asn(optional_idx = 0)]
    pub s_cell_ul_configured: Option<CellUlConfigured>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SCellToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SCellToBeSetupModList(pub Vec<SCellToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "31", extensible = true)]
pub struct SCellIndex(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SfnOffset {
    pub sfn_time_offset: BitString183,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SfnOffsetIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Sib1Message(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Sib10Message(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Sib12Message(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Sib13Message(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Sib14Message(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "6", ub = "8", extensible = true)]
pub struct SibTypePws(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32", extensible = true)]
pub struct SItype(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SItypeItem {
    pub s_itype: SItype,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SItypeItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SItypeList(pub Vec<SItypeItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SlConfigDedicatedEutraInfo(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SlPhyMacRlcConfig(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "512", extensible = true)]
pub struct Sldrbid(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SldrbInformation {
    pub sldrb_qo_s: Pc5QoSParameters,
    pub flows_mapped_to_sldrb_list: FlowsMappedToSldrbList,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct SldrBsFailedToBeModifiedItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SldrBsFailedToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsFailedToBeModifiedList(pub Vec<SldrBsFailedToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct SldrBsFailedToBeSetupItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SldrBsFailedToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsFailedToBeSetupList(pub Vec<SldrBsFailedToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct SldrBsFailedToBeSetupModItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SldrBsFailedToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsFailedToBeSetupModList(pub Vec<SldrBsFailedToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsModifiedItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsModifiedList(pub Vec<SldrBsModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsModifiedConfItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsModifiedConfItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsModifiedConfList(pub Vec<SldrBsModifiedConfListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsRequiredToBeModifiedItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsRequiredToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsRequiredToBeModifiedList(pub Vec<SldrBsRequiredToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsRequiredToBeReleasedItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsRequiredToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsRequiredToBeReleasedList(pub Vec<SldrBsRequiredToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsSetupItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsSetupList(pub Vec<SldrBsSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsSetupModItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsSetupModList(pub Vec<SldrBsSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct SldrBsToBeModifiedItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub sldrb_information: Option<SldrbInformation>,
    #[asn(optional_idx = 1)]
    pub rlc_mode: Option<RlcMode>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SldrBsToBeModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsToBeModifiedList(pub Vec<SldrBsToBeModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsToBeReleasedItem {
    pub sldrbid: Sldrbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsToBeReleasedList(pub Vec<SldrBsToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SldrBsToBeSetupItem {
    pub sldrbid: Sldrbid,
    pub sldrb_information: SldrbInformation,
    pub rlc_mode: RlcMode,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SldrBsToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsToBeSetupList(pub Vec<SldrBsToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct SldrBsToBeSetupModItem {
    pub sldrbid: Sldrbid,
    pub sldrb_information: SldrbInformation,
    #[asn(optional_idx = 0)]
    pub rlc_mode: Option<RlcMode>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SldrBsToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct SldrBsToBeSetupModList(pub Vec<SldrBsToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct Snssai {
    pub sst: OctetString184,
    #[asn(optional_idx = 0)]
    pub sd: Option<OctetString185>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SnssaiIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SnssaiItem {
    pub snssai: Snssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SnssaiItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct SnssaiList(pub Vec<SnssaiItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct SnssaiAvailableCapacityItem {
    pub snssai: Snssai,
    #[asn(optional_idx = 0)]
    pub slice_available_capacity_value_downlink: Option<Integer186>,
    #[asn(optional_idx = 1)]
    pub slice_available_capacity_value_uplink: Option<Integer187>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SnssaiAvailableCapacityItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct SnssaiAvailableCapacityList(pub Vec<SnssaiAvailableCapacityItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3", extensible = true)]
pub struct Srbid(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SrBsFailedToBeSetupItem {
    pub srbid: Srbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SrBsFailedToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsFailedToBeSetupList(pub Vec<SrBsFailedToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SrBsFailedToBeSetupModItem {
    pub srbid: Srbid,
    #[asn(optional_idx = 0)]
    pub cause: Option<Cause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SrBsFailedToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsFailedToBeSetupModList(pub Vec<SrBsFailedToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SrBsModifiedItem {
    pub srbid: Srbid,
    pub lcid: Lcid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrBsModifiedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsModifiedList(pub Vec<SrBsModifiedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SrBsRequiredToBeReleasedItem {
    pub srbid: Srbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrBsRequiredToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsRequiredToBeReleasedList(pub Vec<SrBsRequiredToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SrBsSetupItem {
    pub srbid: Srbid,
    pub lcid: Lcid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrBsSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsSetupList(pub Vec<SrBsSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SrBsSetupModItem {
    pub srbid: Srbid,
    pub lcid: Lcid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrBsSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsSetupModList(pub Vec<SrBsSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SrBsToBeReleasedItem {
    pub srbid: Srbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrBsToBeReleasedItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsToBeReleasedList(pub Vec<SrBsToBeReleasedListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SrBsToBeSetupItem {
    pub srbid: Srbid,
    #[asn(optional_idx = 0)]
    pub duplication_indication: Option<DuplicationIndication>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SrBsToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsToBeSetupList(pub Vec<SrBsToBeSetupListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SrBsToBeSetupModItem {
    pub srbid: Srbid,
    #[asn(optional_idx = 0)]
    pub duplication_indication: Option<DuplicationIndication>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SrBsToBeSetupModItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SrBsToBeSetupModList(pub Vec<SrBsToBeSetupModListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SrsCarrierList(pub Vec<SrsCarrierListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct SrsCarrierListItem {
    pub point_a: Integer188,
    pub uplink_channel_bw_per_scs_list: UplinkChannelBwPerScsList,
    pub active_ulbwp: ActiveUlbwp,
    #[asn(optional_idx = 0)]
    pub pci: Option<Nrpci>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SrsCarrierListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct SrsConfig {
    #[asn(optional_idx = 0)]
    pub srs_resource_list: Option<SrsResourceList>,
    #[asn(optional_idx = 1)]
    pub pos_srs_resource_list: Option<PosSrsResourceList>,
    #[asn(optional_idx = 2)]
    pub srs_resource_set_list: Option<SrsResourceSetList2>, // manually edited
    #[asn(optional_idx = 3)]
    pub pos_srs_resource_set_list: Option<PosSrsResourceSetList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<SrsConfigIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SrsConfiguration {
    pub srs_carrier_list: SrsCarrierList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrsConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct SrsPosResourceId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SrsResource {
    pub srs_resource_id: SrsResourceId,
    pub nrof_srs_ports: Enumerated189,
    pub transmission_comb: TransmissionComb,
    pub start_position: Integer190,
    pub nrof_symbols: Enumerated191,
    pub repetition_factor: Enumerated192,
    pub freq_domain_position: Integer193,
    pub freq_domain_shift: Integer194,
    pub c_srs: Integer195,
    pub b_srs: Integer196,
    pub b_hop: Integer197,
    pub group_or_sequence_hopping: Enumerated198,
    pub resource_type: ResourceType,
    pub sequence_id: Integer199,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrsResourceIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SrsResourceList(pub Vec<SrsResource>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct SrsResourceId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct SrsResourceIdList(pub Vec<SrsResourceId>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SrsResourceSet {
    pub srs_resource_set_id: SrsResourceSetId,
    pub srs_resource_id_list: SrsResourceIdList,
    pub resource_set_type: ResourceSetType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrsResourceSetIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct SrsResourceSetList2(pub Vec<SrsResourceSet>); // edited manually

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct SrsResourceSetId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct SrsResourceSetItem {
    #[asn(optional_idx = 0)]
    pub num_sr_sresourcesperset: Option<Integer200>,
    #[asn(optional_idx = 1)]
    pub periodicity_list: Option<PeriodicityList>,
    #[asn(optional_idx = 2)]
    pub spatial_relation_info: Option<SpatialRelationInfo>,
    #[asn(optional_idx = 3)]
    pub pathloss_reference_info: Option<PathlossReferenceInfo>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<SrsResourceSetItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct SrsResourceSetList(pub Vec<SrsResourceSetItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SrsResourceTrigger {
    pub aperiodic_srs_resource_trigger_list: AperiodicSrsResourceTriggerList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SrsResourceTriggerIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SrsType {
    #[asn(key = 0, extended = false)]
    SemipersistentSrs(SemipersistentSrs),
    #[asn(key = 1, extended = false)]
    AperiodicSrs(AperiodicSrs),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(SrsTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct Ssb {
    pub pci_nr: Nrpci,
    #[asn(optional_idx = 0)]
    pub ssb_index: Option<SsbIndex>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SsbIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct SsbIndex(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SsbPositionsInBurst {
    #[asn(key = 0, extended = false)]
    ShortBitmap(BitString201),
    #[asn(key = 1, extended = false)]
    MediumBitmap(BitString202),
    #[asn(key = 2, extended = false)]
    LongBitmap(BitString203),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(SsbPositionsInBurstchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct SsbTfConfiguration {
    pub ssb_frequency: Integer204,
    pub ssb_subcarrier_spacing: Enumerated205,
    pub ssb_transmit_power: Integer206,
    pub ssb_periodicity: Enumerated207,
    pub ssb_half_frame_offset: Integer208,
    pub ssb_sfn_offset: Integer209,
    #[asn(optional_idx = 0)]
    pub ssb_position_in_burst: Option<SsbPositionsInBurst>,
    #[asn(optional_idx = 1)]
    pub sfn_initialisation_time: Option<RelativeTime1900>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SsbTfConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct SsbFreqInfo(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct SsbSubcarrierSpacing(pub u8);
impl SsbSubcarrierSpacing {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ120: u8 = 2u8;
    const K_HZ240: u8 = 3u8;
    const SPARE3: u8 = 4u8;
    const SPARE2: u8 = 5u8;
    const SPARE1: u8 = 6u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SsbTransmissionBitmap {
    #[asn(key = 0, extended = false)]
    ShortBitmap(BitString210),
    #[asn(key = 1, extended = false)]
    MediumBitmap(BitString211),
    #[asn(key = 2, extended = false)]
    LongBitmap(BitString212),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(SsbTransmissionBitmapchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct SsbTransmissionPeriodicity(pub u8);
impl SsbTransmissionPeriodicity {
    const SF10: u8 = 0u8;
    const SF20: u8 = 1u8;
    const SF40: u8 = 2u8;
    const SF80: u8 = 3u8;
    const SF160: u8 = 4u8;
    const SF320: u8 = 5u8;
    const SF640: u8 = 6u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127", extensible = true)]
pub struct SsbTransmissionTimingOffset(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SsbAreaCapacityValueItem {
    pub ssb_index: Integer213,
    pub ssb_area_capacity_value: Integer214,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SsbAreaCapacityValueItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SsbAreaCapacityValueList(pub Vec<SsbAreaCapacityValueItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct SsbAreaRadioResourceStatusItem {
    pub ssb_index: Integer215,
    pub ssb_area_dlgbrpr_busage: Integer216,
    pub ssb_area_ulgbrpr_busage: Integer217,
    pub ssb_area_d_lnon_gbrpr_busage: Integer218,
    pub ssb_area_u_lnon_gbrpr_busage: Integer219,
    pub ssb_area_dl_total_pr_busage: Integer220,
    pub ssb_area_ul_total_pr_busage: Integer221,
    #[asn(optional_idx = 0)]
    pub d_lscheduling_pdcchcc_eusage: Option<Integer222>,
    #[asn(optional_idx = 1)]
    pub u_lscheduling_pdcchcc_eusage: Option<Integer223>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SsbAreaRadioResourceStatusItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SsbAreaRadioResourceStatusList(pub Vec<SsbAreaRadioResourceStatusItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SsbInformation {
    pub ssb_information_list: SsbInformationList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SsbInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SsbInformationItem {
    pub ssb_configuration: SsbTfConfiguration,
    pub pci_nr: Nrpci,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SsbInformationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "255"
)]
pub struct SsbInformationList(pub Vec<SsbInformationItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SsbToReportItem {
    pub ssb_index: Integer224,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SsbToReportItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SsbToReportList(pub Vec<SsbToReportItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SulInformation {
    pub sul_nrarfcn: Integer225,
    pub sul_transmission_bandwidth: TransmissionBandwidth,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SulInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SulAccessIndication(pub u8);
impl SulAccessIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SearchWindowInformation {
    pub expected_propagation_delay: Integer226,
    pub delay_uncertainty: Integer227,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SearchWindowInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SelectedBandCombinationIndex(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SelectedFeatureSetEntryIndex(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SemipersistentSrs {
    pub srs_resource_set_id: SrsResourceSetId,
    #[asn(optional_idx = 0)]
    pub srs_spatial_relation: Option<SpatialRelationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SemipersistentSrsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct SerialNumber(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "31", extensible = true)]
pub struct ServCellIndex(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ServedCellInformation {
    pub nrcgi: Nrcgi,
    pub nrpci: Nrpci,
    #[asn(optional_idx = 0)]
    pub five_gs_tac: Option<FiveGsTac>,
    #[asn(optional_idx = 1)]
    pub configured_eps_tac: Option<ConfiguredEpsTac>,
    pub served_plm_ns: ServedPlmNsList,
    pub nr_mode_info: NrModeInfo,
    pub measurement_timing_configuration: OctetString228,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ServedCellInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServedCellsToAddItem {
    pub served_cell_information: ServedCellInformation,
    #[asn(optional_idx = 0)]
    pub gnb_du_system_information: Option<GnbDuSystemInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ServedCellsToAddItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct ServedCellsToAddList(pub Vec<ServedCellsToAddListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ServedCellsToDeleteItem {
    pub old_nrcgi: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ServedCellsToDeleteItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct ServedCellsToDeleteList(pub Vec<ServedCellsToDeleteListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServedCellsToModifyItem {
    pub old_nrcgi: Nrcgi,
    pub served_cell_information: ServedCellInformation,
    #[asn(optional_idx = 0)]
    pub gnb_du_system_information: Option<GnbDuSystemInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ServedCellsToModifyItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "512"
)]
pub struct ServedCellsToModifyList(pub Vec<ServedCellsToModifyListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ServedEutraCellsInformation {
    pub eutra_mode_info: EutraModeInfo,
    pub protected_eutra_resource_indication: ProtectedEutraResourceIndication,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ServedEutraCellsInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ServedPlmNsItem {
    pub plmn_identity: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ServedPlmNsItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "6")]
pub struct ServedPlmNsList(pub Vec<ServedPlmNsItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ServiceState(pub u8);
impl ServiceState {
    const IN_SERVICE: u8 = 0u8;
    const OUT_OF_SERVICE: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServiceStatus {
    pub service_state: ServiceState,
    #[asn(optional_idx = 0)]
    pub switching_off_ongoing: Option<Enumerated229>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ServiceStatusIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "64", extensible = true)]
pub struct ServingCellMo(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "22")]
pub struct ShortDrxCycleLength(pub u8);
impl ShortDrxCycleLength {
    const MS2: u8 = 0u8;
    const MS3: u8 = 1u8;
    const MS4: u8 = 2u8;
    const MS5: u8 = 3u8;
    const MS6: u8 = 4u8;
    const MS7: u8 = 5u8;
    const MS8: u8 = 6u8;
    const MS10: u8 = 7u8;
    const MS14: u8 = 8u8;
    const MS16: u8 = 9u8;
    const MS20: u8 = 10u8;
    const MS30: u8 = 11u8;
    const MS32: u8 = 12u8;
    const MS35: u8 = 13u8;
    const MS40: u8 = 14u8;
    const MS64: u8 = 15u8;
    const MS80: u8 = 16u8;
    const MS128: u8 = 17u8;
    const MS160: u8 = 18u8;
    const MS256: u8 = 19u8;
    const MS320: u8 = 20u8;
    const MS512: u8 = 21u8;
    const MS640: u8 = 22u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct ShortDrxCycleTimer(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SibtypetobeupdatedListItem {
    pub si_btype: Integer230,
    pub si_bmessage: OctetString231,
    pub value_tag: Integer232,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SibtypetobeupdatedListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SliceAvailableCapacity {
    pub slice_available_capacity_list: SliceAvailableCapacityList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceAvailableCapacityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SliceAvailableCapacityItem {
    pub plmn_identity: PlmnIdentity,
    pub snssai_available_capacity_list: SnssaiAvailableCapacityList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceAvailableCapacityItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct SliceAvailableCapacityList(pub Vec<SliceAvailableCapacityItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SliceSupportItem {
    pub snssai: Snssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceSupportItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct SliceSupportList(pub Vec<SliceSupportItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SliceToReportItem {
    pub plmn_identity: PlmnIdentity,
    pub snssa_ilist: SnssaiList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceToReportItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct SliceToReportList(pub Vec<SliceToReportItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SlotConfigurationItem {
    pub slot_index: Integer233,
    pub symbol_alloc_in_slot: SymbolAllocInSlot,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SlotConfigurationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "5120"
)]
pub struct SlotConfigurationList(pub Vec<SlotConfigurationItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct SlotNumber(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SpatialDirectionInformation {
    pub nr_prs_beam_information: NrPrsBeamInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SpatialDirectionInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SpatialRelationInfo {
    pub spatial_relationfor_resource_id: SpatialRelationforResourceId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SpatialRelationInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SpatialRelationPos {
    #[asn(key = 0, extended = false)]
    SsbPos(Ssb),
    #[asn(key = 1, extended = false)]
    PrsInformationPos(PrsInformationPos),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(SpatialRelationPoschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SpatialRelationforResourceId(pub Vec<SpatialRelationforResourceIdItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SpatialRelationforResourceIdItem {
    pub reference_signal: ReferenceSignal,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SpatialRelationforResourceIdItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct SpectrumSharingGroupId(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct SrsFrequency(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct SubcarrierSpacing(pub u8);
impl SubcarrierSpacing {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ60: u8 = 2u8;
    const K_HZ120: u8 = 3u8;
    const K_HZ240: u8 = 4u8;
    const SPARE3: u8 = 5u8;
    const SPARE2: u8 = 6u8;
    const SPARE1: u8 = 7u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct SubscriberProfileIDforRfp(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomeValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SupportedSulFreqBandItem {
    pub freq_band_indicator_nr: Integer234,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SupportedSulFreqBandItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SymbolAllocInSlot {
    #[asn(key = 0, extended = false)]
    AllDl(Null235),
    #[asn(key = 1, extended = false)]
    AllUl(Null236),
    #[asn(key = 2, extended = false)]
    BothDlAndUl(NumDlulSymbols),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(SymbolAllocInSlotchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct SystemFrameNumber(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct SystemInformationAreaId(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SystemInformationDeliveryCommand {
    pub protocol_i_es: SystemInformationDeliveryCommandProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TddInfo {
    pub nr_freq_info: NrFreqInfo,
    pub transmission_bandwidth: TransmissionBandwidth,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TddInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TddUlDlConfigCommonNr(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct TnlAssociationUsage(pub u8);
impl TnlAssociationUsage {
    const UE: u8 = 0u8;
    const NON_UE: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TnlCapacityIndicator {
    pub dltnl_offered_capacity: Integer237,
    pub dltnl_available_capacity: Integer238,
    pub ultnl_offered_capacity: Integer239,
    pub ultnl_available_capacity: Integer240,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TnlCapacityIndicatorIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TrpMeasurementRequestItem {
    pub trpid: Trpid,
    #[asn(optional_idx = 0)]
    pub search_window_information: Option<SearchWindowInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TrpMeasurementRequestItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct TrpMeasurementRequestList(pub Vec<TrpMeasurementRequestItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535", extensible = true)]
pub struct Trpid(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TrpInformation {
    pub trpid: Trpid,
    pub trp_information_type_response_list: TrpInformationTypeResponseList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TrpInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TrpInformationFailure {
    pub protocol_i_es: TrpInformationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TrpInformationItem {
    pub trp_information: TrpInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TrpInformationItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpInformationListTrpResp(pub Vec<TrpInformationListTrpRespEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TrpInformationRequest {
    pub protocol_i_es: TrpInformationRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TrpInformationResponse {
    pub protocol_i_es: TrpInformationResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct TrpInformationTypeItem(pub u8);
impl TrpInformationTypeItem {
    const NR_PCI: u8 = 0u8;
    const N_G_RAN_CGI: u8 = 1u8;
    const ARFCN: u8 = 2u8;
    const P_RS_CONFIG: u8 = 3u8;
    const S_SB_CONFIG: u8 = 4u8;
    const S_FN_INIT_TIME: u8 = 5u8;
    const SPATIAL_DIRECT_INFO: u8 = 6u8;
    const GEO_COORD: u8 = 7u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct TrpInformationTypeListTrpReq(pub Vec<TrpInformationTypeListTrpReqEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "8", extensible = false)]
pub enum TrpInformationTypeResponseItem {
    #[asn(key = 0, extended = false)]
    PciNr(Nrpci),
    #[asn(key = 1, extended = false)]
    NgRanCgi(Nrcgi),
    #[asn(key = 2, extended = false)]
    Nrarfcn(Integer241),
    #[asn(key = 3, extended = false)]
    PrsConfiguration(PrsConfiguration),
    #[asn(key = 4, extended = false)]
    SsBinformation(SsbInformation),
    #[asn(key = 5, extended = false)]
    SfnInitialisationTime(RelativeTime1900),
    #[asn(key = 6, extended = false)]
    SpatialDirectionInformation(SpatialDirectionInformation),
    #[asn(key = 7, extended = false)]
    GeographicalCoordinates(GeographicalCoordinates),
    #[asn(key = 8, extended = false)]
    ChoiceExtension(TrpInformationTypeResponseItemchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct TrpInformationTypeResponseList(pub Vec<TrpInformationTypeResponseItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TrpList(pub Vec<TrpListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TrpListItem {
    pub trpid: Trpid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TrpListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TrpMeasurementQuality {
    pub tr_pmeasurement_quality_item: TrpMeasurementQualityItem,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TrpMeasurementQualityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TrpMeasurementQualityItem {
    #[asn(key = 0, extended = false)]
    TimingMeasurementQuality(TimingMeasurementQuality),
    #[asn(key = 1, extended = false)]
    AngleMeasurementQuality(AngleMeasurementQuality),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(TrpMeasurementQualityItemchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TrpPositionDefinitionType {
    #[asn(key = 0, extended = false)]
    Direct(TrpPositionDirect),
    #[asn(key = 1, extended = false)]
    Referenced(TrpPositionReferenced),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(TrpPositionDefinitionTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TrpPositionDirect {
    pub accuracy: TrpPositionDirectAccuracy,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TrpPositionDirectIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TrpPositionDirectAccuracy {
    #[asn(key = 0, extended = false)]
    TrpPosition(AccessPointPosition),
    #[asn(key = 1, extended = false)]
    TrphAposition(NgranHighAccuracyAccessPointPosition),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(TrpPositionDirectAccuracychoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TrpPositionReferenced {
    pub reference_point: ReferencePoint,
    pub reference_point_type: TrpReferencePointType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TrpPositionReferencedIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TrpReferencePointType {
    #[asn(key = 0, extended = false)]
    TrpPositionRelativeGeodetic(RelativeGeodeticLocation),
    #[asn(key = 1, extended = false)]
    TrpPositionRelativeCartesian(RelativeCartesianLocation),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(TrpReferencePointTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TscAssistanceInformation {
    pub periodicity: Periodicity,
    #[asn(optional_idx = 0)]
    pub burst_arrival_time: Option<BurstArrivalTime>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TscAssistanceInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TscTrafficCharacteristics {
    #[asn(optional_idx = 0)]
    pub tsc_assistance_information_dl: Option<TscAssistanceInformation>,
    #[asn(optional_idx = 1)]
    pub tsc_assistance_information_ul: Option<TscAssistanceInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TscTrafficCharacteristicsIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TargetCellList(pub Vec<TargetCellListItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TargetCellListItem {
    pub target_cell: Nrcgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetCellListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct TimeInformationType(pub u8);
impl TimeInformationType {
    const LOCAL_CLOCK: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TimeReferenceInformation {
    pub reference_time: ReferenceTime,
    pub reference_sfn: ReferenceSfn,
    pub uncertainty: Uncertainty,
    pub time_information_type: TimeInformationType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TimeReferenceInformationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TimeStamp {
    pub system_frame_number: SystemFrameNumber,
    pub slot_index: TimeStampSlotIndex,
    #[asn(optional_idx = 0)]
    pub measurement_time: Option<RelativeTime1900>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<TimeStampIeExtension>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum TimeStampSlotIndex {
    #[asn(key = 0, extended = false)]
    Scs15(Integer242),
    #[asn(key = 1, extended = false)]
    Scs30(Integer243),
    #[asn(key = 2, extended = false)]
    Scs60(Integer244),
    #[asn(key = 3, extended = false)]
    Scs120(Integer245),
    #[asn(key = 4, extended = false)]
    ChoiceExtension(TimeStampSlotIndexchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TimeToWait(pub u8);
impl TimeToWait {
    const V1S: u8 = 0u8;
    const V2S: u8 = 1u8;
    const V5S: u8 = 2u8;
    const V10S: u8 = 3u8;
    const V20S: u8 = 4u8;
    const V60S: u8 = 5u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TimingMeasurementQuality {
    pub measurement_quality: Integer246,
    pub resolution: Enumerated247,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TimingMeasurementQualityIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct TraceActivation {
    pub trace_id: TraceId,
    pub interfaces_to_trace: InterfacesToTrace,
    pub trace_depth: TraceDepth,
    pub trace_collection_entity_ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TraceActivationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TraceDepth(pub u8);
impl TraceDepth {
    const MINIMUM: u8 = 0u8;
    const MEDIUM: u8 = 1u8;
    const MAXIMUM: u8 = 2u8;
    const MINIMUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 3u8;
    const MEDIUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 4u8;
    const MAXIMUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 5u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct TraceId(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TraceStart {
    pub protocol_i_es: TraceStartProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TrafficMappingInfo {
    #[asn(key = 0, extended = false)]
    IPtolayer2TrafficMappingInfo(IPtolayer2TrafficMappingInfo),
    #[asn(key = 1, extended = false)]
    BaPlayerBhrlCchannelMappingInfo(BaPlayerBhrlCchannelMappingInfo),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(TrafficMappingInfochoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct TransactionId(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TransmissionBandwidth {
    pub nrscs: Nrscs,
    pub nrnrb: Nrnrb,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TransmissionBandwidthIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TransmissionActionIndicator(pub u8);
impl TransmissionActionIndicator {
    const STOP: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TransmissionComb {
    #[asn(key = 0, extended = false)]
    N2(TransmissionCombn2),
    #[asn(key = 1, extended = false)]
    N4(TransmissionCombn4),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(TransmissionCombchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum TransmissionCombPos {
    #[asn(key = 0, extended = false)]
    N2(TransmissionCombPosn2),
    #[asn(key = 1, extended = false)]
    N4(TransmissionCombPosn4),
    #[asn(key = 2, extended = false)]
    N8(TransmissionCombPosn8),
    #[asn(key = 3, extended = false)]
    ChoiceExtension(TransmissionCombPoschoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TransmissionStopIndicator(pub u8);
impl TransmissionStopIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct TransportLayerAddressInfo {
    #[asn(optional_idx = 0)]
    pub transport_up_layer_address_info_to_add_list: Option<TransportUpLayerAddressInfoToAddList>,
    #[asn(optional_idx = 1)]
    pub transport_up_layer_address_info_to_remove_list:
        Option<TransportUpLayerAddressInfoToRemoveList>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TransportLayerAddressInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TransportUpLayerAddressInfoToAddItem {
    pub ip_sec_transport_layer_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub gtp_transport_layer_address_to_add: Option<GtptlAs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TransportUpLayerAddressInfoToAddItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TransportUpLayerAddressInfoToAddList(pub Vec<TransportUpLayerAddressInfoToAddItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct TransportUpLayerAddressInfoToRemoveItem {
    pub ip_sec_transport_layer_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub gtp_transport_layer_address_to_remove: Option<GtptlAs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TransportUpLayerAddressInfoToRemoveItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TransportUpLayerAddressInfoToRemoveList(
    pub Vec<TransportUpLayerAddressInfoToRemoveItem>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "1", sz_ub = "160")]
pub struct TransportLayerAddress(pub BitVec<Msb0, u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct TriggeringMessage(pub u8);
impl TriggeringMessage {
    const INITIATING_MESSAGE: u8 = 0u8;
    const SUCCESSFUL_OUTCOME: u8 = 1u8;
    const UNSUCCESSFUL_OUTCOME: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TypeOfError(pub u8);
impl TypeOfError {
    const NOT_UNDERSTOOD: u8 = 0u8;
    const MISSING: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UacAssistanceInfo {
    pub uacplmn_list: UacplmnList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UacAssistanceInfoIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct UacAction(pub u8);
impl UacAction {
    const REJECT_NON_EMERGENCY_MO_DT: u8 = 0u8;
    const REJECT_RRC_CR_SIGNALLING: u8 = 1u8;
    const PERMIT_EMERGENCY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 2u8;
    const PERMIT_HIGH_PRIORITY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UacCategoryType {
    #[asn(key = 0, extended = false)]
    UaCstandardized(UacAction),
    #[asn(key = 1, extended = false)]
    UacOperatorDefined(UacOperatorDefined),
    #[asn(key = 2, extended = false)]
    ChoiceExtension(UacCategoryTypechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UacOperatorDefined {
    pub access_category: Integer258,
    pub access_identity: BitString259,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UacOperatorDefinedIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UacplmnItem {
    pub plmn_identity: PlmnIdentity,
    pub uac_type_list: UacTypeList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UacplmnItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct UacplmnList(pub Vec<UacplmnItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct UacReductionIndication(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UacTypeItem {
    pub uac_reduction_indication: UacReductionIndication,
    pub uac_category_type: UacCategoryType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UacTypeItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct UacTypeList(pub Vec<UacTypeItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeCapabilityRatContainerList(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UeAssociatedLogicalF1ConnectionItem {
    #[asn(optional_idx = 0)]
    pub gnb_cu_ue_f1ap_id: Option<GnbCuUeF1apId>,
    #[asn(optional_idx = 1)]
    pub gnb_du_ue_f1ap_id: Option<GnbDuUeF1apId>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UeAssociatedLogicalF1ConnectionItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct UeAssociatedLogicalF1ConnectionListRes(
    pub Vec<UeAssociatedLogicalF1ConnectionListResEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct UeAssociatedLogicalF1ConnectionListResAck(
    pub Vec<UeAssociatedLogicalF1ConnectionListResAckEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeAssistanceInformation(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeAssistanceInformationEutra(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationConfirm {
    pub protocol_i_es: UeContextModificationConfirmProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationFailure {
    pub protocol_i_es: UeContextModificationFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationRefuse {
    pub protocol_i_es: UeContextModificationRefuseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationRequest {
    pub protocol_i_es: UeContextModificationRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationRequired {
    pub protocol_i_es: UeContextModificationRequiredProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationResponse {
    pub protocol_i_es: UeContextModificationResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeContextNotRetrievable(pub u8);
impl UeContextNotRetrievable {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseCommand {
    pub protocol_i_es: UeContextReleaseCommandProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseComplete {
    pub protocol_i_es: UeContextReleaseCompleteProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseRequest {
    pub protocol_i_es: UeContextReleaseRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSetupFailure {
    pub protocol_i_es: UeContextSetupFailureProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSetupRequest {
    pub protocol_i_es: UeContextSetupRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSetupResponse {
    pub protocol_i_es: UeContextSetupResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UeIdentityIndexValue {
    #[asn(key = 0, extended = false)]
    IndexLength10(BitString260),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(UeIdentityIndexValuechoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeInactivityNotification {
    pub protocol_i_es: UeInactivityNotificationProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct UlAoA {
    pub azimuth_ao_a: Integer261,
    #[asn(optional_idx = 0)]
    pub zenith_ao_a: Option<Integer262>,
    #[asn(optional_idx = 1)]
    pub lcs_to_gcs_translation_ao_a: Option<LcsToGcsTranslationAoA>,
    pub ie_extensions: UlAoAIeExtensions,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UlBhNonUpTrafficMapping {
    pub ul_bh_non_up_traffic_mapping_list: UlBhNonUpTrafficMappingList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlBhNonUpTrafficMappingIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct UlBhNonUpTrafficMappingItem {
    pub non_up_traffic_type: NonUpTrafficType,
    pub bh_info: BhInfo,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlBhNonUpTrafficMappingItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct UlBhNonUpTrafficMappingList(pub Vec<UlBhNonUpTrafficMappingItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct UlRtoaMeasurement {
    pub ul_rtoa_measurement_item: UlRtoaMeasurementItem,
    #[asn(optional_idx = 0)]
    pub additional_path_list: Option<AdditionalPathList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UlRtoaMeasurementIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum UlRtoaMeasurementItem {
    #[asn(key = 0, extended = false)]
    K0(Integer263),
    #[asn(key = 1, extended = false)]
    K1(Integer264),
    #[asn(key = 2, extended = false)]
    K2(Integer265),
    #[asn(key = 3, extended = false)]
    K3(Integer266),
    #[asn(key = 4, extended = false)]
    K4(Integer267),
    #[asn(key = 5, extended = false)]
    K5(Integer268),
    #[asn(key = 6, extended = false)]
    ChoiceExtension(UlRtoaMeasurementItemchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "126")]
pub struct UlSrsRsrp(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct UlUpTnlAddressToUpdateList(pub Vec<UlUpTnlAddressToUpdateListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UlUpTnlAddressToUpdateListItem {
    pub old_ip_adress: TransportLayerAddress,
    pub new_ip_adress: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlUpTnlAddressToUpdateListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32678"
)]
pub struct UlUpTnlInformationToUpdateList(pub Vec<UlUpTnlInformationToUpdateListEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UlUpTnlInformationToUpdateListItem {
    pub uluptnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub new_uluptnl_information: Option<UpTransportLayerInformation>,
    pub bh_info: BhInfo,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UlUpTnlInformationToUpdateListItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UlConfiguration {
    pub ulue_configuration: UlueConfiguration,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlConfigurationIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UlrrcMessageTransfer {
    pub protocol_i_es: UlrrcMessageTransferProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct UlueConfiguration(pub u8);
impl UlueConfiguration {
    const NO_DATA: u8 = 0u8;
    const SHARED: u8 = 1u8;
    const ONLY: u8 = 2u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UluptnlInformationToBeSetupItem {
    pub uluptnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UluptnlInformationToBeSetupItemIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct UluptnlInformationToBeSetupList(pub Vec<UluptnlInformationToBeSetupItem>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UpTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    GtpTunnel(GtpTunnel),
    #[asn(key = 1, extended = false)]
    ChoiceExtension(UpTransportLayerInformationchoiceExtension),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct UriAddress(pub String);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32767", extensible = true)]
pub struct Uncertainty(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "5")]
pub struct UplinkChannelBwPerScsList(pub Vec<ScsSpecificCarrier>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UplinkTxDirectCurrentListInformation(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VehicleUe(pub u8);
impl VehicleUe {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct VictimgNbSetId {
    pub victimg_nb_set_id: GnbSetId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<VictimgNbSetIdIeExtensions>,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningRequest {
    pub protocol_i_es: WriteReplaceWarningRequestProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningResponse {
    pub protocol_i_es: WriteReplaceWarningResponseProtocolIEs,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null2;

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AbortTransmissionchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AccessAndMobilityIndicationProtocolIEsEntryValue {
    #[asn(key = 359)]
    IdRachReportInformationList(RachReportInformationList),
    #[asn(key = 360)]
    IdRlfReportInformationList(RlfReportInformationList),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AccessAndMobilityIndicationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AccessAndMobilityIndicationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AccessAndMobilityIndicationProtocolIEs(
    pub Vec<AccessAndMobilityIndicationProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Enumerated3(pub u8);
impl Enumerated3 {
    const NORTH: u8 = 0u8;
    const SOUTH: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct Integer4(pub u32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct Integer5(pub i32);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Enumerated6(pub u8);
impl Enumerated6 {
    const HEIGHT: u8 = 0u8;
    const DEPTH: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct Integer7(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Integer8(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Integer9(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct Integer10(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Integer11(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Integer12(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AccessPointPositionIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AccessPointPositionIeExtensions(pub Vec<AccessPointPositionIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AccessSuccessProtocolIEsEntryValue {
    #[asn(key = 111)]
    IdNrcgi(Nrcgi),
    #[asn(key = 40)]
    IdGNbCuUeF1apId(GnbCuUeF1apId),
    #[asn(key = 41)]
    IdGNbDuUeF1apId(GnbDuUeF1apId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AccessSuccessProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AccessSuccessProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AccessSuccessProtocolIEs(pub Vec<AccessSuccessProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ActivatedCellsToBeUpdatedListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ActivatedCellsToBeUpdatedListItemIeExtensions(
    pub Vec<ActivatedCellsToBeUpdatedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "37949", extensible = true)]
pub struct Integer13(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Enumerated14(pub u8);
impl Enumerated14 {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ60: u8 = 2u8;
    const K_HZ120: u8 = 3u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Enumerated15(pub u8);
impl Enumerated15 {
    const NORMAL: u8 = 0u8;
    const EXTENDED: u8 = 1u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3301", extensible = true)]
pub struct Integer16(pub u16);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated17(pub u8);
impl Enumerated17 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ActiveUlbwpIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ActiveUlbwpIeExtensions(pub Vec<ActiveUlbwpIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AdditionalPdcpDuplicationTnlItemIeExtensionsEntryExtensionValue {
    #[asn(key = 280)]
    IdBhInfo(BhInfo),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalPdcpDuplicationTnlItemIeExtensionsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AdditionalPdcpDuplicationTnlItemIeExtensionsEntryExtensionValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AdditionalPdcpDuplicationTnlItemIeExtensions(
    pub Vec<AdditionalPdcpDuplicationTnlItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalPathItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AdditionalPathItemIeExtensions(pub Vec<AdditionalPathItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString18(pub Vec<u8>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalSibMessageListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AdditionalSibMessageListItemIeExtensions(
    pub Vec<AdditionalSibMessageListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AggressorCellListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AggressorCellListItemIeExtensions(pub Vec<AggressorCellListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AggressorgNbSetIdIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AggressorgNbSetIdIeExtensions(pub Vec<AggressorgNbSetIdIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationAndRetentionPriorityIeExtensions(
    pub Vec<AllocationAndRetentionPriorityIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AlternativeQoSParaSetItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AlternativeQoSParaSetItemIeExtensions(
    pub Vec<AlternativeQoSParaSetItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer19(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Integer20(pub u8);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated21(pub u8);
impl Enumerated21 {
    const DEG0DOT1: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AngleMeasurementQualityIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AngleMeasurementQualityIeExtensions(pub Vec<AngleMeasurementQualityIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated22(pub u8);
impl Enumerated22 {
    const TRUE: u8 = 0u8;
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AperiodicSrsIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AperiodicSrsIeExtensions(pub Vec<AperiodicSrsIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedSCellItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssociatedSCellItemIeExtensions(pub Vec<AssociatedSCellItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssociatedSCellListEntryValue {
    #[asn(key = 200)]
    IdAssociatedSCellItem(AssociatedSCellItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedSCellListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AssociatedSCellListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AvailablePlmnListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AvailablePlmnListItemIeExtensions(pub Vec<AvailablePlmnListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AvailableSnpnIdListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AvailableSnpnIdListItemIeExtensions(pub Vec<AvailableSnpnIdListItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BapMappingConfigurationProtocolIEsEntryValue {
    #[asn(key = 283)]
    IdBhRoutingInformationAddedList(BhRoutingInformationAddedList),
    #[asn(key = 285)]
    IdBhRoutingInformationRemovedList(BhRoutingInformationRemovedList),
    #[asn(key = 299)]
    IdTrafficMappingInformation(TrafficMappingInfo),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BapMappingConfigurationProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BapMappingConfigurationProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct BapMappingConfigurationProtocolIEs(pub Vec<BapMappingConfigurationProtocolIEsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BapMappingConfigurationAcknowledgeProtocolIEsEntryValue {
    #[asn(key = 7)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 78)]
    IdTransactionId(TransactionId),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BapMappingConfigurationAcknowledgeProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BapMappingConfigurationAcknowledgeProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct BapMappingConfigurationAcknowledgeProtocolIEs(
    pub Vec<BapMappingConfigurationAcknowledgeProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BapMappingConfigurationFailureProtocolIEsEntryValue {
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
pub struct BapMappingConfigurationFailureProtocolIEsEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BapMappingConfigurationFailureProtocolIEsEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct BapMappingConfigurationFailureProtocolIEs(
    pub Vec<BapMappingConfigurationFailureProtocolIEsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BapRoutingIdIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BapRoutingIdIeExtensions(pub Vec<BapRoutingIdIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BaPlayerBhrlCchannelMappingInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BaPlayerBhrlCchannelMappingInfoIeExtensions(
    pub Vec<BaPlayerBhrlCchannelMappingInfoIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BaPlayerBhrlCchannelMappingInfoItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BaPlayerBhrlCchannelMappingInfoItemIeExtensions(
    pub Vec<BaPlayerBhrlCchannelMappingInfoItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhRoutingInformationAddedListEntryValue {
    #[asn(key = 284)]
    IdBhRoutingInformationAddedListItem(BhRoutingInformationAddedListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhRoutingInformationAddedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhRoutingInformationAddedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhRoutingInformationAddedListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhRoutingInformationAddedListItemIeExtensions(
    pub Vec<BhRoutingInformationAddedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhRoutingInformationRemovedListEntryValue {
    #[asn(key = 286)]
    IdBhRoutingInformationRemovedListItem(BhRoutingInformationRemovedListItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhRoutingInformationRemovedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhRoutingInformationRemovedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhRoutingInformationRemovedListItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhRoutingInformationRemovedListItemIeExtensions(
    pub Vec<BhRoutingInformationRemovedListItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsFailedToBeModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsFailedToBeModifiedItemIeExtensions(
    pub Vec<BhChannelsFailedToBeModifiedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsFailedToBeModifiedListEntryValue {
    #[asn(key = 268)]
    IdBhChannelsFailedToBeModifiedItem(BhChannelsFailedToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsFailedToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsFailedToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsFailedToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsFailedToBeSetupItemIeExtensions(
    pub Vec<BhChannelsFailedToBeSetupItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsFailedToBeSetupListEntryValue {
    #[asn(key = 278)]
    IdBhChannelsFailedToBeSetupItem(BhChannelsFailedToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsFailedToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsFailedToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsFailedToBeSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsFailedToBeSetupModItemIeExtensions(
    pub Vec<BhChannelsFailedToBeSetupModItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsFailedToBeSetupModListEntryValue {
    #[asn(key = 270)]
    IdBhChannelsFailedToBeSetupModItem(BhChannelsFailedToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsFailedToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsFailedToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsModifiedItemIeExtensions(pub Vec<BhChannelsModifiedItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsModifiedListEntryValue {
    #[asn(key = 272)]
    IdBhChannelsModifiedItem(BhChannelsModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsRequiredToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsRequiredToBeReleasedItemIeExtensions(
    pub Vec<BhChannelsRequiredToBeReleasedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsRequiredToBeReleasedListEntryValue {
    #[asn(key = 276)]
    IdBhChannelsRequiredToBeReleasedItem(BhChannelsRequiredToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsRequiredToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsRequiredToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsSetupItemIeExtensions(pub Vec<BhChannelsSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsSetupListEntryValue {
    #[asn(key = 261)]
    IdBhChannelsSetupItem(BhChannelsSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsSetupModItemIeExtensions(pub Vec<BhChannelsSetupModItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsSetupModListEntryValue {
    #[asn(key = 274)]
    IdBhChannelsSetupModItem(BhChannelsSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeModifiedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsToBeModifiedItemIeExtensions(
    pub Vec<BhChannelsToBeModifiedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsToBeModifiedListEntryValue {
    #[asn(key = 262)]
    IdBhChannelsToBeModifiedItem(BhChannelsToBeModifiedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeModifiedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsToBeModifiedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeReleasedItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsToBeReleasedItemIeExtensions(
    pub Vec<BhChannelsToBeReleasedItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsToBeReleasedListEntryValue {
    #[asn(key = 264)]
    IdBhChannelsToBeReleasedItem(BhChannelsToBeReleasedItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeReleasedListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsToBeReleasedListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeSetupItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsToBeSetupItemIeExtensions(pub Vec<BhChannelsToBeSetupItemIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsToBeSetupListEntryValue {
    #[asn(key = 259)]
    IdBhChannelsToBeSetupItem(BhChannelsToBeSetupItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeSetupListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsToBeSetupListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeSetupModItemIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhChannelsToBeSetupModItemIeExtensions(
    pub Vec<BhChannelsToBeSetupModItemIeExtensionsEntry>,
);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BhChannelsToBeSetupModListEntryValue {
    #[asn(key = 266)]
    IdBhChannelsToBeSetupModItem(BhChannelsToBeSetupModItem),
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhChannelsToBeSetupModListEntry {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: BhChannelsToBeSetupModListEntryValue,
}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhInfoIeExtensionsEntry {}

#[derive(Clone, Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BhInfoIeExtensions(pub Vec<BhInfoIeExtensionsEntry>);

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BhQoSInformationchoiceExtension {}

#[derive(Clone, Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BplmnIdInfoItemIeExtensionsEntryExtensionValue {
    #[asn(key = 425)]
    IdConfiguredTacIndication(ConfiguredTacIndication),
    #[asn(key = 383)]
    IdNpnBroadcastInformation(NpnBroadcastInformation),
}
