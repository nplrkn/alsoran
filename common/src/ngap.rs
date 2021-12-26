# ! [allow (dead_code , unreachable_patterns)]use bitvec::vec::BitVec;
use bitvec::order::Msb0;
use asn1_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AmfTnlAssociationSetupItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AmfTnlAssociationSetupItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationSetupList(Vec<AmfTnlAssociationSetupItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AmfTnlAssociationToAddItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TnlAssociationUsage>,
    pub tnl_address_weight_factor: TnlAddressWeightFactor,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AmfTnlAssociationToAddItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationToAddList(Vec<AmfTnlAssociationToAddItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AmfTnlAssociationToRemoveItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AmfTnlAssociationToRemoveItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationToRemoveList(Vec<AmfTnlAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AmfTnlAssociationToUpdateItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TnlAssociationUsage>,
    #[asn(optional_idx = 1)]
    pub tnl_address_weight_factor: Option<TnlAddressWeightFactor>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<AmfTnlAssociationToUpdateItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationToUpdateList(Vec<AmfTnlAssociationToUpdateItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1099511627775")]
pub struct AmfUeNgapId(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfcpRelocationIndication {
    pub protocol_i_es: AmfcpRelocationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfConfigurationUpdate {
    pub protocol_i_es: AmfConfigurationUpdateProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfConfigurationUpdateAcknowledge {
    pub protocol_i_es: AmfConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfConfigurationUpdateFailure {
    pub protocol_i_es: AmfConfigurationUpdateFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "PrintableString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AmfName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AmfNameUtf8String(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AmfNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum AmfPagingTarget {
    #[asn(key = 0, extended = false)]
    GlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 1, extended = false)]
    Tai(Tai),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(AmfPagingTargetchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct AmfPointer(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct AmfRegionId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct AmfSetId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfStatusIndication {
    pub protocol_i_es: AmfStatusIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AdditionalDluptnlInformationForHoItem {
    pub additional_dl_ngu_up_tnl_information: UpTransportLayerInformation,
    pub additional_qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 0)]
    pub additional_dl_forwarding_uptnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AdditionalDluptnlInformationForHoItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct AdditionalDluptnlInformationForHoList(Vec<AdditionalDluptnlInformationForHoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AdditionalQosFlowInformation(u8);
impl AdditionalQosFlowInformation {
    const MORE_LIKELY: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationAndRetentionPriority {
    pub priority_level_arp: PriorityLevelArp,
    pub pre_emption_capability: PreEmptionCapability,
    pub pre_emption_vulnerability: PreEmptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationAndRetentionPriorityIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct AllowedCagListPerPlmn(Vec<CagId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedPniNpnItem {
    pub plmn_identity: PlmnIdentity,
    pub pni_npn_restricted: Enumerated2,
    pub allowed_cag_list_per_plmn: AllowedCagListPerPlmn,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllowedPniNpnItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AllowedPniNpnList(Vec<AllowedPniNpnItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AllowedNssai(Vec<AllowedNssaiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedNssaiItem {
    pub s_nssai: SNssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllowedNssaiItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AllowedTaCs(Vec<Tac>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8", extensible = true)]
pub struct AlternativeQoSParaSetIndex(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct AlternativeQoSParaSetItem {
    pub alternative_qo_s_para_set_index: AlternativeQoSParaSetIndex,
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

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AlternativeQoSParaSetList(Vec<AlternativeQoSParaSetItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "8", extensible = true)]
pub struct AlternativeQoSParaSetNotifyIndex(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct AreaOfInterest {
    #[asn(optional_idx = 0)]
    pub area_of_interest_tai_list: Option<AreaOfInterestTaiList>,
    #[asn(optional_idx = 1)]
    pub area_of_interest_cell_list: Option<AreaOfInterestCellList>,
    #[asn(optional_idx = 2)]
    pub area_of_interest_ran_node_list: Option<AreaOfInterestRanNodeList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<AreaOfInterestIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestCellItem {
    pub ngran_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestCellItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct AreaOfInterestCellList(Vec<AreaOfInterestCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestItem {
    pub area_of_interest: AreaOfInterest,
    pub location_reporting_reference_id: LocationReportingReferenceId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestList(Vec<AreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestRanNodeItem {
    pub global_ran_node_id: GlobalRanNodeId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestRanNodeItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestRanNodeList(Vec<AreaOfInterestRanNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestTaiItem {
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestTaiItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AreaOfInterestTaiList(Vec<AreaOfInterestTaiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMdtEutra {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMdtEutra),
    #[asn(key = 1, extended = false)]
    TaBased(TaBasedMdt),
    #[asn(key = 2, extended = false)]
    PlmnWide(Null3),
    #[asn(key = 3, extended = false)]
    TaiBased(TaiBasedMdt),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(AreaScopeOfMdtEutrAchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMdtNr {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMdtNr),
    #[asn(key = 1, extended = false)]
    TaBased(TaBasedMdt),
    #[asn(key = 2, extended = false)]
    PlmnWide(Null4),
    #[asn(key = 3, extended = false)]
    TaiBased(TaiBasedMdt),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(AreaScopeOfMdtNRchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AreaScopeOfNeighCellsItem {
    pub nr_frequency_info: NrFrequencyInfo,
    #[asn(optional_idx = 0)]
    pub pci_list_for_mdt: Option<PciListForMdt>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AreaScopeOfNeighCellsItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AreaScopeOfNeighCellsList(Vec<AreaScopeOfNeighCellsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AssistanceDataForPaging {
    #[asn(optional_idx = 0)]
    pub assistance_data_for_recommended_cells: Option<AssistanceDataForRecommendedCells>,
    #[asn(optional_idx = 1)]
    pub paging_attempt_information: Option<PagingAttemptInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<AssistanceDataForPagingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceDataForRecommendedCells {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AssistanceDataForRecommendedCellsIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AssociatedQosFlowItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub qos_flow_mapping_indication: Option<Enumerated5>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AssociatedQosFlowItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AssociatedQosFlowList(Vec<AssociatedQosFlowItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AuthenticatedIndication(u8);
impl AuthenticatedIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct AveragingWindow(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4000000000000", extensible = true)]
pub struct BitRate(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BluetoothMeasConfig(u8);
impl BluetoothMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BluetoothMeasConfigNameItem {
    pub bluetooth_name: BluetoothName,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BluetoothMeasConfigNameItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct BluetoothMeasConfigNameList(Vec<BluetoothMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct BluetoothMeasurementConfiguration {
    pub bluetooth_meas_config: BluetoothMeasConfig,
    #[asn(optional_idx = 0)]
    pub bluetooth_meas_config_name_list: Option<BluetoothMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub bt_rssi: Option<Enumerated6>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BluetoothMeasurementConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "248")]
pub struct BluetoothName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCancelledAreaList {
    #[asn(key = 0, extended = false)]
    CellIdCancelledEutra(CellIdCancelledEutra),
    #[asn(key = 1, extended = false)]
    TaiCancelledEutra(TaiCancelledEutra),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIdCancelledEutra(EmergencyAreaIdCancelledEutra),
    #[asn(key = 3, extended = false)]
    CellIdCancelledNr(CellIdCancelledNr),
    #[asn(key = 4, extended = false)]
    TaiCancelledNr(TaiCancelledNr),
    #[asn(key = 5, extended = false)]
    EmergencyAreaIdCancelledNr(EmergencyAreaIdCancelledNr),
    #[asn(key = 6, extended = false)]
    ChoiceExtensions(BroadcastCancelledAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCompletedAreaList {
    #[asn(key = 0, extended = false)]
    CellIdBroadcastEutra(CellIdBroadcastEutra),
    #[asn(key = 1, extended = false)]
    TaiBroadcastEutra(TaiBroadcastEutra),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIdBroadcastEutra(EmergencyAreaIdBroadcastEutra),
    #[asn(key = 3, extended = false)]
    CellIdBroadcastNr(CellIdBroadcastNr),
    #[asn(key = 4, extended = false)]
    TaiBroadcastNr(TaiBroadcastNr),
    #[asn(key = 5, extended = false)]
    EmergencyAreaIdBroadcastNr(EmergencyAreaIdBroadcastNr),
    #[asn(key = 6, extended = false)]
    ChoiceExtensions(BroadcastCompletedAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastPlmnItem {
    pub plmn_identity: PlmnIdentity,
    pub tai_slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastPlmnItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastPlmnList(Vec<BroadcastPlmnItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct BurstArrivalTime(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct CagId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CEmodeBSupportIndicator(u8);
impl CEmodeBSupportIndicator {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CEmodeBrestricted(u8);
impl CEmodeBrestricted {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CnAssistedRanTuning {
    #[asn(optional_idx = 0)]
    pub expected_ue_behaviour: Option<ExpectedUeBehaviour>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CnAssistedRanTuningIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct CnTypeRestrictionsForEquivalent(Vec<CnTypeRestrictionsForEquivalentItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CnTypeRestrictionsForEquivalentItem {
    pub plmn_identity: PlmnIdentity,
    pub cn_type: Enumerated7,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CnTypeRestrictionsForEquivalentItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CnTypeRestrictionsForServing(u8);
impl CnTypeRestrictionsForServing {
    const EPC_FORBIDDEN: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CountValueForPdcpSn12 {
    pub pdcp_sn12: Integer8,
    pub hfn_pdcp_sn12: Integer9,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CountValueForPdcpSn12IeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CountValueForPdcpSn18 {
    pub pdcp_sn18: Integer10,
    pub hfn_pdcp_sn18: Integer11,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CountValueForPdcpSn18IeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CpTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    EndpointIpAddress(TransportLayerAddress),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(CpTransportLayerInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CancelAllWarningMessages(u8);
impl CancelAllWarningMessages {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiEutra(Vec<CancelledCellsInEaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEaiEutraItem {
    pub eutra_cgi: EutraCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEaiEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiNr(Vec<CancelledCellsInEaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEaiNrItem {
    pub nr_cgi: NrCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEaiNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiEutra(Vec<CancelledCellsInTaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTaiEutraItem {
    pub eutra_cgi: EutraCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTaiEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiNr(Vec<CancelledCellsInTaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTaiNrItem {
    pub nr_cgi: NrCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTaiNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CandidateCell {
    #[asn(key = 0, extended = false)]
    CandidateCgi(CandidateCellId),
    #[asn(key = 1, extended = false)]
    CandidatePci(CandidatePci),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(CandidateCellchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellId {
    pub candidate_cell_id: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateCellIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellItem {
    pub candidate_cell: CandidateCell,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateCellItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CandidateCellList(Vec<CandidateCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidatePci {
    pub candidate_pci: Integer12,
    pub candidate_nrarfcn: Integer13,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidatePciIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = false)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    RadioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    Transport(CauseTransport),
    #[asn(key = 2, extended = false)]
    Nas(CauseNas),
    #[asn(key = 3, extended = false)]
    Protocol(CauseProtocol),
    #[asn(key = 4, extended = false)]
    Misc(CauseMisc),
    #[asn(key = 5, extended = false)]
    ChoiceExtensions(CausechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct CauseMisc(u8);
impl CauseMisc {
    const CONTROL_PROCESSING_OVERLOAD: u8 = 0u8;
    const NOT_ENOUGH_USER_PLANE_PROCESSING_RESOURCES: u8 = 1u8;
    const HARDWARE_FAILURE: u8 = 2u8;
    const OM_INTERVENTION: u8 = 3u8;
    const UNKNOWN_PLMN_OR_SNPN: u8 = 4u8;
    const UNSPECIFIED: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CauseNas(u8);
impl CauseNas {
    const NORMAL_RELEASE: u8 = 0u8;
    const AUTHENTICATION_FAILURE: u8 = 1u8;
    const DEREGISTER: u8 = 2u8;
    const UNSPECIFIED: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct CauseProtocol(u8);
impl CauseProtocol {
    const TRANSFER_SYNTAX_ERROR: u8 = 0u8;
    const ABSTRACT_SYNTAX_ERROR_REJECT: u8 = 1u8;
    const ABSTRACT_SYNTAX_ERROR_IGNORE_AND_NOTIFY: u8 = 2u8;
    const MESSAGE_NOT_COMPATIBLE_WITH_RECEIVER_STATE: u8 = 3u8;
    const SEMANTIC_ERROR: u8 = 4u8;
    const ABSTRACT_SYNTAX_ERROR_FALSELY_CONSTRUCTED_MESSAGE: u8 = 5u8;
    const UNSPECIFIED: u8 = 6u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "44")]
pub struct CauseRadioNetwork(u8);
impl CauseRadioNetwork {
    const UNSPECIFIED: u8 = 0u8;
    const TXNRELOCOVERALL_EXPIRY: u8 = 1u8;
    const SUCCESSFUL_HANDOVER: u8 = 2u8;
    const RELEASE_DUE_TO_NGRAN_GENERATED_REASON: u8 = 3u8;
    const RELEASE_DUE_TO_5GC_GENERATED_REASON: u8 = 4u8;
    const HANDOVER_CANCELLED: u8 = 5u8;
    const PARTIAL_HANDOVER: u8 = 6u8;
    const HO_FAILURE_IN_TARGET_5GC_NGRAN_NODE_OR_TARGET_SYSTEM: u8 = 7u8;
    const HO_TARGET_NOT_ALLOWED: u8 = 8u8;
    const TNGRELOCOVERALL_EXPIRY: u8 = 9u8;
    const TNGRELOCPREP_EXPIRY: u8 = 10u8;
    const CELL_NOT_AVAILABLE: u8 = 11u8;
    const UNKNOWN_TARGET_ID: u8 = 12u8;
    const NO_RADIO_RESOURCES_AVAILABLE_IN_TARGET_CELL: u8 = 13u8;
    const UNKNOWN_LOCAL_UE_NGAP_ID: u8 = 14u8;
    const INCONSISTENT_REMOTE_UE_NGAP_ID: u8 = 15u8;
    const HANDOVER_DESIRABLE_FOR_RADIO_REASON: u8 = 16u8;
    const TIME_CRITICAL_HANDOVER: u8 = 17u8;
    const RESOURCE_OPTIMISATION_HANDOVER: u8 = 18u8;
    const REDUCE_LOAD_IN_SERVING_CELL: u8 = 19u8;
    const USER_INACTIVITY: u8 = 20u8;
    const RADIO_CONNECTION_WITH_UE_LOST: u8 = 21u8;
    const RADIO_RESOURCES_NOT_AVAILABLE: u8 = 22u8;
    const INVALID_QOS_COMBINATION: u8 = 23u8;
    const FAILURE_IN_RADIO_INTERFACE_PROCEDURE: u8 = 24u8;
    const INTERACTION_WITH_OTHER_PROCEDURE: u8 = 25u8;
    const UNKNOWN_PDU_SESSION_ID: u8 = 26u8;
    const UNKOWN_QOS_FLOW_ID: u8 = 27u8;
    const MULTIPLE_PDU_SESSION_ID_INSTANCES: u8 = 28u8;
    const MULTIPLE_QOS_FLOW_ID_INSTANCES: u8 = 29u8;
    const ENCRYPTION_AND_OR_INTEGRITY_PROTECTION_ALGORITHMS_NOT_SUPPORTED: u8 = 30u8;
    const NG_INTRA_SYSTEM_HANDOVER_TRIGGERED: u8 = 31u8;
    const NG_INTER_SYSTEM_HANDOVER_TRIGGERED: u8 = 32u8;
    const XN_HANDOVER_TRIGGERED: u8 = 33u8;
    const NOT_SUPPORTED_5QI_VALUE: u8 = 34u8;
    const UE_CONTEXT_TRANSFER: u8 = 35u8;
    const IMS_VOICE_EPS_FALLBACK_OR_RAT_FALLBACK_TRIGGERED: u8 = 36u8;
    const UP_INTEGRITY_PROTECTION_NOT_POSSIBLE: u8 = 37u8;
    const UP_CONFIDENTIALITY_PROTECTION_NOT_POSSIBLE: u8 = 38u8;
    const SLICE_NOT_SUPPORTED: u8 = 39u8;
    const UE_IN_RRC_INACTIVE_STATE_NOT_REACHABLE: u8 = 40u8;
    const REDIRECTION: u8 = 41u8;
    const RESOURCES_NOT_AVAILABLE_FOR_THE_SLICE: u8 = 42u8;
    const UE_MAX_INTEGRITY_PROTECTED_DATA_RATE_REASON: u8 = 43u8;
    const RELEASE_DUE_TO_CN_DETECTED_MOBILITY: u8 = 44u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CauseTransport(u8);
impl CauseTransport {
    const TRANSPORT_RESOURCE_UNAVAILABLE: u8 = 0u8;
    const UNSPECIFIED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellCagInformation {
    pub ngran_cgi: NgranCgi,
    pub cell_cag_list: CellCagList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellCagInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMdtEutra {
    pub cell_id_listfor_mdt: CellIdListforMdtEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMdtEutraIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMdtNr {
    pub cell_id_listfor_mdt: CellIdListforMdtNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMdtNrIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct CellCagList(Vec<CagId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastEutra(Vec<CellIdBroadcastEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdBroadcastEutraItem {
    pub eutra_cgi: EutraCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdBroadcastEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastNr(Vec<CellIdBroadcastNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdBroadcastNrItem {
    pub nr_cgi: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdBroadcastNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledEutra(Vec<CellIdCancelledEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdCancelledEutraItem {
    pub eutra_cgi: EutraCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdCancelledEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledNr(Vec<CellIdCancelledNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdCancelledNrItem {
    pub nr_cgi: NrCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdCancelledNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CellIdListForRestart {
    #[asn(key = 0, extended = false)]
    EutraCgiListforRestart(EutraCgiList),
    #[asn(key = 1, extended = false)]
    NrCgiListforRestart(NrCgiList),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(CellIdListForRestartchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMdtEutra(Vec<EutraCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMdtNr(Vec<NrCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CellSize(u8);
impl CellSize {
    const VERYSMALL: u8 = 0u8;
    const SMALL: u8 = 1u8;
    const MEDIUM: u8 = 2u8;
    const LARGE: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellTrafficTrace {
    pub protocol_i_es: CellTrafficTraceProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellType {
    pub cell_size: CellSize,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellTypeIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CommonNetworkInstance(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiEutra(Vec<CompletedCellsInEaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEaiEutraItem {
    pub eutra_cgi: EutraCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEaiEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiNr(Vec<CompletedCellsInEaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEaiNrItem {
    pub nr_cgi: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEaiNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiEutra(Vec<CompletedCellsInTaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTaiEutraItem {
    pub eutra_cgi: EutraCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTaiEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiNr(Vec<CompletedCellsInTaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTaiNrItem {
    pub nr_cgi: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTaiNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ConcurrentWarningMessageInd(u8);
impl ConcurrentWarningMessageInd {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ConfidentialityProtectionIndication(u8);
impl ConfidentialityProtectionIndication {
    const REQUIRED: u8 = 0u8;
    const PREFERRED: u8 = 1u8;
    const NOT_NEEDED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ConfidentialityProtectionResult(u8);
impl ConfidentialityProtectionResult {
    const PERFORMED: u8 = 0u8;
    const NOT_PERFORMED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "128", sz_ub = "128")]
pub struct ConfiguredNssai(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ConfiguredTacIndication(u8);
impl ConfiguredTacIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ConnectionEstablishmentIndication {
    pub protocol_i_es: ConnectionEstablishmentIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct CoreNetworkAssistanceInformationForInactive {
    pub ue_identity_index_value: UeIdentityIndexValue,
    #[asn(optional_idx = 0)]
    pub ue_specific_drx: Option<PagingDrx>,
    pub periodic_registration_update_timer: PeriodicRegistrationUpdateTimer,
    #[asn(optional_idx = 1)]
    pub mico_mode_indication: Option<MicoModeIndication>,
    pub tai_list_for_inactive: TaiListForInactive,
    #[asn(optional_idx = 2)]
    pub expected_ue_behaviour: Option<ExpectedUeBehaviour>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<CoreNetworkAssistanceInformationForInactiveIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CoverageEnhancementLevel(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(u8);
impl Criticality {
    const REJECT: u8 = 0u8;
    const IGNORE: u8 = 1u8;
    const NOTIFY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct CriticalityDiagnostics {
    #[asn(optional_idx = 0)]
    pub procedure_code: Option<ProcedureCode>,
    #[asn(optional_idx = 1)]
    pub triggering_message: Option<TriggeringMessage>,
    #[asn(optional_idx = 2)]
    pub procedure_criticality: Option<Criticality>,
    #[asn(optional_idx = 3)]
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIeList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<CriticalityDiagnosticsIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnosticsIeItem {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIeId,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CriticalityDiagnosticsIeItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct CriticalityDiagnosticsIeList(Vec<CriticalityDiagnosticsIeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DapsRequestInfo {
    pub daps_indicator: Enumerated14,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DapsRequestInfoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DapsResponseInfo {
    pub dapsresponseindicator: Enumerated15,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DapsResponseInfoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DapsResponseInfoItem {
    pub drb_id: DrbId,
    pub daps_response_info: DapsResponseInfo,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DapsResponseInfoItemIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DapsResponseInfoList(Vec<DapsResponseInfoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DlCpSecurityInformation {
    pub dl_nas_mac: DlNasMac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DlCpSecurityInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct DlNasMac(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DlNguTnlInformationReused(u8);
impl DlNguTnlInformationReused {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DlForwarding(u8);
impl DlForwarding {
    const DL_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32", extensible = true)]
pub struct DrbId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DrbStatusDl {
    #[asn(key = 0, extended = false)]
    DrbStatusDl12(DrbStatusDl12),
    #[asn(key = 1, extended = false)]
    DrbStatusDl18(DrbStatusDl18),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(DrbStatusDLchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrbStatusDl12 {
    pub dl_count_value: CountValueForPdcpSn12,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrbStatusDl12IeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrbStatusDl18 {
    pub dl_count_value: CountValueForPdcpSn18,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrbStatusDl18IeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DrbStatusUl {
    #[asn(key = 0, extended = false)]
    DrbStatusUl12(DrbStatusUl12),
    #[asn(key = 1, extended = false)]
    DrbStatusUl18(DrbStatusUl18),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(DrbStatusULchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrbStatusUl12 {
    pub ul_count_value: CountValueForPdcpSn12,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BitString16>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DrbStatusUl12IeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrbStatusUl18 {
    pub ul_count_value: CountValueForPdcpSn18,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BitString17>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DrbStatusUl18IeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsSubjectToEarlyStatusTransferItem {
    pub drb_id: DrbId,
    pub first_dlcount: DrbStatusDl,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrBsSubjectToEarlyStatusTransferItemIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DrBsSubjectToEarlyStatusTransferList(Vec<DrBsSubjectToEarlyStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsSubjectToStatusTransferItem {
    pub drb_id: DrbId,
    pub drb_status_ul: DrbStatusUl,
    pub drb_status_dl: DrbStatusDl,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrBsSubjectToStatusTransferItemIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DrBsSubjectToStatusTransferList(Vec<DrBsSubjectToStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsToQosFlowsMappingItem {
    pub drb_id: DrbId,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrBsToQosFlowsMappingItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DrBsToQosFlowsMappingList(Vec<DrBsToQosFlowsMappingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct DataCodingScheme(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DataForwardingAccepted(u8);
impl DataForwardingAccepted {
    const DATA_FORWARDING_ACCEPTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DataForwardingNotPossible(u8);
impl DataForwardingNotPossible {
    const DATA_FORWARDING_NOT_POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DataForwardingResponseDrbItem {
    pub drb_id: DrbId,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DataForwardingResponseDrbItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DataForwardingResponseDrbList(Vec<DataForwardingResponseDrbItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct DataForwardingResponseErabList(Vec<DataForwardingResponseErabListItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataForwardingResponseErabListItem {
    pub e_rab_id: ERabId,
    pub dl_forwarding_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DataForwardingResponseErabListItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DeactivateTrace {
    pub protocol_i_es: DeactivateTraceProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DelayCritical(u8);
impl DelayCritical {
    const DELAY_CRITICAL: u8 = 0u8;
    const NON_DELAY_CRITICAL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DirectForwardingPathAvailability(u8);
impl DirectForwardingPathAvailability {
    const DIRECT_PATH_AVAILABLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNasTransport {
    pub protocol_i_es: DownlinkNasTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNonUeAssociatedNrpPaTransport {
    pub protocol_i_es: DownlinkNonUeAssociatedNrpPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRanConfigurationTransfer {
    pub protocol_i_es: DownlinkRanConfigurationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRanEarlyStatusTransfer {
    pub protocol_i_es: DownlinkRanEarlyStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRanStatusTransfer {
    pub protocol_i_es: DownlinkRanStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRimInformationTransfer {
    pub protocol_i_es: DownlinkRimInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkUeAssociatedNrpPaTransport {
    pub protocol_i_es: DownlinkUeAssociatedNrpPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct Dynamic5QiDescriptor {
    pub priority_level_qos: PriorityLevelQos,
    pub packet_delay_budget: PacketDelayBudget,
    pub packet_error_rate: PacketErrorRate,
    #[asn(optional_idx = 0)]
    pub five_qi: Option<FiveQi>,
    #[asn(optional_idx = 1)]
    pub delay_critical: Option<DelayCritical>,
    #[asn(optional_idx = 2)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 3)]
    pub maximum_data_burst_volume: Option<MaximumDataBurstVolume>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<Dynamic5QiDescriptorIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct ERabId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ERabInformationItem {
    pub e_rab_id: ERabId,
    #[asn(optional_idx = 0)]
    pub dl_forwarding: Option<DlForwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ERabInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct ERabInformationList(Vec<ERabInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EdtSession(u8);
impl EdtSession {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EnDcsonConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum EnbId {
    #[asn(key = 0, extended = false)]
    MacroEnbId(BitString18),
    #[asn(key = 1, extended = false)]
    HomeEnbId(BitString19),
    #[asn(key = 2, extended = false)]
    ShortMacroEnbId(BitString20),
    #[asn(key = 3, extended = false)]
    LongMacroEnbId(BitString21),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(EnbIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct EpsTac(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EpsTai {
    pub plmn_identity: PlmnIdentity,
    pub eps_tac: EpsTac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EpsTaiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EutraCgi {
    pub plmn_identity: PlmnIdentity,
    pub eutra_cell_identity: EutraCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraCgiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct EutraCgiList(Vec<EutraCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EutraCgiListForWarning(Vec<EutraCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct EutraCellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EutrAencryptionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EutrAintegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EarlyStatusTransferTransparentContainer {
    pub procedure_stage: ProcedureStageChoice,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EarlyStatusTransferTransparentContainerIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct EmergencyAreaId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastEutra(Vec<EmergencyAreaIdBroadcastEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdBroadcastEutraItem {
    pub emergency_area_id: EmergencyAreaId,
    pub completed_cells_in_eai_eutra: CompletedCellsInEaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdBroadcastEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastNr(Vec<EmergencyAreaIdBroadcastNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdBroadcastNrItem {
    pub emergency_area_id: EmergencyAreaId,
    pub completed_cells_in_eai_nr: CompletedCellsInEaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdBroadcastNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledEutra(Vec<EmergencyAreaIdCancelledEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdCancelledEutraItem {
    pub emergency_area_id: EmergencyAreaId,
    pub cancelled_cells_in_eai_eutra: CancelledCellsInEaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdCancelledEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledNr(Vec<EmergencyAreaIdCancelledNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdCancelledNrItem {
    pub emergency_area_id: EmergencyAreaId,
    pub cancelled_cells_in_eai_nr: CancelledCellsInEaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdCancelledNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdList(Vec<EmergencyAreaId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct EmergencyAreaIdListForRestart(Vec<EmergencyAreaId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EmergencyFallbackIndicator {
    pub emergency_fallback_request_indicator: EmergencyFallbackRequestIndicator,
    #[asn(optional_idx = 0)]
    pub emergency_service_target_cn: Option<EmergencyServiceTargetCn>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EmergencyFallbackIndicatorIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EmergencyFallbackRequestIndicator(u8);
impl EmergencyFallbackRequestIndicator {
    const EMERGENCY_FALLBACK_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EmergencyServiceTargetCn(u8);
impl EmergencyServiceTargetCn {
    const FIVE_GC: u8 = 0u8;
    const EPC: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EndIndication(u8);
impl EndIndication {
    const NO_FURTHER_DATA: u8 = 0u8;
    const FURTHER_DATA_EXISTS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EndpointIpAddressAndPort {
    pub endpoint_ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EndpointIpAddressAndPortIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EnhancedCoverageRestriction(u8);
impl EnhancedCoverageRestriction {
    const RESTRICTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct EquivalentPlmNs(Vec<PlmnIdentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EventL1LoggedMdtConfig {
    pub l1_threshold: MeasurementThresholdL1LoggedMdt,
    pub hysteresis: Hysteresis,
    pub time_to_trigger: TimeToTrigger,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EventL1LoggedMdtConfigIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum EventTrigger {
    #[asn(key = 0, extended = false)]
    OutOfCoverage(Enumerated22),
    #[asn(key = 1, extended = false)]
    EventL1LoggedMdtConfig(EventL1LoggedMdtConfig),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(EventTriggerchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct EventType(u8);
impl EventType {
    const DIRECT: u8 = 0u8;
    const CHANGE_OF_SERVE_CELL: u8 = 1u8;
    const UE_PRESENCE_IN_AREA_OF_INTEREST: u8 = 2u8;
    const STOP_CHANGE_OF_SERVE_CELL: u8 = 3u8;
    const STOP_UE_PRESENCE_IN_AREA_OF_INTEREST: u8 = 4u8;
    const CANCEL_LOCATION_REPORTING_FOR_THE_UE: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "30", extensible = true)]
pub struct ExpectedActivityPeriod(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct ExpectedHoInterval(u8);
impl ExpectedHoInterval {
    const SEC15: u8 = 0u8;
    const SEC30: u8 = 1u8;
    const SEC60: u8 = 2u8;
    const SEC90: u8 = 3u8;
    const SEC120: u8 = 4u8;
    const SEC180: u8 = 5u8;
    const LONG_TIME: u8 = 6u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "30", extensible = true)]
pub struct ExpectedIdlePeriod(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ExpectedUeActivityBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_activity_period: Option<ExpectedActivityPeriod>,
    #[asn(optional_idx = 1)]
    pub expected_idle_period: Option<ExpectedIdlePeriod>,
    #[asn(optional_idx = 2)]
    pub source_of_ue_activity_behaviour_information:
        Option<SourceOfUeActivityBehaviourInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ExpectedUeActivityBehaviourIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ExpectedUeBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_ue_activity_behaviour: Option<ExpectedUeActivityBehaviour>,
    #[asn(optional_idx = 1)]
    pub expected_ho_interval: Option<ExpectedHoInterval>,
    #[asn(optional_idx = 2)]
    pub expected_ue_mobility: Option<ExpectedUeMobility>,
    #[asn(optional_idx = 3)]
    pub expected_ue_moving_trajectory: Option<ExpectedUeMovingTrajectory>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<ExpectedUeBehaviourIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ExpectedUeMobility(u8);
impl ExpectedUeMobility {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ExpectedUeMovingTrajectory(Vec<ExpectedUeMovingTrajectoryItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ExpectedUeMovingTrajectoryItem {
    pub ngran_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<Integer23>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ExpectedUeMovingTrajectoryItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedAmfName {
    #[asn(optional_idx = 0)]
    pub amf_name_visible_string: Option<AmfNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub amf_name_utf8_string: Option<AmfNameUtf8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedAmfNameIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ExtendedConnectedTime(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedRanNodeName {
    #[asn(optional_idx = 0)]
    pub ran_node_name_visible_string: Option<RanNodeNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub ran_node_name_utf8_string: Option<RanNodeNameUtf8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedRanNodeNameIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65535", extensible = true)]
pub struct ExtendedPacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ExtendedRatRestrictionInformation {
    pub primary_rat_restriction: BitString24,
    pub secondary_rat_restriction: BitString25,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ExtendedRatRestrictionInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRncId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedSliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct ExtendedUeIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FailureIndication {
    pub uerlf_report_container: UerlfReportContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FailureIndicationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FirstDlCount {
    pub dr_bs_subject_to_early_status_transfer: DrBsSubjectToEarlyStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<FirstDlCountIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FiveGSTmsi {
    pub amf_set_id: AmfSetId,
    pub amf_pointer: AmfPointer,
    pub five_g_tmsi: FiveGTmsi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FiveGSTmsiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct FiveGTmsi(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct FiveQi(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ForbiddenAreaInformation(Vec<ForbiddenAreaInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForbiddenAreaInformationItem {
    pub plmn_identity: PlmnIdentity,
    pub forbidden_ta_cs: ForbiddenTaCs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ForbiddenAreaInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4096")]
pub struct ForbiddenTaCs(Vec<Tac>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromEutraNtoNgran {
    pub sourcee_nbid: IntersystemSoNeNbid,
    pub target_ngra_nnode_id: IntersystemSonngraNnodeId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromEutraNtoNgranIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromNgraNtoEutran {
    pub source_ngra_nnode_id: IntersystemSonngraNnodeId,
    pub targete_nbid: IntersystemSoNeNbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromNgraNtoEutranIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GbrQosInformation {
    pub maximum_flow_bit_rate_dl: BitRate,
    pub maximum_flow_bit_rate_ul: BitRate,
    pub guaranteed_flow_bit_rate_dl: BitRate,
    pub guaranteed_flow_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub notification_control: Option<NotificationControl>,
    #[asn(optional_idx = 1)]
    pub maximum_packet_loss_rate_dl: Option<PacketLossRate>,
    #[asn(optional_idx = 2)]
    pub maximum_packet_loss_rate_ul: Option<PacketLossRate>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<GbrQosInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GnbId {
    #[asn(key = 0, extended = false)]
    GnbId(BitString26),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(GnbIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "22")]
pub struct GnbSetId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GtpTeid(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GtpTunnel {
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GtpTeid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GtpTunnelIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Guami {
    pub plmn_identity: PlmnIdentity,
    pub amf_region_id: AmfRegionId,
    pub amf_set_id: AmfSetId,
    pub amf_pointer: AmfPointer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GuamiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct GuamiType(u8);
impl GuamiType {
    const NATIVE: u8 = 0u8;
    const MAPPED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalCableId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalEnbId {
    pub plm_nidentity: PlmnIdentity,
    pub enb_id: EnbId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalEnbIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalGnbId {
    pub plmn_identity: PlmnIdentity,
    pub gnb_id: GnbId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalGnbIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GlobalLineId {
    pub global_line_identity: GlobalLineIdentity,
    #[asn(optional_idx = 0)]
    pub line_type: Option<LineType>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GlobalLineIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalLineIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalN3iwfId {
    pub plmn_identity: PlmnIdentity,
    pub n3iwf_id: N3iwfId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalN3iwfIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalNgEnbId {
    pub plmn_identity: PlmnIdentity,
    pub ng_enb_id: NgEnbId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalNgEnbIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum GlobalRanNodeId {
    #[asn(key = 0, extended = false)]
    GlobalGnbId(GlobalGnbId),
    #[asn(key = 1, extended = false)]
    GlobalNgEnbId(GlobalNgEnbId),
    #[asn(key = 2, extended = false)]
    GlobalN3iwfId(GlobalN3iwfId),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(GlobalRanNodeIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTngfId {
    pub plmn_identity: PlmnIdentity,
    pub tngf_id: TngfId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTngfIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTwifId {
    pub plmn_identity: PlmnIdentity,
    pub twif_id: TwifId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTwifIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalWAgfId {
    pub plmn_identity: PlmnIdentity,
    pub w_agf_id: WAgfId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalWAgfIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct HfcNodeId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct HoReport {
    pub handover_report_type: Enumerated27,
    pub handover_cause: Cause,
    pub sourcecell_cgi: NgranCgi,
    pub targetcell_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub reestablishmentcell_cgi: Option<NgranCgi>,
    #[asn(optional_idx = 1)]
    pub sourcecell_c_rnti: Option<BitString28>,
    #[asn(optional_idx = 2)]
    pub targetcellin_e_utran: Option<EutraCgi>,
    #[asn(optional_idx = 3)]
    pub mobility_information: Option<MobilityInformation>,
    #[asn(optional_idx = 4)]
    pub uerlf_report_container: Option<UerlfReportContainer>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<HoReportIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCancel {
    pub protocol_i_es: HandoverCancelProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCancelAcknowledge {
    pub protocol_i_es: HandoverCancelAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCommand {
    pub protocol_i_es: HandoverCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct HandoverCommandTransfer {
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub qos_flow_to_be_forwarded_list: Option<QosFlowToBeForwardedList>,
    #[asn(optional_idx = 2)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDrbList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<HandoverCommandTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverFailure {
    pub protocol_i_es: HandoverFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct HandoverFlag(u8);
impl HandoverFlag {
    const HANDOVER_PREPARATION: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverNotify {
    pub protocol_i_es: HandoverNotifyProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverPreparationFailure {
    pub protocol_i_es: HandoverPreparationFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HandoverPreparationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HandoverPreparationUnsuccessfulTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequest {
    pub protocol_i_es: HandoverRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequestAcknowledge {
    pub protocol_i_es: HandoverRequestAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct HandoverRequestAcknowledgeTransfer {
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    pub qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDrbList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<HandoverRequestAcknowledgeTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequired {
    pub protocol_i_es: HandoverRequiredProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverRequiredTransfer {
    #[asn(optional_idx = 0)]
    pub direct_forwarding_path_availability: Option<DirectForwardingPathAvailability>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<HandoverRequiredTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverResourceAllocationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<HandoverResourceAllocationUnsuccessfulTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverSuccess {
    pub protocol_i_es: HandoverSuccessProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct HandoverType(u8);
impl HandoverType {
    const INTRA5GS: u8 = 0u8;
    const FIVEGS_TO_EPS: u8 = 1u8;
    const EPS_TO_5GS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct Hysteresis(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IabAuthorized(u8);
impl IabAuthorized {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IabSupported(u8);
impl IabSupported {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IabNodeIndication(u8);
impl IabNodeIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ImsVoiceSupportIndicator(u8);
impl ImsVoiceSupportIndicator {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct ImmediateMdtNr {
    pub measurements_to_activate: MeasurementsToActivate,
    #[asn(optional_idx = 0)]
    pub m1_configuration: Option<M1Configuration>,
    #[asn(optional_idx = 1)]
    pub m4_configuration: Option<M4Configuration>,
    #[asn(optional_idx = 2)]
    pub m5_configuration: Option<M5Configuration>,
    #[asn(optional_idx = 3)]
    pub m6_configuration: Option<M6Configuration>,
    #[asn(optional_idx = 4)]
    pub m7_configuration: Option<M7Configuration>,
    #[asn(optional_idx = 5)]
    pub bluetooth_measurement_configuration: Option<BluetoothMeasurementConfiguration>,
    #[asn(optional_idx = 6)]
    pub wlan_measurement_configuration: Option<WlanMeasurementConfiguration>,
    #[asn(optional_idx = 7)]
    pub mdt_location_info: Option<MdtLocationInfo>,
    #[asn(optional_idx = 8)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 9)]
    pub ie_extensions: Option<ImmediateMdtNrIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct IndexToRfsp(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InfoOnRecommendedCellsAndRanNodesForPaging {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    pub recommend_ran_nodes_for_paging: RecommendedRanNodesForPaging,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InfoOnRecommendedCellsAndRanNodesForPagingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupFailure {
    pub protocol_i_es: InitialContextSetupFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupRequest {
    pub protocol_i_es: InitialContextSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupResponse {
    pub protocol_i_es: InitialContextSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialUeMessage {
    pub protocol_i_es: InitialUeMessageProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessageValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct IntegrityProtectionIndication(u8);
impl IntegrityProtectionIndication {
    const REQUIRED: u8 = 0u8;
    const PREFERRED: u8 = 1u8;
    const NOT_NEEDED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IntegrityProtectionResult(u8);
impl IntegrityProtectionResult {
    const PERFORMED: u8 = 0u8;
    const NOT_PERFORMED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct IntendedNumberOfPagingAttempts(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct InterSystemFailureIndication {
    #[asn(optional_idx = 0)]
    pub uerlf_report_container: Option<UerlfReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<InterSystemFailureIndicationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterSystemHoReport {
    pub handover_report_type: InterSystemHandoverReportType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterSystemHoReportIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum InterSystemHandoverReportType {
    #[asn(key = 0, extended = false)]
    TooearlyIntersystemHo(TooearlyIntersystemHo),
    #[asn(key = 1, extended = false)]
    IntersystemUnnecessaryHo(IntersystemUnnecessaryHo),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(InterSystemHandoverReportTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct InterfacesToTrace(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSonConfigurationTransfer {
    pub transfer_type: IntersystemSonTransferType,
    pub intersystem_son_information: IntersystemSonInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSonConfigurationTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum IntersystemSonInformation {
    #[asn(key = 0, extended = false)]
    IntersystemSonInformationReport(IntersystemSonInformationReport),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(IntersystemSonInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSonInformationReport {
    #[asn(key = 0, extended = false)]
    HoReportInformation(InterSystemHoReport),
    #[asn(key = 1, extended = false)]
    FailureIndicationInformation(InterSystemFailureIndication),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(IntersystemSonInformationReportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSonngraNnodeId {
    pub global_ran_node_id: GlobalRanNodeId,
    pub selected_tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSonngraNnodeIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSonTransferType {
    #[asn(key = 0, extended = false)]
    FromEutraNtoNgran(FromEutraNtoNgran),
    #[asn(key = 1, extended = false)]
    FromNgraNtoEutran(FromNgraNtoEutran),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(IntersystemSonTransferTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSoNeNbid {
    pub globale_nbid: GlobalEnbId,
    pub selected_epstai: EpsTai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSoNeNbidIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemUnnecessaryHo {
    pub sourcecell_id: NgranCgi,
    pub targetcell_id: EutraCgi,
    pub early_iratho: Enumerated29,
    pub candidate_cell_list: CandidateCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemUnnecessaryHoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Lac(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Lai {
    pub plm_nidentity: PlmnIdentity,
    pub lac: Lac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LaiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LtemIndication(u8);
impl LtemIndication {
    const LTE_M: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LteuerlfReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LteueSidelinkAggregateMaximumBitrate {
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LteueSidelinkAggregateMaximumBitrateIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Ltev2xServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUe>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUe>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Ltev2xServicesAuthorizedIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum LastVisitedCellInformation {
    #[asn(key = 0, extended = false)]
    NgranCell(LastVisitedNgranCellInformation),
    #[asn(key = 1, extended = false)]
    EutranCell(LastVisitedEutranCellInformation),
    #[asn(key = 2, extended = false)]
    UtranCell(LastVisitedUtranCellInformation),
    #[asn(key = 3, extended = false)]
    GeranCell(LastVisitedGeranCellInformation),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(LastVisitedCellInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedCellItem {
    pub last_visited_cell_information: LastVisitedCellInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastVisitedCellItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedEutranCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedGeranCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LastVisitedNgranCellInformation {
    pub global_cell_id: NgranCgi,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: TimeUeStayedInCell,
    #[asn(optional_idx = 0)]
    pub time_ue_stayed_in_cell_enhanced_granularity:
        Option<TimeUeStayedInCellEnhancedGranularity>,
    #[asn(optional_idx = 1)]
    pub ho_cause_value: Option<Cause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LastVisitedNgranCellInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedUtranCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct LineType(u8);
impl LineType {
    const DSL: u8 = 0u8;
    const PON: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct LinksToLog(u8);
impl LinksToLog {
    const UPLINK: u8 = 0u8;
    const DOWNLINK: u8 = 1u8;
    const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LocationReportingAdditionalInfo(u8);
impl LocationReportingAdditionalInfo {
    const INCLUDE_PS_CELL: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingControl {
    pub protocol_i_es: LocationReportingControlProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingFailureIndication {
    pub protocol_i_es: LocationReportingFailureIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "64", extensible = true)]
pub struct LocationReportingReferenceId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LocationReportingRequestType {
    pub event_type: EventType,
    pub report_area: ReportArea,
    #[asn(optional_idx = 0)]
    pub area_of_interest_list: Option<AreaOfInterestList>,
    #[asn(optional_idx = 1)]
    pub location_reporting_reference_id_to_be_cancelled: Option<LocationReportingReferenceId>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LocationReportingRequestTypeIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct LoggedMdtNr {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    pub logged_mdt_trigger: LoggedMdtTrigger,
    #[asn(optional_idx = 0)]
    pub bluetooth_measurement_configuration: Option<BluetoothMeasurementConfiguration>,
    #[asn(optional_idx = 1)]
    pub wlan_measurement_configuration: Option<WlanMeasurementConfiguration>,
    #[asn(optional_idx = 2)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 3)]
    pub area_scope_of_neigh_cells_list: Option<AreaScopeOfNeighCellsList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<LoggedMdtNrIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum LoggedMdtTrigger {
    #[asn(key = 0, extended = false)]
    Periodical(Null30),
    #[asn(key = 1, extended = false)]
    EventTrigger(EventTrigger),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(LoggedMdtTriggerchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct LoggingDuration(u8);
impl LoggingDuration {
    const M10: u8 = 0u8;
    const M20: u8 = 1u8;
    const M40: u8 = 2u8;
    const M60: u8 = 3u8;
    const M90: u8 = 4u8;
    const M120: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "10")]
pub struct LoggingInterval(u8);
impl LoggingInterval {
    const MS320: u8 = 0u8;
    const MS640: u8 = 1u8;
    const MS1280: u8 = 2u8;
    const MS2560: u8 = 3u8;
    const MS5120: u8 = 4u8;
    const MS10240: u8 = 5u8;
    const MS20480: u8 = 6u8;
    const MS30720: u8 = 7u8;
    const MS40960: u8 = 8u8;
    const MS61440: u8 = 9u8;
    const INFINITY: u8 = 10u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct M1Configuration {
    pub m1reporting_trigger: M1ReportingTrigger,
    #[asn(optional_idx = 0)]
    pub m1threshold_event_a2: Option<M1ThresholdEventA2>,
    #[asn(optional_idx = 1)]
    pub m1periodic_reporting: Option<M1PeriodicReporting>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<M1ConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1PeriodicReporting {
    pub report_interval: ReportIntervalMdt,
    pub report_amount: ReportAmountMdt,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M1PeriodicReportingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct M1ReportingTrigger(u8);
impl M1ReportingTrigger {
    const PERIODIC: u8 = 0u8;
    const A2EVENTTRIGGERED: u8 = 1u8;
    const A2EVENTTRIGGERED_PERIODIC: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1ThresholdEventA2 {
    pub m1_threshold_type: M1ThresholdType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M1ThresholdEventA2IeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum M1ThresholdType {
    #[asn(key = 0, extended = false)]
    ThresholdRsrp(ThresholdRsrp),
    #[asn(key = 1, extended = false)]
    ThresholdRsrq(ThresholdRsrq),
    #[asn(key = 2, extended = false)]
    ThresholdSinr(ThresholdSinr),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(M1ThresholdTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M4Configuration {
    pub m4period: M4period,
    pub m4_links_to_log: LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M4ConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct M4period(u8);
impl M4period {
    const MS1024: u8 = 0u8;
    const MS2048: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
    const MIN1: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M5Configuration {
    pub m5period: M5period,
    pub m5_links_to_log: LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M5ConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct M5period(u8);
impl M5period {
    const MS1024: u8 = 0u8;
    const MS2048: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
    const MIN1: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M6Configuration {
    pub m6report_interval: M6reportInterval,
    pub m6_links_to_log: LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M6ConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct M6reportInterval(u8);
impl M6reportInterval {
    const MS120: u8 = 0u8;
    const MS240: u8 = 1u8;
    const MS480: u8 = 2u8;
    const MS640: u8 = 3u8;
    const MS1024: u8 = 4u8;
    const MS2048: u8 = 5u8;
    const MS5120: u8 = 6u8;
    const MS10240: u8 = 7u8;
    const MS20480: u8 = 8u8;
    const MS40960: u8 = 9u8;
    const MIN1: u8 = 10u8;
    const MIN6: u8 = 11u8;
    const MIN12: u8 = 12u8;
    const MIN30: u8 = 13u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M7Configuration {
    pub m7period: M7period,
    pub m7_links_to_log: LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M7ConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60", extensible = true)]
pub struct M7period(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MdtActivation(u8);
impl MdtActivation {
    const IMMEDIATE_MDT_ONLY: u8 = 0u8;
    const LOGGED_MDT_ONLY: u8 = 1u8;
    const IMMEDIATE_MDT_AND_TRACE: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MdtConfiguration {
    #[asn(optional_idx = 0)]
    pub mdt_config_nr: Option<MdtConfigurationNr>,
    #[asn(optional_idx = 1)]
    pub mdt_config_eutra: Option<MdtConfigurationEutra>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<MdtConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MdtConfigurationEutra {
    pub mdt_activation: MdtActivation,
    pub area_scope_of_mdt: AreaScopeOfMdtEutra,
    pub mdt_mode: MdtModeEutra,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MdtplmnList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MdtConfigurationEutraIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MdtConfigurationNr {
    pub mdt_activation: MdtActivation,
    pub area_scope_of_mdt: AreaScopeOfMdtNr,
    pub mdt_mode_nr: MdtModeNr,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MdtplmnList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MdtConfigurationNrIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MdtLocationInfo {
    pub mdt_location_information: MdtLocationInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MdtLocationInfoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MdtLocationInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MdtModeEutra(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum MdtModeNr {
    #[asn(key = 0, extended = false)]
    ImmediateMdtNr(ImmediateMdtNr),
    #[asn(key = 1, extended = false)]
    LoggedMdtNr(LoggedMdtNr),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(MdtModeNrchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MdtplmnList(Vec<PlmnIdentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MicoModeIndication(u8);
impl MicoModeIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct MaskedImeisv(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct MaximumDataBurstVolume(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MaximumIntegrityProtectedDataRate(u8);
impl MaximumIntegrityProtectedDataRate {
    const BITRATE64KBS: u8 = 0u8;
    const MAXIMUM_UE_RATE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum MeasurementThresholdL1LoggedMdt {
    #[asn(key = 0, extended = false)]
    ThresholdRsrp(ThresholdRsrp),
    #[asn(key = 1, extended = false)]
    ThresholdRsrq(ThresholdRsrq),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(MeasurementThresholdL1LoggedMdTchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MeasurementsToActivate(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct MessageIdentifier(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct MobilityInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct MobilityRestrictionList {
    pub serving_plmn: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub equivalent_plm_ns: Option<EquivalentPlmNs>,
    #[asn(optional_idx = 1)]
    pub rat_restrictions: Option<RatRestrictions>,
    #[asn(optional_idx = 2)]
    pub forbidden_area_information: Option<ForbiddenAreaInformation>,
    #[asn(optional_idx = 3)]
    pub service_area_information: Option<ServiceAreaInformation>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<MobilityRestrictionListIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum N3iwfId {
    #[asn(key = 0, extended = false)]
    N3iwfId(BitString31),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(N3iwfIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NasPdu(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NasNonDeliveryIndication {
    pub protocol_i_es: NasNonDeliveryIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NasSecurityParametersFromNgran(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NbIoTDefaultPagingDrx(u8);
impl NbIoTDefaultPagingDrx {
    const RF128: u8 = 0u8;
    const RF256: u8 = 1u8;
    const RF512: u8 = 2u8;
    const RF1024: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct NbIoTPagingTimeWindow(u8);
impl NbIoTPagingTimeWindow {
    const S1: u8 = 0u8;
    const S2: u8 = 1u8;
    const S3: u8 = 2u8;
    const S4: u8 = 3u8;
    const S5: u8 = 4u8;
    const S6: u8 = 5u8;
    const S7: u8 = 6u8;
    const S8: u8 = 7u8;
    const S9: u8 = 8u8;
    const S10: u8 = 9u8;
    const S11: u8 = 10u8;
    const S12: u8 = 11u8;
    const S13: u8 = 12u8;
    const S14: u8 = 13u8;
    const S15: u8 = 14u8;
    const S16: u8 = 15u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct NbIoTPagingEDrxCycle(u8);
impl NbIoTPagingEDrxCycle {
    const HF2: u8 = 0u8;
    const HF4: u8 = 1u8;
    const HF6: u8 = 2u8;
    const HF8: u8 = 3u8;
    const HF10: u8 = 4u8;
    const HF12: u8 = 5u8;
    const HF14: u8 = 6u8;
    const HF16: u8 = 7u8;
    const HF32: u8 = 8u8;
    const HF64: u8 = 9u8;
    const HF128: u8 = 10u8;
    const HF256: u8 = 11u8;
    const HF512: u8 = 12u8;
    const HF1024: u8 = 13u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NbIoTPagingEDrxInfo {
    pub nb_io_t_paging_e_drx_cycle: NbIoTPagingEDrxCycle,
    #[asn(optional_idx = 0)]
    pub nb_io_t_paging_time_window: Option<NbIoTPagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NbIoTPagingEDrxInfoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NbIoTPagingDrx(u8);
impl NbIoTPagingDrx {
    const RF32: u8 = 0u8;
    const RF64: u8 = 1u8;
    const RF128: u8 = 2u8;
    const RF256: u8 = 3u8;
    const RF512: u8 = 4u8;
    const RF1024: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct NbIoTUePriority(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum NgapPdu {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NgranCgi {
    #[asn(key = 0, extended = false)]
    NrCgi(NrCgi),
    #[asn(key = 1, extended = false)]
    EutraCgi(EutraCgi),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(NgranCgIchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NgranTnlAssociationToRemoveItem {
    pub tnl_association_transport_layer_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_transport_layer_address_amf: Option<CpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NgranTnlAssociationToRemoveItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NgranTnlAssociationToRemoveList(Vec<NgranTnlAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NgranTraceId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgReset {
    pub protocol_i_es: NgResetProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgResetAcknowledge {
    pub protocol_i_es: NgResetAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgSetupFailure {
    pub protocol_i_es: NgSetupFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgSetupRequest {
    pub protocol_i_es: NgSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgSetupResponse {
    pub protocol_i_es: NgSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "44", sz_ub = "44")]
pub struct Nid(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnAccessInformation {
    #[asn(key = 0, extended = false)]
    PniNpnAccessInformation(CellCagList),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(NpnAccessInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NpnMobilityInformation {
    #[asn(key = 0, extended = false)]
    SnpnMobilityInformation(SnpnMobilityInformation),
    #[asn(key = 1, extended = false)]
    PniNpnMobilityInformation(PniNpnMobilityInformation),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(NpnMobilityInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnPagingAssistanceInformation {
    #[asn(key = 0, extended = false)]
    PniNpnPagingAssistance(AllowedPniNpnList),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(NpnPagingAssistanceInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnSupport {
    #[asn(key = 0, extended = false)]
    Snpn(Nid),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(NpnSupportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrCgi {
    pub plmn_identity: PlmnIdentity,
    pub nr_cell_identity: NrCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrCgiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16384")]
pub struct NrCgiList(Vec<NrCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrCgiListForWarning(Vec<NrCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct NrPci(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Nrarfcn(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NrCellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1024", extensible = true)]
pub struct NrFrequencyBand(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NrFrequencyBandList(Vec<NrFrequencyBandItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrFrequencyBandItem {
    pub nr_frequency_band: NrFrequencyBand,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NrFrequencyBandItemIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrFrequencyInfo {
    pub nr_arfcn: Nrarfcn,
    pub frequency_band_list: NrFrequencyBandList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NrFrequencyInfoIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NrMobilityHistoryReport(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NrpPaPdu(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NruerlfReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrueSidelinkAggregateMaximumBitrate {
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrueSidelinkAggregateMaximumBitrateIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Nrv2xServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUe>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUe>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Nrv2xServicesAuthorizedIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct NRencryptionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct NRintegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct NetworkInstance(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NewSecurityContextInd(u8);
impl NewSecurityContextInd {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct NextHopChainingCount(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NextPagingAreaScope(u8);
impl NextPagingAreaScope {
    const SAME: u8 = 0u8;
    const CHANGED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum NgEnbId {
    #[asn(key = 0, extended = false)]
    MacroNgEnbId(BitString32),
    #[asn(key = 1, extended = false)]
    ShortMacroNgEnbId(BitString33),
    #[asn(key = 2, extended = false)]
    LongMacroNgEnbId(BitString34),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(NgEnbIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NonDynamic5QiDescriptor {
    pub five_qi: FiveQi,
    #[asn(optional_idx = 0)]
    pub priority_level_qos: Option<PriorityLevelQos>,
    #[asn(optional_idx = 1)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 2)]
    pub maximum_data_burst_volume: Option<MaximumDataBurstVolume>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<NonDynamic5QiDescriptorIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct NotAllowedTaCs(Vec<Tac>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NotificationCause(u8);
impl NotificationCause {
    const FULFILLED: u8 = 0u8;
    const NOT_FULFILLED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NotificationControl(u8);
impl NotificationControl {
    const NOTIFICATION_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NotifySourceNgranNode(u8);
impl NotifySourceNgranNode {
    const NOTIFY_SOURCE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberOfBroadcasts(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberOfBroadcastsRequested(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct OverloadAction(u8);
impl OverloadAction {
    const REJECT_NON_EMERGENCY_MO_DT: u8 = 0u8;
    const REJECT_RRC_CR_SIGNALLING: u8 = 1u8;
    const PERMIT_EMERGENCY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 2u8;
    const PERMIT_HIGH_PRIORITY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum OverloadResponse {
    #[asn(key = 0, extended = false)]
    OverloadAction(OverloadAction),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(OverloadResponsechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStart {
    pub protocol_i_es: OverloadStartProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct OverloadStartNssaiItem {
    pub slice_overload_list: SliceOverloadList,
    #[asn(optional_idx = 0)]
    pub slice_overload_response: Option<OverloadResponse>,
    #[asn(optional_idx = 1)]
    pub slice_traffic_load_reduction_indication: Option<TrafficLoadReductionIndication>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<OverloadStartNssaiItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct OverloadStartNssaiList(Vec<OverloadStartNssaiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStop {
    pub protocol_i_es: OverloadStopProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Pc5FlowBitRates {
    pub guaranteed_flow_bit_rate: BitRate,
    pub maximum_flow_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Pc5FlowBitRatesIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Pc5QoSFlowItem {
    pub pqi: FiveQi,
    #[asn(optional_idx = 0)]
    pub pc5_flow_bit_rates: Option<Pc5FlowBitRates>,
    #[asn(optional_idx = 1)]
    pub range: Option<Range>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Pc5QoSFlowItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct Pc5QoSFlowList(Vec<Pc5QoSFlowItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Pc5QoSParameters {
    pub pc5_qo_s_flow_list: Pc5QoSFlowList,
    #[asn(optional_idx = 0)]
    pub pc5_link_aggregate_bit_rates: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Pc5QoSParametersIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PciListForMdt(Vec<NrPci>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionAggregateMaximumBitRate {
    pub pdu_session_aggregate_maximum_bit_rate_dl: BitRate,
    pub pdu_session_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionAggregateMaximumBitRateIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PduSessionId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceAdmittedItem {
    pub pdu_session_id: PduSessionId,
    pub handover_request_acknowledge_transfer: OctetString35,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceAdmittedItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceAdmittedList(Vec<PduSessionResourceAdmittedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToModifyItemModCfm {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_indication_unsuccessful_transfer: OctetString36,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToModifyItemModCfmIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToModifyItemModRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_unsuccessful_transfer: OctetString37,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToModifyItemModResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToModifyListModCfm(Vec<PduSessionResourceFailedToModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToModifyListModRes(Vec<PduSessionResourceFailedToModifyItemModRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToResumeItemResReq {
    pub pdu_session_id: PduSessionId,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToResumeItemResReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToResumeItemResRes {
    pub pdu_session_id: PduSessionId,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToResumeItemResResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToResumeListResReq(Vec<PduSessionResourceFailedToResumeItemResReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToResumeListResRes(Vec<PduSessionResourceFailedToResumeItemResRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemCxtFail {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_unsuccessful_transfer: OctetString38,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemCxtFailIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemCxtRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_unsuccessful_transfer: OctetString39,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemCxtResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemHoAck {
    pub pdu_session_id: PduSessionId,
    pub handover_resource_allocation_unsuccessful_transfer: OctetString40,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemHoAckIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemPsReq {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_setup_failed_transfer: OctetString41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemPsReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemSuRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_unsuccessful_transfer: OctetString42,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemSuResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListCxtFail(Vec<PduSessionResourceFailedToSetupItemCxtFail>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListCxtRes(Vec<PduSessionResourceFailedToSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListHoAck(Vec<PduSessionResourceFailedToSetupItemHoAck>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListPsReq(Vec<PduSessionResourceFailedToSetupItemPsReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListSuRes(Vec<PduSessionResourceFailedToSetupItemSuRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceHandoverItem {
    pub pdu_session_id: PduSessionId,
    pub handover_command_transfer: OctetString43,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceHandoverItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceHandoverList(Vec<PduSessionResourceHandoverItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceInformationItem {
    pub pdu_session_id: PduSessionId,
    pub qos_flow_information_list: QosFlowInformationList,
    #[asn(optional_idx = 0)]
    pub dr_bs_to_qos_flows_mapping_list: Option<DrBsToQosFlowsMappingList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceInformationList(Vec<PduSessionResourceInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceItemCxtRelCpl {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceItemCxtRelCplIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceItemCxtRelReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceItemCxtRelReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceItemHoRqd {
    pub pdu_session_id: PduSessionId,
    pub handover_required_transfer: OctetString44,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceItemHoRqdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceListCxtRelCpl(Vec<PduSessionResourceItemCxtRelCpl>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceListCxtRelReq(Vec<PduSessionResourceItemCxtRelReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceListHoRqd(Vec<PduSessionResourceItemHoRqd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyConfirm {
    pub protocol_i_es: PduSessionResourceModifyConfirmProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PduSessionResourceModifyConfirmTransfer {
    pub qos_flow_modify_confirm_list: QosFlowModifyConfirmList,
    pub ulngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub additional_ng_uuptnl_information: Option<UpTransportLayerInformationPairList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_failed_to_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PduSessionResourceModifyConfirmTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyIndication {
    pub protocol_i_es: PduSessionResourceModifyIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceModifyIndicationTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTnlInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTnlInformationList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceModifyIndicationTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyIndicationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<PduSessionResourceModifyIndicationUnsuccessfulTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyItemModCfm {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_confirm_transfer: OctetString45,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModCfmIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyItemModInd {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_indication_transfer: OctetString46,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModIndIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceModifyItemModReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NasPdu>,
    pub pdu_session_resource_modify_request_transfer: OctetString47,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyItemModRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_response_transfer: OctetString48,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModCfm(Vec<PduSessionResourceModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModInd(Vec<PduSessionResourceModifyItemModInd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModReq(Vec<PduSessionResourceModifyItemModReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModRes(Vec<PduSessionResourceModifyItemModRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyRequest {
    pub protocol_i_es: PduSessionResourceModifyRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyRequestTransfer {
    pub protocol_i_es: PduSessionResourceModifyRequestTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyResponse {
    pub protocol_i_es: PduSessionResourceModifyResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct PduSessionResourceModifyResponseTransfer {
    #[asn(optional_idx = 0)]
    pub dl_ngu_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_ngu_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub qos_flow_add_or_modify_response_list: Option<QosFlowAddOrModifyResponseList>,
    #[asn(optional_idx = 3)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTnlInformationList>,
    #[asn(optional_idx = 4)]
    pub qos_flow_failed_to_add_or_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<PduSessionResourceModifyResponseTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceModifyUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceModifyUnsuccessfulTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceNotify {
    pub protocol_i_es: PduSessionResourceNotifyProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceNotifyItem {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_notify_transfer: OctetString49,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceNotifyItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceNotifyList(Vec<PduSessionResourceNotifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceNotifyReleasedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceNotifyReleasedTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PduSessionResourceNotifyTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_notify_list: Option<QosFlowNotifyList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_released_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PduSessionResourceNotifyTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceReleaseCommand {
    pub protocol_i_es: PduSessionResourceReleaseCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleaseCommandTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleaseCommandTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceReleaseResponse {
    pub protocol_i_es: PduSessionResourceReleaseResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleaseResponseTransfer {
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleaseResponseTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemNot {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_notify_released_transfer: OctetString50,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemNotIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemPsAck {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_unsuccessful_transfer: OctetString51,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemPsAckIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemPsFail {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_unsuccessful_transfer: OctetString52,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemPsFailIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemRelRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_release_response_transfer: OctetString53,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemRelResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListNot(Vec<PduSessionResourceReleasedItemNot>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListPsAck(Vec<PduSessionResourceReleasedItemPsAck>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListPsFail(Vec<PduSessionResourceReleasedItemPsFail>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListRelRes(Vec<PduSessionResourceReleasedItemRelRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceResumeItemResReq {
    pub pdu_session_id: PduSessionId,
    pub ue_context_resume_request_transfer: OctetString54,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceResumeItemResReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceResumeItemResRes {
    pub pdu_session_id: PduSessionId,
    pub ue_context_resume_response_transfer: OctetString55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceResumeItemResResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceResumeListResReq(Vec<PduSessionResourceResumeItemResReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceResumeListResRes(Vec<PduSessionResourceResumeItemResRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSecondaryRatUsageItem {
    pub pdu_session_id: PduSessionId,
    pub secondary_rat_data_usage_report_transfer: OctetString56,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSecondaryRatUsageItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSecondaryRatUsageList(Vec<PduSessionResourceSecondaryRatUsageItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceSetupItemCxtReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NasPdu>,
    pub s_nssai: SNssai,
    pub pdu_session_resource_setup_request_transfer: OctetString57,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceSetupItemCxtReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSetupItemCxtRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_response_transfer: OctetString58,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSetupItemCxtResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSetupItemHoReq {
    pub pdu_session_id: PduSessionId,
    pub s_nssai: SNssai,
    pub handover_request_transfer: OctetString59,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSetupItemHoReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceSetupItemSuReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub pdu_session_nas_pdu: Option<NasPdu>,
    pub s_nssai: SNssai,
    pub pdu_session_resource_setup_request_transfer: OctetString60,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceSetupItemSuReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSetupItemSuRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_response_transfer: OctetString61,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSetupItemSuResIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListCxtReq(Vec<PduSessionResourceSetupItemCxtReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListCxtRes(Vec<PduSessionResourceSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListHoReq(Vec<PduSessionResourceSetupItemHoReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListSuReq(Vec<PduSessionResourceSetupItemSuReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListSuRes(Vec<PduSessionResourceSetupItemSuRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceSetupRequest {
    pub protocol_i_es: PduSessionResourceSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceSetupRequestTransfer {
    pub protocol_i_es: PduSessionResourceSetupRequestTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceSetupResponse {
    pub protocol_i_es: PduSessionResourceSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PduSessionResourceSetupResponseTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTnlInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTnlInformationList>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<PduSessionResourceSetupResponseTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceSetupUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceSetupUnsuccessfulTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSuspendItemSusReq {
    pub pdu_session_id: PduSessionId,
    pub ue_context_suspend_request_transfer: OctetString62,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSuspendItemSusReqIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSuspendListSusReq(Vec<PduSessionResourceSuspendItemSusReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSwitchedItem {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_acknowledge_transfer: OctetString63,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSwitchedItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSwitchedList(Vec<PduSessionResourceSwitchedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceToBeSwitchedDlItem {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_transfer: OctetString64,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceToBeSwitchedDlItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceToBeSwitchedDlList(Vec<PduSessionResourceToBeSwitchedDlItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceToReleaseItemHoCmd {
    pub pdu_session_id: PduSessionId,
    pub handover_preparation_unsuccessful_transfer: OctetString65,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceToReleaseItemHoCmdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceToReleaseItemRelCmd {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_release_command_transfer: OctetString66,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceToReleaseItemRelCmdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceToReleaseListHoCmd(Vec<PduSessionResourceToReleaseItemHoCmd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceToReleaseListRelCmd(Vec<PduSessionResourceToReleaseItemRelCmd>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PduSessionType(u8);
impl PduSessionType {
    const IPV4: u8 = 0u8;
    const IPV6: u8 = 1u8;
    const IPV4V6: u8 = 2u8;
    const ETHERNET: u8 = 3u8;
    const UNSTRUCTURED: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionUsageReport {
    pub rat_type: Enumerated67,
    pub pdu_session_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionUsageReportIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PlmnIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PlmnSupportItem {
    pub plmn_identity: PlmnIdentity,
    pub slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PlmnSupportItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct PlmnSupportList(Vec<PlmnSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PniNpnMobilityInformation {
    pub allowed_pni_npi_list: AllowedPniNpnList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PniNpnMobilityInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsCancelRequest {
    pub protocol_i_es: PwsCancelRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsCancelResponse {
    pub protocol_i_es: PwsCancelResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PwsFailedCellIdList {
    #[asn(key = 0, extended = false)]
    EutraCgiPwsFailedList(EutraCgiList),
    #[asn(key = 1, extended = false)]
    NrCgiPwsFailedList(NrCgiList),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(PwsFailedCellIdListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsFailureIndication {
    pub protocol_i_es: PwsFailureIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsRestartIndication {
    pub protocol_i_es: PwsRestartIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1023", extensible = true)]
pub struct PacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PacketErrorRate {
    pub per_scalar: Integer68,
    pub per_exponent: Integer69,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PacketErrorRateIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1000", extensible = true)]
pub struct PacketLossRate(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Paging {
    pub protocol_i_es: PagingProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct PagingTimeWindow(u8);
impl PagingTimeWindow {
    const S1: u8 = 0u8;
    const S2: u8 = 1u8;
    const S3: u8 = 2u8;
    const S4: u8 = 3u8;
    const S5: u8 = 4u8;
    const S6: u8 = 5u8;
    const S7: u8 = 6u8;
    const S8: u8 = 7u8;
    const S9: u8 = 8u8;
    const S10: u8 = 9u8;
    const S11: u8 = 10u8;
    const S12: u8 = 11u8;
    const S13: u8 = 12u8;
    const S14: u8 = 13u8;
    const S15: u8 = 14u8;
    const S16: u8 = 15u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct PagingEDrxCycle(u8);
impl PagingEDrxCycle {
    const HFHALF: u8 = 0u8;
    const HF1: u8 = 1u8;
    const HF2: u8 = 2u8;
    const HF4: u8 = 3u8;
    const HF6: u8 = 4u8;
    const HF8: u8 = 5u8;
    const HF10: u8 = 6u8;
    const HF12: u8 = 7u8;
    const HF14: u8 = 8u8;
    const HF16: u8 = 9u8;
    const HF32: u8 = 10u8;
    const HF64: u8 = 11u8;
    const HF128: u8 = 12u8;
    const HF256: u8 = 13u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PagingAssisDataforCEcapabUe {
    pub eutra_cgi: EutraCgi,
    pub coverage_enhancement_level: CoverageEnhancementLevel,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PagingAssisDataforCEcapabUeIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct PagingAttemptCount(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PagingAttemptInformation {
    pub paging_attempt_count: PagingAttemptCount,
    pub intended_number_of_paging_attempts: IntendedNumberOfPagingAttempts,
    #[asn(optional_idx = 0)]
    pub next_paging_area_scope: Option<NextPagingAreaScope>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PagingAttemptInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct PagingDrx(u8);
impl PagingDrx {
    const V32: u8 = 0u8;
    const V64: u8 = 1u8;
    const V128: u8 = 2u8;
    const V256: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PagingOrigin(u8);
impl PagingOrigin {
    const NON_3GPP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct PagingPriority(u8);
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

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "20")]
pub struct PagingProbabilityInformation(u8);
impl PagingProbabilityInformation {
    const P00: u8 = 0u8;
    const P05: u8 = 1u8;
    const P10: u8 = 2u8;
    const P15: u8 = 3u8;
    const P20: u8 = 4u8;
    const P25: u8 = 5u8;
    const P30: u8 = 6u8;
    const P35: u8 = 7u8;
    const P40: u8 = 8u8;
    const P45: u8 = 9u8;
    const P50: u8 = 10u8;
    const P55: u8 = 11u8;
    const P60: u8 = 12u8;
    const P65: u8 = 13u8;
    const P70: u8 = 14u8;
    const P75: u8 = 15u8;
    const P80: u8 = 16u8;
    const P85: u8 = 17u8;
    const P90: u8 = 18u8;
    const P95: u8 = 19u8;
    const P100: u8 = 20u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PagingeDrxInformation {
    pub paging_e_drx_cycle: PagingEDrxCycle,
    #[asn(optional_idx = 0)]
    pub paging_time_window: Option<PagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PagingeDrxInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequest {
    pub protocol_i_es: PathSwitchRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestAcknowledge {
    pub protocol_i_es: PathSwitchRequestAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestAcknowledgeTransfer {
    #[asn(optional_idx = 0)]
    pub ul_ngu_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_indication: Option<SecurityIndication>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PathSwitchRequestAcknowledgeTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestFailure {
    pub protocol_i_es: PathSwitchRequestFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestSetupFailedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PathSwitchRequestSetupFailedTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestTransfer {
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_ngu_tnl_information_reused: Option<DlNguTnlInformationReused>,
    #[asn(optional_idx = 1)]
    pub user_plane_security_information: Option<UserPlaneSecurityInformation>,
    pub qos_flow_accepted_list: QosFlowAcceptedList,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PathSwitchRequestTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PathSwitchRequestUnsuccessfulTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PedestrianUe(u8);
impl PedestrianUe {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct PeriodicRegistrationUpdateTimer(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "640000", extensible = true)]
pub struct Periodicity(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct PortNumber(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PreEmptionCapability(u8);
impl PreEmptionCapability {
    const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PreEmptionVulnerability(u8);
impl PreEmptionVulnerability {
    const NOT_PRE_EMPTABLE: u8 = 0u8;
    const PRE_EMPTABLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Presence(u8);
impl Presence {
    const OPTIONAL: u8 = 0u8;
    const CONDITIONAL: u8 = 1u8;
    const MANDATORY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "15")]
pub struct PriorityLevelArp(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "127", extensible = true)]
pub struct PriorityLevelQos(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PrivacyIndicator(u8);
impl PrivacyIndicator {
    const IMMEDIATE_MDT: u8 = 0u8;
    const LOGGED_MDT: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum PrivateIeId {
    #[asn(key = 0, extended = false)]
    Local(Integer70),
    #[asn(key = 1, extended = false)]
    Global(ObjectIdentifier71),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PrivateMessage {
    pub private_i_es: PrivateMessagePrivateIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ProcedureCode(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ProcedureStageChoice {
    #[asn(key = 0, extended = false)]
    FirstDlCount(FirstDlCount),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(ProcedureStageChoicechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIeId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QoSFlowsUsageReportItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub rat_type: Enumerated72,
    pub qo_s_flows_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QoSFlowsUsageReportItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QoSFlowsUsageReportList(Vec<QoSFlowsUsageReportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum QosCharacteristics {
    #[asn(key = 0, extended = false)]
    NonDynamic5Qi(NonDynamic5QiDescriptor),
    #[asn(key = 1, extended = false)]
    Dynamic5Qi(Dynamic5QiDescriptor),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(QosCharacteristicschoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAcceptedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowAcceptedItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAcceptedList(Vec<QosFlowAcceptedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct QosFlowAddOrModifyRequestItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub qos_flow_level_qos_parameters: Option<QosFlowLevelQosParameters>,
    #[asn(optional_idx = 1)]
    pub e_rab_id: Option<ERabId>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<QosFlowAddOrModifyRequestItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAddOrModifyRequestList(Vec<QosFlowAddOrModifyRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAddOrModifyResponseItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowAddOrModifyResponseItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAddOrModifyResponseList(Vec<QosFlowAddOrModifyResponseItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct QosFlowFeedbackItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub update_feedback: Option<UpdateFeedback>,
    #[asn(optional_idx = 1)]
    pub c_npacket_delay_budget_dl: Option<ExtendedPacketDelayBudget>,
    #[asn(optional_idx = 2)]
    pub c_npacket_delay_budget_ul: Option<ExtendedPacketDelayBudget>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<QosFlowFeedbackItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowFeedbackList(Vec<QosFlowFeedbackItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63", extensible = true)]
pub struct QosFlowIdentifier(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowInformationItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub dl_forwarding: Option<DlForwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<QosFlowInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowInformationList(Vec<QosFlowInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowItemWithDataForwarding {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub data_forwarding_accepted: Option<DataForwardingAccepted>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<QosFlowItemWithDataForwardingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct QosFlowLevelQosParameters {
    pub qos_characteristics: QosCharacteristics,
    pub allocation_and_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GbrQosInformation>,
    #[asn(optional_idx = 1)]
    pub reflective_qos_attribute: Option<ReflectiveQosAttribute>,
    #[asn(optional_idx = 2)]
    pub additional_qos_flow_information: Option<AdditionalQosFlowInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<QosFlowLevelQosParametersIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowListWithCause(Vec<QosFlowWithCauseItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowListWithDataForwarding(Vec<QosFlowItemWithDataForwarding>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowModifyConfirmItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowModifyConfirmItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowModifyConfirmList(Vec<QosFlowModifyConfirmItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowNotifyItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub notification_cause: NotificationCause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowNotifyItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowNotifyList(Vec<QosFlowNotifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowParametersItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub alternative_qo_s_para_set_list: Option<AlternativeQoSParaSetList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<QosFlowParametersItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowParametersList(Vec<QosFlowParametersItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTnlInformation {
    pub up_transport_layer_information: UpTransportLayerInformation,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTnlInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTnlInformationItem {
    pub qos_flow_per_tnl_information: QosFlowPerTnlInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTnlInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct QosFlowPerTnlInformationList(Vec<QosFlowPerTnlInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowSetupRequestItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub qos_flow_level_qos_parameters: QosFlowLevelQosParameters,
    #[asn(optional_idx = 0)]
    pub e_rab_id: Option<ERabId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<QosFlowSetupRequestItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowSetupRequestList(Vec<QosFlowSetupRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowToBeForwardedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowToBeForwardedItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowToBeForwardedList(Vec<QosFlowToBeForwardedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowWithCauseItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowWithCauseItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1800", extensible = true)]
pub struct QosMonitoringReportingFrequency(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct QosMonitoringRequest(u8);
impl QosMonitoringRequest {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct RanUeNgapId(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RancpRelocationIndication {
    pub protocol_i_es: RancpRelocationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RanConfigurationUpdate {
    pub protocol_i_es: RanConfigurationUpdateProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RanConfigurationUpdateAcknowledge {
    pub protocol_i_es: RanConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RanConfigurationUpdateFailure {
    pub protocol_i_es: RanConfigurationUpdateFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "PrintableString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RanNodeName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RanNodeNameUtf8String(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RanNodeNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct RanPagingPriority(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RanStatusTransferTransparentContainer {
    pub dr_bs_subject_to_status_transfer_list: DrBsSubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RanStatusTransferTransparentContainerIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RatInformation(u8);
impl RatInformation {
    const UNLICENSED: u8 = 0u8;
    const NB_IO_T: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct RatRestrictionInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RatRestrictions(Vec<RatRestrictionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RatRestrictionsItem {
    pub plmn_identity: PlmnIdentity,
    pub rat_restriction_information: RatRestrictionInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RatRestrictionsItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RgLevelWirelineAccessCharacteristics(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RimInformation {
    pub targetg_nb_set_id: GnbSetId,
    pub rim_rs_detection: Enumerated73,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RimInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RimInformationTransfer {
    pub target_ran_node_id: TargetRanNodeId,
    pub source_ran_node_id: SourceRanNodeId,
    pub rim_information: RimInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RimInformationTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RncId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RrcContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "9")]
pub struct RrcEstablishmentCause(u8);
impl RrcEstablishmentCause {
    const EMERGENCY: u8 = 0u8;
    const HIGH_PRIORITY_ACCESS: u8 = 1u8;
    const MT_ACCESS: u8 = 2u8;
    const MO_SIGNALLING: u8 = 3u8;
    const MO_DATA: u8 = 4u8;
    const MO_VOICE_CALL: u8 = 5u8;
    const MO_VIDEO_CALL: u8 = 6u8;
    const MO_SMS: u8 = 7u8;
    const MPS_PRIORITY_ACCESS: u8 = 8u8;
    const MCS_PRIORITY_ACCESS: u8 = 9u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RrcInactiveTransitionReport {
    pub protocol_i_es: RrcInactiveTransitionReportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct RrcInactiveTransitionReportRequest(u8);
impl RrcInactiveTransitionReportRequest {
    const SUBSEQUENT_STATE_TRANSITION_REPORT: u8 = 0u8;
    const SINGLE_RRC_CONNECTED_STATE_REPORT: u8 = 1u8;
    const CANCEL_REPORT: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RrcState(u8);
impl RrcState {
    const INACTIVE: u8 = 0u8;
    const CONNECTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Rsn(u8);
impl Rsn {
    const V1: u8 = 0u8;
    const V2: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "8")]
pub struct Range(u8);
impl Range {
    const M50: u8 = 0u8;
    const M80: u8 = 1u8;
    const M180: u8 = 2u8;
    const M200: u8 = 3u8;
    const M350: u8 = 4u8;
    const M400: u8 = 5u8;
    const M500: u8 = 6u8;
    const M700: u8 = 7u8;
    const M1000: u8 = 8u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RecommendedCellItem {
    pub ngran_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<Integer74>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RecommendedCellItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedCellList(Vec<RecommendedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedCellsForPaging {
    pub recommended_cell_list: RecommendedCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedCellsForPagingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRanNodeItem {
    pub amf_paging_target: AmfPagingTarget,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRanNodeItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedRanNodeList(Vec<RecommendedRanNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRanNodesForPaging {
    pub recommended_ran_node_list: RecommendedRanNodeList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRanNodesForPagingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RedirectionVoiceFallback(u8);
impl RedirectionVoiceFallback {
    const POSSIBLE: u8 = 0u8;
    const NOT_POSSIBLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RedundantPduSessionInformation {
    pub rsn: Rsn,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RedundantPduSessionInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct RedundantQosFlowIndicator(u8);
impl RedundantQosFlowIndicator {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReflectiveQosAttribute(u8);
impl ReflectiveQosAttribute {
    const SUBJECT_TO: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct RejectedNssaIinPlmn(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct RejectedNssaIinTa(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RelativeAmfCapacity(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct RepetitionPeriod(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct ReportAmountMdt(u8);
impl ReportAmountMdt {
    const R1: u8 = 0u8;
    const R2: u8 = 1u8;
    const R4: u8 = 2u8;
    const R8: u8 = 3u8;
    const R16: u8 = 4u8;
    const R32: u8 = 5u8;
    const R64: u8 = 6u8;
    const RINFINITY: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReportArea(u8);
impl ReportArea {
    const CELL: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "12")]
pub struct ReportIntervalMdt(u8);
impl ReportIntervalMdt {
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
    const MIN60: u8 = 12u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RerouteNasRequest {
    pub protocol_i_es: RerouteNasRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ResetAll(u8);
impl ResetAll {
    const RESET_ALL: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum ResetType {
    #[asn(key = 0, extended = false)]
    NgInterface(ResetAll),
    #[asn(key = 1, extended = false)]
    PartOfNgInterface(UeAssociatedLogicalNgConnectionList),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(ResetTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RetrieveUeInformation {
    pub protocol_i_es: RetrieveUeInformationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RoutingId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SNssai {
    pub sst: Sst,
    #[asn(optional_idx = 0)]
    pub sd: Option<Sd>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SNssaiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct SctpTlAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct Sd(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SnpnMobilityInformation {
    pub serving_nid: Nid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SnpnMobilityInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SonConfigurationTransfer {
    pub target_ran_node_id: TargetRanNodeId,
    pub source_ran_node_id: SourceRanNodeId,
    pub son_information: SonInformation,
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTnlConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SonConfigurationTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SonInformation {
    #[asn(key = 0, extended = false)]
    SonInformationRequest(SonInformationRequest),
    #[asn(key = 1, extended = false)]
    SonInformationReply(SonInformationReply),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(SonInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SonInformationReply {
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTnlConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SonInformationReplyIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SonInformationReport {
    #[asn(key = 0, extended = false)]
    FailureIndicationInformation(FailureIndication),
    #[asn(key = 1, extended = false)]
    HoReportInformation(HoReport),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(SonInformationReportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SonInformationRequest(u8);
impl SonInformationRequest {
    const XN_TNL_CONFIGURATION_INFO: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SrvccOperationPossible(u8);
impl SrvccOperationPossible {
    const POSSIBLE: u8 = 0u8;
    const NOT_POSSIBLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct Sst(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ScheduledCommunicationTime {
    #[asn(optional_idx = 0)]
    pub dayof_week: Option<BitString75>,
    #[asn(optional_idx = 1)]
    pub timeof_day_start: Option<Integer76>,
    #[asn(optional_idx = 2)]
    pub timeof_day_end: Option<Integer77>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ScheduledCommunicationTimeIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SecondaryRatDataUsageReport {
    pub protocol_i_es: SecondaryRatDataUsageReportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecondaryRatDataUsageReportTransfer {
    #[asn(optional_idx = 0)]
    pub secondary_rat_usage_information: Option<SecondaryRatUsageInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SecondaryRatDataUsageReportTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SecondaryRatUsageInformation {
    #[asn(optional_idx = 0)]
    pub pdu_session_usage_report: Option<PduSessionUsageReport>,
    #[asn(optional_idx = 1)]
    pub qos_flows_usage_report_list: Option<QoSFlowsUsageReportList>,
    #[asn(optional_idx = 2)]
    pub ie_extension: Option<SecondaryRatUsageInformationIeExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityContext {
    pub next_hop_chaining_count: NextHopChainingCount,
    pub next_hop_nh: SecurityKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecurityContextIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecurityIndication {
    pub integrity_protection_indication: IntegrityProtectionIndication,
    pub confidentiality_protection_indication: ConfidentialityProtectionIndication,
    #[asn(optional_idx = 0)]
    pub maximum_integrity_protected_data_rate_ul: Option<MaximumIntegrityProtectedDataRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SecurityIndicationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "256", sz_ub = "256")]
pub struct SecurityKey(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityResult {
    pub integrity_protection_result: IntegrityProtectionResult,
    pub confidentiality_protection_result: ConfidentialityProtectionResult,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecurityResultIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SensorMeasConfig(u8);
impl SensorMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SensorMeasConfigNameItem {
    pub sensor_name_config: SensorNameConfig,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SensorMeasConfigNameItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct SensorMeasConfigNameList(Vec<SensorMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SensorMeasurementConfiguration {
    pub sensor_meas_config: SensorMeasConfig,
    #[asn(optional_idx = 0)]
    pub sensor_meas_config_name_list: Option<SensorMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SensorMeasurementConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SensorNameConfig {
    #[asn(key = 0, extended = false)]
    UncompensatedBarometricConfig(Enumerated78),
    #[asn(key = 1, extended = false)]
    UeSpeedConfig(Enumerated79),
    #[asn(key = 2, extended = false)]
    UeOrientationConfig(Enumerated80),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(SensorNameConfigchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct SerialNumber(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServedGuamiItem {
    pub guami: Guami,
    #[asn(optional_idx = 0)]
    pub backup_amf_name: Option<AmfName>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ServedGuamiItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct ServedGuamiList(Vec<ServedGuamiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ServiceAreaInformation(Vec<ServiceAreaInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ServiceAreaInformationItem {
    pub plmn_identity: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub allowed_ta_cs: Option<AllowedTaCs>,
    #[asn(optional_idx = 1)]
    pub not_allowed_ta_cs: Option<NotAllowedTaCs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ServiceAreaInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct SgNbUeX2apId(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceOverloadItem {
    pub s_nssai: SNssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceOverloadItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct SliceOverloadList(Vec<SliceOverloadItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceSupportItem {
    pub s_nssai: SNssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceSupportItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct SliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceNgranNodeToTargetNgranNodeTransparentContainer {
    pub rrc_container: RrcContainer,
    #[asn(optional_idx = 0)]
    pub pdu_session_resource_information_list: Option<PduSessionResourceInformationList>,
    #[asn(optional_idx = 1)]
    pub e_rab_information_list: Option<ERabInformationList>,
    pub target_cell_id: NgranCgi,
    #[asn(optional_idx = 2)]
    pub index_to_rfsp: Option<IndexToRfsp>,
    pub ue_history_information: UeHistoryInformation,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceNgranNodeToTargetNgranNodeTransparentContainerIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SourceOfUeActivityBehaviourInformation(u8);
impl SourceOfUeActivityBehaviourInformation {
    const SUBSCRIPTION_INFORMATION: u8 = 0u8;
    const STATISTICS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SourceRanNodeId {
    pub global_ran_node_id: GlobalRanNodeId,
    pub selected_tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceRanNodeIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceToTargetAmfInformationReroute {
    #[asn(optional_idx = 0)]
    pub configured_nssai: Option<ConfiguredNssai>,
    #[asn(optional_idx = 1)]
    pub rejected_nssa_iin_plmn: Option<RejectedNssaIinPlmn>,
    #[asn(optional_idx = 2)]
    pub rejected_nssa_iin_ta: Option<RejectedNssaIinTa>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceToTargetAmfInformationRerouteIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceToTargetTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SupportedTaItem {
    pub tac: Tac,
    pub broadcast_plmn_list: BroadcastPlmnList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SupportedTaItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct SupportedTaList(Vec<SupportedTaItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SuspendRequestIndication(u8);
impl SuspendRequestIndication {
    const SUSPEND_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SuspendResponseIndication(u8);
impl SuspendResponseIndication {
    const SUSPEND_INDICATED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SuspendIndicator(u8);
impl SuspendIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaBasedMdt {
    pub ta_listfor_mdt: TaListforMdt,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaBasedMdtIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct Tac(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Tai {
    pub plmn_identity: PlmnIdentity,
    pub tac: Tac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiBasedMdt {
    pub tai_listfor_mdt: TaiListforMdt,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiBasedMdtIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastEutra(Vec<TaiBroadcastEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiBroadcastEutraItem {
    pub tai: Tai,
    pub completed_cells_in_tai_eutra: CompletedCellsInTaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiBroadcastEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastNr(Vec<TaiBroadcastNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiBroadcastNrItem {
    pub tai: Tai,
    pub completed_cells_in_tai_nr: CompletedCellsInTaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiBroadcastNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledEutra(Vec<TaiCancelledEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiCancelledEutraItem {
    pub tai: Tai,
    pub cancelled_cells_in_tai_eutra: CancelledCellsInTaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiCancelledEutraItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledNr(Vec<TaiCancelledNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiCancelledNrItem {
    pub tai: Tai,
    pub cancelled_cells_in_tai_nr: CancelledCellsInTaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiCancelledNrItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TaiListForInactive(Vec<TaiListForInactiveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiListForInactiveItem {
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiListForInactiveItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TaiListForPaging(Vec<TaiListForPagingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiListForPagingItem {
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiListForPagingItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct TaiListForRestart(Vec<Tai>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiListForWarning(Vec<Tai>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TaiListforMdt(Vec<Tai>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TaListforMdt(Vec<Tac>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TnapId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TngfId {
    #[asn(key = 0, extended = false)]
    TngfId(BitString81),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(TngfIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TnlAddressWeightFactor(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TnlAssociationItem {
    pub tnl_association_address: CpTransportLayerInformation,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TnlAssociationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct TnlAssociationList(Vec<TnlAssociationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct TnlAssociationUsage(u8);
impl TnlAssociationUsage {
    const UE: u8 = 0u8;
    const NON_UE: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TscAssistanceInformation {
    pub periodicity: Periodicity,
    #[asn(optional_idx = 0)]
    pub burst_arrival_time: Option<BurstArrivalTime>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TscAssistanceInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TscTrafficCharacteristics {
    #[asn(optional_idx = 0)]
    pub tsc_assistance_information_dl: Option<TscAssistanceInformation>,
    #[asn(optional_idx = 1)]
    pub tsc_assistance_information_ul: Option<TscAssistanceInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TscTrafficCharacteristicsIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TwapId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TwifId {
    #[asn(key = 0, extended = false)]
    TwifId(BitString82),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(TwifIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TargetId {
    #[asn(key = 0, extended = false)]
    TargetRanNodeId(TargetRanNodeId),
    #[asn(key = 1, extended = false)]
    TargeteNbId(TargeteNbId),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(TargetIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNgranNodeToSourceNgranNodeFailureTransparentContainer {
    pub cell_cag_information: CellCagInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<TargetNgranNodeToSourceNgranNodeFailureTransparentContainerIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNgranNodeToSourceNgranNodeTransparentContainer {
    pub rrc_container: RrcContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetNgranNodeToSourceNgranNodeTransparentContainerIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetRanNodeId {
    pub global_ran_node_id: GlobalRanNodeId,
    pub selected_tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetRanNodeIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRncId {
    pub lai: Lai,
    pub rnc_id: RncId,
    #[asn(optional_idx = 0)]
    pub extended_rnc_id: Option<ExtendedRncId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRncIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetToSourceTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNbId {
    pub global_enb_id: GlobalNgEnbId,
    pub selected_eps_tai: EpsTai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargeteNbIdIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargettoSourceFailureTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdRsrp(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdRsrq(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdSinr(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct TimeStamp(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "15")]
pub struct TimeToTrigger(u8);
impl TimeToTrigger {
    const MS0: u8 = 0u8;
    const MS40: u8 = 1u8;
    const MS64: u8 = 2u8;
    const MS80: u8 = 3u8;
    const MS100: u8 = 4u8;
    const MS128: u8 = 5u8;
    const MS160: u8 = 6u8;
    const MS256: u8 = 7u8;
    const MS320: u8 = 8u8;
    const MS480: u8 = 9u8;
    const MS512: u8 = 10u8;
    const MS640: u8 = 11u8;
    const MS1024: u8 = 12u8;
    const MS1280: u8 = 13u8;
    const MS2560: u8 = 14u8;
    const MS5120: u8 = 15u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TimeToWait(u8);
impl TimeToWait {
    const V1S: u8 = 0u8;
    const V2S: u8 = 1u8;
    const V5S: u8 = 2u8;
    const V10S: u8 = 3u8;
    const V20S: u8 = 4u8;
    const V60S: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct TimeUeStayedInCell(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "40950")]
pub struct TimeUeStayedInCellEnhancedGranularity(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TimerApproachForGuamiRemoval(u8);
impl TimerApproachForGuamiRemoval {
    const APPLY_TIMER: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TooearlyIntersystemHo {
    pub sourcecell_id: EutraCgi,
    pub failurecell_id: NgranCgi,
    #[asn(optional_idx = 0)]
    pub uerlf_report_container: Option<UerlfReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TooearlyIntersystemHoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceActivation {
    pub ngran_trace_id: NgranTraceId,
    pub interfaces_to_trace: InterfacesToTrace,
    pub trace_depth: TraceDepth,
    pub trace_collection_entity_ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TraceActivationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TraceDepth(u8);
impl TraceDepth {
    const MINIMUM: u8 = 0u8;
    const MEDIUM: u8 = 1u8;
    const MAXIMUM: u8 = 2u8;
    const MINIMUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 3u8;
    const MEDIUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 4u8;
    const MAXIMUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TraceFailureIndication {
    pub protocol_i_es: TraceFailureIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TraceStart {
    pub protocol_i_es: TraceStartProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "99")]
pub struct TrafficLoadReductionIndication(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "1", sz_ub = "160")]
pub struct TransportLayerAddress(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct TriggeringMessage(u8);
impl TriggeringMessage {
    const INITIATING_MESSAGE: u8 = 0u8;
    const SUCCESSFUL_OUTCOME: u8 = 1u8;
    const UNSUCCESSFULL_OUTCOME: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TypeOfError(u8);
impl TypeOfError {
    const NOT_UNDERSTOOD: u8 = 0u8;
    const MISSING: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct UeDifferentiationInfo {
    #[asn(optional_idx = 0)]
    pub periodic_communication_indicator: Option<Enumerated83>,
    #[asn(optional_idx = 1)]
    pub periodic_time: Option<Integer84>,
    #[asn(optional_idx = 2)]
    pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
    #[asn(optional_idx = 3)]
    pub stationary_indication: Option<Enumerated85>,
    #[asn(optional_idx = 4)]
    pub traffic_profile: Option<Enumerated86>,
    #[asn(optional_idx = 5)]
    pub battery_indication: Option<Enumerated87>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<UeDifferentiationInfoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeNgapIdPair {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_ue_ngap_id: RanUeNgapId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UeNgapIdPairIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UeNgapIDs {
    #[asn(key = 0, extended = false)]
    UeNgapIdPair(UeNgapIdPair),
    #[asn(key = 1, extended = false)]
    AmfUeNgapId(AmfUeNgapId),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(UeNgapIDschoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeUpCIoTSupport(u8);
impl UeUpCIoTSupport {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UeAssociatedLogicalNgConnectionItem {
    #[asn(optional_idx = 0)]
    pub amf_ue_ngap_id: Option<AmfUeNgapId>,
    #[asn(optional_idx = 1)]
    pub ran_ue_ngap_id: Option<RanUeNgapId>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UeAssociatedLogicalNgConnectionItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65536")]
pub struct UeAssociatedLogicalNgConnectionList(Vec<UeAssociatedLogicalNgConnectionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeAggregateMaximumBitRate {
    pub ue_aggregate_maximum_bit_rate_dl: BitRate,
    pub ue_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UeAggregateMaximumBitRateIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeCapabilityInfoRequest(u8);
impl UeCapabilityInfoRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationFailure {
    pub protocol_i_es: UeContextModificationFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationRequest {
    pub protocol_i_es: UeContextModificationRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationResponse {
    pub protocol_i_es: UeContextModificationResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseCommand {
    pub protocol_i_es: UeContextReleaseCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseComplete {
    pub protocol_i_es: UeContextReleaseCompleteProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseRequest {
    pub protocol_i_es: UeContextReleaseRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeContextRequest(u8);
impl UeContextRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextResumeFailure {
    pub protocol_i_es: UeContextResumeFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextResumeRequest {
    pub protocol_i_es: UeContextResumeRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UeContextResumeRequestTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UeContextResumeRequestTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextResumeResponse {
    pub protocol_i_es: UeContextResumeResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UeContextResumeResponseTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UeContextResumeResponseTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSuspendFailure {
    pub protocol_i_es: UeContextSuspendFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSuspendRequest {
    pub protocol_i_es: UeContextSuspendRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UeContextSuspendRequestTransfer {
    #[asn(optional_idx = 0)]
    pub suspend_indicator: Option<SuspendIndicator>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UeContextSuspendRequestTransferIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSuspendResponse {
    pub protocol_i_es: UeContextSuspendResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct UeHistoryInformation(Vec<LastVisitedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UeHistoryInformationFromTheUe {
    #[asn(key = 0, extended = false)]
    Nr(NrMobilityHistoryReport),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UeHistoryInformationFromTheUEchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UeIdentityIndexValue {
    #[asn(key = 0, extended = false)]
    IndexLength10(BitString88),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UeIdentityIndexValuechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeInformationTransfer {
    pub protocol_i_es: UeInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UePagingIdentity {
    #[asn(key = 0, extended = false)]
    FiveGSTmsi(FiveGSTmsi),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UePagingIdentitychoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct UePresence(u8);
impl UePresence {
    const IN: u8 = 0u8;
    const OUT: u8 = 1u8;
    const UNKNOWN: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UePresenceInAreaOfInterestItem {
    pub location_reporting_reference_id: LocationReportingReferenceId,
    pub ue_presence: UePresence,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UePresenceInAreaOfInterestItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct UePresenceInAreaOfInterestList(Vec<UePresenceInAreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UerlfReportContainer {
    #[asn(key = 0, extended = false)]
    Nr(NruerlfReportContainer),
    #[asn(key = 1, extended = false)]
    Lte(LteuerlfReportContainer),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(UerlfReportContainerchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapability(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityCheckRequest {
    pub protocol_i_es: UeRadioCapabilityCheckRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityCheckResponse {
    pub protocol_i_es: UeRadioCapabilityCheckResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UeRadioCapabilityForPaging {
    #[asn(optional_idx = 0)]
    pub ue_radio_capability_for_paging_of_nr: Option<UeRadioCapabilityForPagingOfNr>,
    #[asn(optional_idx = 1)]
    pub ue_radio_capability_for_paging_of_eutra: Option<UeRadioCapabilityForPagingOfEutra>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UeRadioCapabilityForPagingIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityForPagingOfEutra(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityForPagingOfNbIoT(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityForPagingOfNr(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityIdMappingRequest {
    pub protocol_i_es: UeRadioCapabilityIdMappingRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityIdMappingResponse {
    pub protocol_i_es: UeRadioCapabilityIdMappingResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityInfoIndication {
    pub protocol_i_es: UeRadioCapabilityInfoIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeRetentionInformation(u8);
impl UeRetentionInformation {
    const UES_RETAINED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeSecurityCapabilities {
    pub n_rencryption_algorithms: NRencryptionAlgorithms,
    pub n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms,
    pub eutr_aencryption_algorithms: EutrAencryptionAlgorithms,
    pub eutr_aintegrity_protection_algorithms: EutrAintegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UeSecurityCapabilitiesIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UetnlaBindingReleaseRequest {
    pub protocol_i_es: UetnlaBindingReleaseRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UlCpSecurityInformation {
    pub ul_nas_mac: UlNasMac,
    pub ul_nas_count: UlNasCount,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlCpSecurityInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "5", sz_ub = "5")]
pub struct UlNasCount(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct UlNasMac(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UlNguUpTnlModifyItem {
    pub ul_ngu_up_tnl_information: UpTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlNguUpTnlModifyItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct UlNguUpTnlModifyList(Vec<UlNguUpTnlModifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UlForwarding(u8);
impl UlForwarding {
    const UL_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UpTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    GtpTunnel(GtpTunnel),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UpTransportLayerInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UpTransportLayerInformationItem {
    pub ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UpTransportLayerInformationItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UpTransportLayerInformationList(Vec<UpTransportLayerInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UpTransportLayerInformationPairItem {
    pub ul_ngu_up_tnl_information: UpTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UpTransportLayerInformationPairItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UpTransportLayerInformationPairList(Vec<UpTransportLayerInformationPairItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct UriAddress(String);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UnavailableGuamiItem {
    pub guami: Guami,
    #[asn(optional_idx = 0)]
    pub timer_approach_for_guami_removal: Option<TimerApproachForGuamiRemoval>,
    #[asn(optional_idx = 1)]
    pub backup_amf_name: Option<AmfName>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UnavailableGuamiItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct UnavailableGuamiList(Vec<UnavailableGuamiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct UpdateFeedback(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNasTransport {
    pub protocol_i_es: UplinkNasTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNonUeAssociatedNrpPaTransport {
    pub protocol_i_es: UplinkNonUeAssociatedNrpPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRanConfigurationTransfer {
    pub protocol_i_es: UplinkRanConfigurationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRanEarlyStatusTransfer {
    pub protocol_i_es: UplinkRanEarlyStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRanStatusTransfer {
    pub protocol_i_es: UplinkRanStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRimInformationTransfer {
    pub protocol_i_es: UplinkRimInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkUeAssociatedNrpPaTransport {
    pub protocol_i_es: UplinkUeAssociatedNrpPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum UserLocationInformation {
    #[asn(key = 0, extended = false)]
    UserLocationInformationEutra(UserLocationInformationEutra),
    #[asn(key = 1, extended = false)]
    UserLocationInformationNr(UserLocationInformationNr),
    #[asn(key = 2, extended = false)]
    UserLocationInformationN3iwf(UserLocationInformationN3iwf),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(UserLocationInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationEutra {
    pub eutra_cgi: EutraCgi,
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationEutraIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformationN3iwf {
    pub ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserLocationInformationN3iwfIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationNr {
    pub nr_cgi: NrCgi,
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationNrIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTngf {
    pub tnap_id: TnapId,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTngfIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTwif {
    pub twap_id: TwapId,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTwifIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UserLocationInformationWAgf {
    #[asn(key = 0, extended = false)]
    GlobalLineId(GlobalLineId),
    #[asn(key = 1, extended = false)]
    HfcNodeId(HfcNodeId),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(UserLocationInformationWAgFchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneSecurityInformation {
    pub security_result: SecurityResult,
    pub security_indication: SecurityIndication,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserPlaneSecurityInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VehicleUe(u8);
impl VehicleUe {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct VolumeTimedReportItem {
    pub start_time_stamp: OctetString89,
    pub end_time_stamp: OctetString90,
    pub usage_count_ul: Integer91,
    pub usage_count_dl: Integer92,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<VolumeTimedReportItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct VolumeTimedReportList(Vec<VolumeTimedReportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum WAgfId {
    #[asn(key = 0, extended = false)]
    WAgfId(BitString93),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(WAgfIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct WlanMeasConfig(u8);
impl WlanMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WlanMeasConfigNameItem {
    pub wlan_name: WlanName,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WlanMeasConfigNameItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct WlanMeasConfigNameList(Vec<WlanMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    #[asn(optional_idx = 0)]
    pub wlan_meas_config_name_list: Option<WlanMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub wlan_rssi: Option<Enumerated94>,
    #[asn(optional_idx = 2)]
    pub wlan_rtt: Option<Enumerated95>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<WlanMeasurementConfigurationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct WlanName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WusAssistanceInformation {
    pub paging_probability_information: PagingProbabilityInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WusAssistanceInformationIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct WarningAreaCoordinates(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum WarningAreaList {
    #[asn(key = 0, extended = false)]
    EutraCgiListForWarning(EutraCgiListForWarning),
    #[asn(key = 1, extended = false)]
    NrCgiListForWarning(NrCgiListForWarning),
    #[asn(key = 2, extended = false)]
    TaiListForWarning(TaiListForWarning),
    #[asn(key = 3, extended = false)]
    EmergencyAreaIdList(EmergencyAreaIdList),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(WarningAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "9600")]
pub struct WarningMessageContents(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "50", sz_ub = "50")]
pub struct WarningSecurityInfo(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct WarningType(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningRequest {
    pub protocol_i_es: WriteReplaceWarningRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningResponse {
    pub protocol_i_es: WriteReplaceWarningResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct XnExtTlaItem {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub gtp_tl_as: Option<XnGtpTlAs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<XnExtTlaItemIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnExtTlAs(Vec<XnExtTlaItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnGtpTlAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct XnTlAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct XnTnlConfigurationInfo {
    pub xn_transport_layer_addresses: XnTlAs,
    #[asn(optional_idx = 0)]
    pub xn_extended_transport_layer_addresses: Option<XnExtTlAs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<XnTnlConfigurationInfoIeExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationSetupItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationSetupItemIeExtensions(Vec<AmfTnlAssociationSetupItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationToAddItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationToAddItemIeExtensions(Vec<AmfTnlAssociationToAddItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfTnlAssociationToRemoveItemIeExtensionsItemExtensionValue {
    #[asn(key = 168)]
    IdTnlAssociationTransportLayerAddressNgran(CpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationToRemoveItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AmfTnlAssociationToRemoveItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationToRemoveItemIeExtensions(Vec<AmfTnlAssociationToRemoveItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationToUpdateItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationToUpdateItemIeExtensions(Vec<AmfTnlAssociationToUpdateItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfcpRelocationIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 148)]
    IdSNssai(SNssai),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfcpRelocationIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfcpRelocationIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfcpRelocationIndicationProtocolIEs(Vec<AmfcpRelocationIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfConfigurationUpdateProtocolIEsItemValue {
    #[asn(key = 6)]
    IdAmfTnlAssociationToAddList(AmfTnlAssociationToAddList),
    #[asn(key = 7)]
    IdAmfTnlAssociationToRemoveList(AmfTnlAssociationToRemoveList),
    #[asn(key = 8)]
    IdAmfTnlAssociationToUpdateList(AmfTnlAssociationToUpdateList),
    #[asn(key = 1)]
    IdAmfName(AmfName),
    #[asn(key = 274)]
    IdExtendedAmfName(ExtendedAmfName),
    #[asn(key = 80)]
    IdPlmnSupportList(PlmnSupportList),
    #[asn(key = 86)]
    IdRelativeAmfCapacity(RelativeAmfCapacity),
    #[asn(key = 96)]
    IdServedGuamiList(ServedGuamiList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfConfigurationUpdateProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfConfigurationUpdateProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfConfigurationUpdateProtocolIEs(Vec<AmfConfigurationUpdateProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfConfigurationUpdateAcknowledgeProtocolIEsItemValue {
    #[asn(key = 4)]
    IdAmfTnlAssociationFailedToSetupList(TnlAssociationList),
    #[asn(key = 5)]
    IdAmfTnlAssociationSetupList(AmfTnlAssociationSetupList),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfConfigurationUpdateAcknowledgeProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfConfigurationUpdateAcknowledgeProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfConfigurationUpdateAcknowledgeProtocolIEs(Vec<AmfConfigurationUpdateAcknowledgeProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfConfigurationUpdateFailureProtocolIEsItemValue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    IdTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfConfigurationUpdateFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfConfigurationUpdateFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfConfigurationUpdateFailureProtocolIEs(Vec<AmfConfigurationUpdateFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfPagingTargetchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfStatusIndicationProtocolIEsItemValue {
    #[asn(key = 120)]
    IdUnavailableGuamiList(UnavailableGuamiList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfStatusIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfStatusIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfStatusIndicationProtocolIEs(Vec<AmfStatusIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AdditionalDluptnlInformationForHoItemIeExtensionsItemExtensionValue {
    #[asn(key = 183)]
    IdAdditionalRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalDluptnlInformationForHoItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AdditionalDluptnlInformationForHoItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AdditionalDluptnlInformationForHoItemIeExtensions(Vec<AdditionalDluptnlInformationForHoItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllocationAndRetentionPriorityIeExtensions(Vec<AllocationAndRetentionPriorityIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated2(u8);
impl Enumerated2 {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedPniNpnItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllowedPniNpnItemIeExtensions(Vec<AllowedPniNpnItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedNssaiItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllowedNssaiItemIeExtensions(Vec<AllowedNssaiItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AlternativeQoSParaSetItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AlternativeQoSParaSetItemIeExtensions(Vec<AlternativeQoSParaSetItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestIeExtensions(Vec<AreaOfInterestIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestCellItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestCellItemIeExtensions(Vec<AreaOfInterestCellItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestItemIeExtensions(Vec<AreaOfInterestItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestRanNodeItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestRanNodeItemIeExtensions(Vec<AreaOfInterestRanNodeItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestTaiItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestTaiItemIeExtensions(Vec<AreaOfInterestTaiItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMdtEutrAchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null4;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMdtNRchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfNeighCellsItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaScopeOfNeighCellsItemIeExtensions(Vec<AreaScopeOfNeighCellsItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssistanceDataForPagingIeExtensionsItemExtensionValue {
    #[asn(key = 260)]
    IdNpnPagingAssistanceInformation(NpnPagingAssistanceInformation),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AssistanceDataForPagingIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssistanceDataForPagingIeExtensions(Vec<AssistanceDataForPagingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForRecommendedCellsIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssistanceDataForRecommendedCellsIeExtensions(Vec<AssistanceDataForRecommendedCellsIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated5(u8);
impl Enumerated5 {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssociatedQosFlowItemIeExtensionsItemExtensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedQosFlowItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AssociatedQosFlowItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssociatedQosFlowItemIeExtensions(Vec<AssociatedQosFlowItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasConfigNameItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BluetoothMeasConfigNameItemIeExtensions(Vec<BluetoothMeasConfigNameItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated6(u8);
impl Enumerated6 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasurementConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BluetoothMeasurementConfigurationIeExtensions(Vec<BluetoothMeasurementConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCancelledAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCompletedAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BroadcastPlmnItemIeExtensionsItemExtensionValue {
    #[asn(key = 271)]
    IdExtendedTaiSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    IdNpnSupport(NpnSupport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastPlmnItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: BroadcastPlmnItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BroadcastPlmnItemIeExtensions(Vec<BroadcastPlmnItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CnAssistedRanTuningIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CnAssistedRanTuningIeExtensions(Vec<CnAssistedRanTuningIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated7(u8);
impl Enumerated7 {
    const EPC_FORBIDDEN: u8 = 0u8;
    const FIVE_GC_FORBIDDEN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CnTypeRestrictionsForEquivalentItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CnTypeRestrictionsForEquivalentItemIeExtensions(Vec<CnTypeRestrictionsForEquivalentItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer8(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct Integer9(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CountValueForPdcpSn12IeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CountValueForPdcpSn12IeExtensions(Vec<CountValueForPdcpSn12IeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct Integer10(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct Integer11(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CountValueForPdcpSn18IeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CountValueForPdcpSn18IeExtensions(Vec<CountValueForPdcpSn18IeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CpTransportLayerInformationchoiceExtensionsValue {
    #[asn(key = 169)]
    IdEndpointIpAddressAndPort(EndpointIpAddressAndPort),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CpTransportLayerInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CpTransportLayerInformationchoiceExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEaiEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiEutraItemIeExtensions(Vec<CancelledCellsInEaiEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEaiNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiNrItemIeExtensions(Vec<CancelledCellsInEaiNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTaiEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiEutraItemIeExtensions(Vec<CancelledCellsInTaiEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTaiNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiNrItemIeExtensions(Vec<CancelledCellsInTaiNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidateCellIdIeExtensions(Vec<CandidateCellIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidateCellItemIeExtensions(Vec<CandidateCellItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct Integer12(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer13(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidatePciIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidatePciIeExtensions(Vec<CandidatePciIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CausechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellCagInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellCagInformationIeExtensions(Vec<CellCagInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMdtEutraIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellBasedMdtEutraIeExtensions(Vec<CellBasedMdtEutraIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMdtNrIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellBasedMdtNrIeExtensions(Vec<CellBasedMdtNrIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdBroadcastEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastEutraItemIeExtensions(Vec<CellIdBroadcastEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdBroadcastNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastNrItemIeExtensions(Vec<CellIdBroadcastNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdCancelledEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledEutraItemIeExtensions(Vec<CellIdCancelledEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdCancelledNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledNrItemIeExtensions(Vec<CellIdCancelledNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdListForRestartchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 43)]
    IdNgranCgi(NgranCgi),
    #[asn(key = 44)]
    IdNgranTraceId(NgranTraceId),
    #[asn(key = 256)]
    IdPrivacyIndicator(PrivacyIndicator),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 109)]
    IdTraceCollectionEntityIpAddress(TransportLayerAddress),
    #[asn(key = 257)]
    IdTraceCollectionEntityUri(UriAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTrafficTraceProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellTrafficTraceProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct CellTrafficTraceProtocolIEs(Vec<CellTrafficTraceProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTypeIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellTypeIeExtensions(Vec<CellTypeIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEaiEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiEutraItemIeExtensions(Vec<CompletedCellsInEaiEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEaiNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiNrItemIeExtensions(Vec<CompletedCellsInEaiNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTaiEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiEutraItemIeExtensions(Vec<CompletedCellsInTaiEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTaiNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiNrItemIeExtensions(Vec<CompletedCellsInTaiNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 212)]
    IdDlCpSecurityInformation(DlCpSecurityInformation),
    #[asn(key = 226)]
    IdEndIndication(EndIndication),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 210)]
    IdNbIoTUePriority(NbIoTUePriority),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 148)]
    IdSNssai(SNssai),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectionEstablishmentIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ConnectionEstablishmentIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct ConnectionEstablishmentIndicationProtocolIEs(Vec<ConnectionEstablishmentIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CoreNetworkAssistanceInformationForInactiveIeExtensionsItemExtensionValue {
    #[asn(key = 280)]
    IdExtendedUeIdentityIndexValue(ExtendedUeIdentityIndexValue),
    #[asn(key = 223)]
    IdPagingeDrxInformation(PagingeDrxInformation),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CoreNetworkAssistanceInformationForInactiveIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: CoreNetworkAssistanceInformationForInactiveIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CoreNetworkAssistanceInformationForInactiveIeExtensions(Vec<CoreNetworkAssistanceInformationForInactiveIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CriticalityDiagnosticsIeExtensions(Vec<CriticalityDiagnosticsIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIeItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CriticalityDiagnosticsIeItemIeExtensions(Vec<CriticalityDiagnosticsIeItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated14(u8);
impl Enumerated14 {
    const DAPS_HO_REQUIRED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DapsRequestInfoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DapsRequestInfoIeExtensions(Vec<DapsRequestInfoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated15(u8);
impl Enumerated15 {
    const DAPS_HO_ACCEPTED: u8 = 0u8;
    const DAPS_HO_NOT_ACCEPTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DapsResponseInfoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DapsResponseInfoIeExtensions(Vec<DapsResponseInfoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DapsResponseInfoItemIeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DapsResponseInfoItemIeExtension(Vec<DapsResponseInfoItemIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlCpSecurityInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DlCpSecurityInformationIeExtensions(Vec<DlCpSecurityInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusDLchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusDl12IeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusDl12IeExtension(Vec<DrbStatusDl12IeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusDl18IeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusDl18IeExtension(Vec<DrbStatusDl18IeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusULchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct BitString16(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusUl12IeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusUl12IeExtension(Vec<DrbStatusUl12IeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "131072")]
pub struct BitString17(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusUl18IeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusUl18IeExtension(Vec<DrbStatusUl18IeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSubjectToEarlyStatusTransferItemIeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrBsSubjectToEarlyStatusTransferItemIeExtension(Vec<DrBsSubjectToEarlyStatusTransferItemIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsSubjectToStatusTransferItemIeExtensionItemExtensionValue {
    #[asn(key = 159)]
    IdOldAssociatedQosFlowListULendmarkerexpected(AssociatedQosFlowList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSubjectToStatusTransferItemIeExtensionItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsSubjectToStatusTransferItemIeExtensionItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrBsSubjectToStatusTransferItemIeExtension(Vec<DrBsSubjectToStatusTransferItemIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToQosFlowsMappingItemIeExtensionsItemExtensionValue {
    #[asn(key = 266)]
    IdDapsRequestInfo(DapsRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToQosFlowsMappingItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsToQosFlowsMappingItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrBsToQosFlowsMappingItemIeExtensions(Vec<DrBsToQosFlowsMappingItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseDrbItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DataForwardingResponseDrbItemIeExtensions(Vec<DataForwardingResponseDrbItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseErabListItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DataForwardingResponseErabListItemIeExtensions(Vec<DataForwardingResponseErabListItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 44)]
    IdNgranTraceId(NgranTraceId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeactivateTraceProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DeactivateTraceProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DeactivateTraceProtocolIEs(Vec<DeactivateTraceProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNasTransportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 226)]
    IdEndIndication(EndIndication),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 31)]
    IdIndexToRfsp(IndexToRfsp),
    #[asn(key = 36)]
    IdMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 48)]
    IdOldAmf(AmfName),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 228)]
    IdUeCapabilityInfoRequest(UeCapabilityInfoRequest),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNasTransportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkNasTransportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkNasTransportProtocolIEs(Vec<DownlinkNasTransportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNonUeAssociatedNrpPaTransportProtocolIEsItemValue {
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUeAssociatedNrpPaTransportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkNonUeAssociatedNrpPaTransportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkNonUeAssociatedNrpPaTransportProtocolIEs(Vec<DownlinkNonUeAssociatedNrpPaTransportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRanConfigurationTransferProtocolIEsItemValue {
    #[asn(key = 157)]
    IdEndcSonConfigurationTransferDl(EnDcsonConfigurationTransfer),
    #[asn(key = 250)]
    IdIntersystemSonConfigurationTransferDl(IntersystemSonConfigurationTransfer),
    #[asn(key = 98)]
    IdSonConfigurationTransferDl(SonConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRanConfigurationTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRanConfigurationTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRanConfigurationTransferProtocolIEs(Vec<DownlinkRanConfigurationTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRanEarlyStatusTransferProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 268)]
    IdEarlyStatusTransferTransparentContainer(EarlyStatusTransferTransparentContainer),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRanEarlyStatusTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRanEarlyStatusTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRanEarlyStatusTransferProtocolIEs(Vec<DownlinkRanEarlyStatusTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRanStatusTransferProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 84)]
    IdRanStatusTransferTransparentContainer(RanStatusTransferTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRanStatusTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRanStatusTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRanStatusTransferProtocolIEs(Vec<DownlinkRanStatusTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRimInformationTransferProtocolIEsItemValue {
    #[asn(key = 175)]
    IdRimInformationTransfer(RimInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRimInformationTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRimInformationTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRimInformationTransferProtocolIEs(Vec<DownlinkRimInformationTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUeAssociatedNrpPaTransportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkUeAssociatedNrpPaTransportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkUeAssociatedNrpPaTransportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkUeAssociatedNrpPaTransportProtocolIEs(Vec<DownlinkUeAssociatedNrpPaTransportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Dynamic5QiDescriptorIeExtensionsItemExtensionValue {
    #[asn(key = 187)]
    IdCnPacketDelayBudgetDl(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    IdCnPacketDelayBudgetUl(ExtendedPacketDelayBudget),
    #[asn(key = 189)]
    IdExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Dynamic5QiDescriptorIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: Dynamic5QiDescriptorIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Dynamic5QiDescriptorIeExtensions(Vec<Dynamic5QiDescriptorIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ERabInformationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ERabInformationItemIeExtensions(Vec<ERabInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BitString18(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BitString19(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BitString20(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BitString21(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnbIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EpsTaiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EpsTaiIeExtensions(Vec<EpsTaiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraCgiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EutraCgiIeExtensions(Vec<EutraCgiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EarlyStatusTransferTransparentContainerIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EarlyStatusTransferTransparentContainerIeExtensions(Vec<EarlyStatusTransferTransparentContainerIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdBroadcastEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastEutraItemIeExtensions(Vec<EmergencyAreaIdBroadcastEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdBroadcastNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastNrItemIeExtensions(Vec<EmergencyAreaIdBroadcastNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdCancelledEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledEutraItemIeExtensions(Vec<EmergencyAreaIdCancelledEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdCancelledNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledNrItemIeExtensions(Vec<EmergencyAreaIdCancelledNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyFallbackIndicatorIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyFallbackIndicatorIeExtensions(Vec<EmergencyFallbackIndicatorIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EndpointIpAddressAndPortIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EndpointIpAddressAndPortIeExtensions(Vec<EndpointIpAddressAndPortIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ErrorIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct ErrorIndicationProtocolIEs(Vec<ErrorIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventL1LoggedMdtConfigIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EventL1LoggedMdtConfigIeExtensions(Vec<EventL1LoggedMdtConfigIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated22(u8);
impl Enumerated22 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventTriggerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUeActivityBehaviourIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUeActivityBehaviourIeExtensions(Vec<ExpectedUeActivityBehaviourIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUeBehaviourIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUeBehaviourIeExtensions(Vec<ExpectedUeBehaviourIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer23(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUeMovingTrajectoryItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUeMovingTrajectoryItemIeExtensions(Vec<ExpectedUeMovingTrajectoryItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedAmfNameIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedAmfNameIeExtensions(Vec<ExtendedAmfNameIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRanNodeNameIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedRanNodeNameIeExtensions(Vec<ExtendedRanNodeNameIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BitString24(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BitString25(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRatRestrictionInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedRatRestrictionInformationIeExtensions(Vec<ExtendedRatRestrictionInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FailureIndicationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FailureIndicationIeExtensions(Vec<FailureIndicationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FirstDlCountIeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FirstDlCountIeExtension(Vec<FirstDlCountIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveGSTmsiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FiveGSTmsiIeExtensions(Vec<FiveGSTmsiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenAreaInformationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ForbiddenAreaInformationItemIeExtensions(Vec<ForbiddenAreaInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromEutraNtoNgranIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FromEutraNtoNgranIeExtensions(Vec<FromEutraNtoNgranIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromNgraNtoEutranIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FromNgraNtoEutranIeExtensions(Vec<FromNgraNtoEutranIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GbrQosInformationIeExtensionsItemExtensionValue {
    #[asn(key = 220)]
    IdAlternativeQoSParaSetList(AlternativeQoSParaSetList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GbrQosInformationIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: GbrQosInformationIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GbrQosInformationIeExtensions(Vec<GbrQosInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct BitString26(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GtpTunnelIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GtpTunnelIeExtensions(Vec<GtpTunnelIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GuamiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GuamiIeExtensions(Vec<GuamiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalEnbIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalEnbIdIeExtensions(Vec<GlobalEnbIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalGnbIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalGnbIdIeExtensions(Vec<GlobalGnbIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalLineIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalLineIdIeExtensions(Vec<GlobalLineIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalN3iwfIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalN3iwfIdIeExtensions(Vec<GlobalN3iwfIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalNgEnbIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalNgEnbIdIeExtensions(Vec<GlobalNgEnbIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GlobalRanNodeIDchoiceExtensionsValue {
    #[asn(key = 240)]
    IdGlobalTngfId(GlobalTngfId),
    #[asn(key = 241)]
    IdGlobalTwifId(GlobalTwifId),
    #[asn(key = 242)]
    IdGlobalWAgfId(GlobalWAgfId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRanNodeIDchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GlobalRanNodeIDchoiceExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTngfIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalTngfIdIeExtensions(Vec<GlobalTngfIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTwifIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalTwifIdIeExtensions(Vec<GlobalTwifIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalWAgfIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalWAgfIdIeExtensions(Vec<GlobalWAgfIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated27(u8);
impl Enumerated27 {
    const HO_TOO_EARLY: u8 = 0u8;
    const HO_TO_WRONG_CELL: u8 = 1u8;
    const INTERSYSTEM_PING_PONG: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BitString28(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HoReportIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HoReportIeExtensions(Vec<HoReportIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverCancelProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCancelProtocolIEs(Vec<HandoverCancelProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelAcknowledgeProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelAcknowledgeProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverCancelAcknowledgeProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCancelAcknowledgeProtocolIEs(Vec<HandoverCancelAcknowledgeProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    IdHandoverType(HandoverType),
    #[asn(key = 39)]
    IdNasSecurityParametersFromNgran(NasSecurityParametersFromNgran),
    #[asn(key = 59)]
    IdPduSessionResourceHandoverList(PduSessionResourceHandoverList),
    #[asn(key = 78)]
    IdPduSessionResourceToReleaseListHoCmd(PduSessionResourceToReleaseListHoCmd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 106)]
    IdTargetToSourceTransparentContainer(TargetToSourceTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverCommandProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCommandProtocolIEs(Vec<HandoverCommandProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandTransferIeExtensionsItemExtensionValue {
    #[asn(key = 152)]
    IdAdditionalDlForwardingUptnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 172)]
    IdAdditionalUlForwardingUptnlInformation(UpTransportLayerInformationList),
    #[asn(key = 249)]
    IdDataForwardingResponseErabList(DataForwardingResponseErabList),
    #[asn(key = 164)]
    IdUlForwardingUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: HandoverCommandTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverCommandTransferIeExtensions(Vec<HandoverCommandTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 262)]
    IdTargettoSourceFailureTransparentContainer(TargettoSourceFailureTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverFailureProtocolIEs(Vec<HandoverFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverNotifyProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 269)]
    IdNotifySourceNgranNode(NotifySourceNgranNode),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverNotifyProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverNotifyProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverNotifyProtocolIEs(Vec<HandoverNotifyProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverPreparationFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 262)]
    IdTargettoSourceFailureTransparentContainer(TargettoSourceFailureTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverPreparationFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverPreparationFailureProtocolIEs(Vec<HandoverPreparationFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationUnsuccessfulTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverPreparationUnsuccessfulTransferIeExtensions(Vec<HandoverPreparationUnsuccessfulTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 28)]
    IdGuami(Guami),
    #[asn(key = 29)]
    IdHandoverType(HandoverType),
    #[asn(key = 199)]
    IdIabAuthorized(IabAuthorized),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    IdManagementBasedMdtplmnList(MdtplmnList),
    #[asn(key = 34)]
    IdMaskedImeisv(MaskedImeisv),
    #[asn(key = 36)]
    IdMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 37)]
    IdNasc(NasPdu),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 41)]
    IdNewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 73)]
    IdPduSessionResourceSetupListHoReq(PduSessionResourceSetupListHoReq),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 146)]
    IdRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
    #[asn(key = 101)]
    IdSourceToTargetTransparentContainer(SourceToTargetTransparentContainer),
    #[asn(key = 108)]
    IdTraceActivation(TraceActivation),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 234)]
    IdUeUpCIoTSupport(UeUpCIoTSupport),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequestProtocolIEs(Vec<HandoverRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 53)]
    IdPduSessionResourceAdmittedList(PduSessionResourceAdmittedList),
    #[asn(key = 56)]
    IdPduSessionResourceFailedToSetupListHoAck(PduSessionResourceFailedToSetupListHoAck),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 106)]
    IdTargetToSourceTransparentContainer(TargetToSourceTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverRequestAcknowledgeProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequestAcknowledgeProtocolIEs(Vec<HandoverRequestAcknowledgeProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeTransferIeExtensionsItemExtensionValue {
    #[asn(key = 153)]
    IdAdditionalDluptnlInformationForHoList(AdditionalDluptnlInformationForHoList),
    #[asn(key = 172)]
    IdAdditionalUlForwardingUptnlInformation(UpTransportLayerInformationList),
    #[asn(key = 249)]
    IdDataForwardingResponseErabList(DataForwardingResponseErabList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 164)]
    IdUlForwardingUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 198)]
    IdUsedRsnInformation(RedundantPduSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: HandoverRequestAcknowledgeTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverRequestAcknowledgeTransferIeExtensions(Vec<HandoverRequestAcknowledgeTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 22)]
    IdDirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 29)]
    IdHandoverType(HandoverType),
    #[asn(key = 61)]
    IdPduSessionResourceListHoRqd(PduSessionResourceListHoRqd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 101)]
    IdSourceToTargetTransparentContainer(SourceToTargetTransparentContainer),
    #[asn(key = 105)]
    IdTargetId(TargetId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverRequiredProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequiredProtocolIEs(Vec<HandoverRequiredProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverRequiredTransferIeExtensions(Vec<HandoverRequiredTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverResourceAllocationUnsuccessfulTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverResourceAllocationUnsuccessfulTransferIeExtensions(Vec<HandoverResourceAllocationUnsuccessfulTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverSuccessProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverSuccessProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverSuccessProtocolIEs(Vec<HandoverSuccessProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMdtNrIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ImmediateMdtNrIeExtensions(Vec<ImmediateMdtNrIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InfoOnRecommendedCellsAndRanNodesForPagingIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InfoOnRecommendedCellsAndRanNodesForPagingIeExtensions(Vec<InfoOnRecommendedCellsAndRanNodesForPagingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 132)]
    IdPduSessionResourceFailedToSetupListCxtFail(PduSessionResourceFailedToSetupListCxtFail),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialContextSetupFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupFailureProtocolIEs(Vec<InitialContextSetupFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    IdEmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 28)]
    IdGuami(Guami),
    #[asn(key = 199)]
    IdIabAuthorized(IabAuthorized),
    #[asn(key = 31)]
    IdIndexToRfsp(IndexToRfsp),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    IdManagementBasedMdtplmnList(MdtplmnList),
    #[asn(key = 34)]
    IdMaskedImeisv(MaskedImeisv),
    #[asn(key = 36)]
    IdMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 48)]
    IdOldAmf(AmfName),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 71)]
    IdPduSessionResourceSetupListCxtReq(PduSessionResourceSetupListCxtReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 238)]
    IdRgLevelWirelineAccessCharacteristics(RgLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 146)]
    IdRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 94)]
    IdSecurityKey(SecurityKey),
    #[asn(key = 108)]
    IdTraceActivation(TraceActivation),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 234)]
    IdUeUpCIoTSupport(UeUpCIoTSupport),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialContextSetupRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupRequestProtocolIEs(Vec<InitialContextSetupRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 55)]
    IdPduSessionResourceFailedToSetupListCxtRes(PduSessionResourceFailedToSetupListCxtRes),
    #[asn(key = 72)]
    IdPduSessionResourceSetupListCxtRes(PduSessionResourceSetupListCxtRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialContextSetupResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupResponseProtocolIEs(Vec<InitialContextSetupResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUeMessageProtocolIEsItemValue {
    #[asn(key = 3)]
    IdAmfSetId(AmfSetId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 245)]
    IdAuthenticatedIndication(AuthenticatedIndication),
    #[asn(key = 224)]
    IdCEmodeBSupportIndicator(CEmodeBSupportIndicator),
    #[asn(key = 227)]
    IdEdtSession(EdtSession),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 201)]
    IdIabNodeIndication(IabNodeIndication),
    #[asn(key = 225)]
    IdLtemIndication(LtemIndication),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 259)]
    IdNpnAccessInformation(NpnAccessInformation),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 90)]
    IdRrcEstablishmentCause(RrcEstablishmentCause),
    #[asn(key = 174)]
    IdSelectedPlmnIdentity(PlmnIdentity),
    #[asn(key = 171)]
    IdSourceToTargetAmfInformationReroute(SourceToTargetAmfInformationReroute),
    #[asn(key = 112)]
    IdUeContextRequest(UeContextRequest),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUeMessageProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialUeMessageProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialUeMessageProtocolIEs(Vec<InitialUeMessageProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessageValue {
    #[asn(key = 64)]
    IdAmfcpRelocationIndication(AmfcpRelocationIndication),
    #[asn(key = 0)]
    IdAmfConfigurationUpdate(AmfConfigurationUpdate),
    #[asn(key = 1)]
    IdAmfStatusIndication(AmfStatusIndication),
    #[asn(key = 2)]
    IdCellTrafficTrace(CellTrafficTrace),
    #[asn(key = 65)]
    IdConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 3)]
    IdDeactivateTrace(DeactivateTrace),
    #[asn(key = 4)]
    IdDownlinkNasTransport(DownlinkNasTransport),
    #[asn(key = 5)]
    IdDownlinkNonUeAssociatedNrpPaTransport(DownlinkNonUeAssociatedNrpPaTransport),
    #[asn(key = 6)]
    IdDownlinkRanConfigurationTransfer(DownlinkRanConfigurationTransfer),
    #[asn(key = 63)]
    IdDownlinkRanEarlyStatusTransfer(DownlinkRanEarlyStatusTransfer),
    #[asn(key = 7)]
    IdDownlinkRanStatusTransfer(DownlinkRanStatusTransfer),
    #[asn(key = 54)]
    IdDownlinkRimInformationTransfer(DownlinkRimInformationTransfer),
    #[asn(key = 8)]
    IdDownlinkUeAssociatedNrpPaTransport(DownlinkUeAssociatedNrpPaTransport),
    #[asn(key = 9)]
    IdErrorIndication(ErrorIndication),
    #[asn(key = 10)]
    IdHandoverCancel(HandoverCancel),
    #[asn(key = 11)]
    IdHandoverNotification(HandoverNotify),
    #[asn(key = 12)]
    IdHandoverPreparation(HandoverRequired),
    #[asn(key = 13)]
    IdHandoverResourceAllocation(HandoverRequest),
    #[asn(key = 61)]
    IdHandoverSuccess(HandoverSuccess),
    #[asn(key = 14)]
    IdInitialContextSetup(InitialContextSetupRequest),
    #[asn(key = 15)]
    IdInitialUeMessage(InitialUeMessage),
    #[asn(key = 18)]
    IdLocationReport(LocationReport),
    #[asn(key = 16)]
    IdLocationReportingControl(LocationReportingControl),
    #[asn(key = 17)]
    IdLocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 19)]
    IdNasNonDeliveryIndication(NasNonDeliveryIndication),
    #[asn(key = 20)]
    IdNgReset(NgReset),
    #[asn(key = 21)]
    IdNgSetup(NgSetupRequest),
    #[asn(key = 22)]
    IdOverloadStart(OverloadStart),
    #[asn(key = 23)]
    IdOverloadStop(OverloadStop),
    #[asn(key = 26)]
    IdPduSessionResourceModify(PduSessionResourceModifyRequest),
    #[asn(key = 27)]
    IdPduSessionResourceModifyIndication(PduSessionResourceModifyIndication),
    #[asn(key = 30)]
    IdPduSessionResourceNotify(PduSessionResourceNotify),
    #[asn(key = 28)]
    IdPduSessionResourceRelease(PduSessionResourceReleaseCommand),
    #[asn(key = 29)]
    IdPduSessionResourceSetup(PduSessionResourceSetupRequest),
    #[asn(key = 32)]
    IdPwsCancel(PwsCancelRequest),
    #[asn(key = 33)]
    IdPwsFailureIndication(PwsFailureIndication),
    #[asn(key = 34)]
    IdPwsRestartIndication(PwsRestartIndication),
    #[asn(key = 24)]
    IdPaging(Paging),
    #[asn(key = 25)]
    IdPathSwitchRequest(PathSwitchRequest),
    #[asn(key = 31)]
    IdPrivateMessage(PrivateMessage),
    #[asn(key = 57)]
    IdRancpRelocationIndication(RancpRelocationIndication),
    #[asn(key = 35)]
    IdRanConfigurationUpdate(RanConfigurationUpdate),
    #[asn(key = 37)]
    IdRrcInactiveTransitionReport(RrcInactiveTransitionReport),
    #[asn(key = 36)]
    IdRerouteNasRequest(RerouteNasRequest),
    #[asn(key = 55)]
    IdRetrieveUeInformation(RetrieveUeInformation),
    #[asn(key = 52)]
    IdSecondaryRatDataUsageReport(SecondaryRatDataUsageReport),
    #[asn(key = 38)]
    IdTraceFailureIndication(TraceFailureIndication),
    #[asn(key = 39)]
    IdTraceStart(TraceStart),
    #[asn(key = 40)]
    IdUeContextModification(UeContextModificationRequest),
    #[asn(key = 41)]
    IdUeContextRelease(UeContextReleaseCommand),
    #[asn(key = 42)]
    IdUeContextReleaseRequest(UeContextReleaseRequest),
    #[asn(key = 58)]
    IdUeContextResume(UeContextResumeRequest),
    #[asn(key = 59)]
    IdUeContextSuspend(UeContextSuspendRequest),
    #[asn(key = 56)]
    IdUeInformationTransfer(UeInformationTransfer),
    #[asn(key = 43)]
    IdUeRadioCapabilityCheck(UeRadioCapabilityCheckRequest),
    #[asn(key = 60)]
    IdUeRadioCapabilityIdMapping(UeRadioCapabilityIdMappingRequest),
    #[asn(key = 44)]
    IdUeRadioCapabilityInfoIndication(UeRadioCapabilityInfoIndication),
    #[asn(key = 45)]
    IdUetnlaBindingRelease(UetnlaBindingReleaseRequest),
    #[asn(key = 46)]
    IdUplinkNasTransport(UplinkNasTransport),
    #[asn(key = 47)]
    IdUplinkNonUeAssociatedNrpPaTransport(UplinkNonUeAssociatedNrpPaTransport),
    #[asn(key = 48)]
    IdUplinkRanConfigurationTransfer(UplinkRanConfigurationTransfer),
    #[asn(key = 62)]
    IdUplinkRanEarlyStatusTransfer(UplinkRanEarlyStatusTransfer),
    #[asn(key = 49)]
    IdUplinkRanStatusTransfer(UplinkRanStatusTransfer),
    #[asn(key = 53)]
    IdUplinkRimInformationTransfer(UplinkRimInformationTransfer),
    #[asn(key = 50)]
    IdUplinkUeAssociatedNrpPaTransport(UplinkUeAssociatedNrpPaTransport),
    #[asn(key = 51)]
    IdWriteReplaceWarning(WriteReplaceWarningRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemFailureIndicationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InterSystemFailureIndicationIeExtensions(Vec<InterSystemFailureIndicationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHoReportIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InterSystemHoReportIeExtensions(Vec<InterSystemHoReportIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHandoverReportTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonConfigurationTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSonConfigurationTransferIeExtensions(Vec<IntersystemSonConfigurationTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonInformationReportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonngraNnodeIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSonngraNnodeIdIeExtensions(Vec<IntersystemSonngraNnodeIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonTransferTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSoNeNbidIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSoNeNbidIeExtensions(Vec<IntersystemSoNeNbidIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated29(u8);
impl Enumerated29 {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemUnnecessaryHoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemUnnecessaryHoIeExtensions(Vec<IntersystemUnnecessaryHoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LaiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LaiIeExtensions(Vec<LaiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LteueSidelinkAggregateMaximumBitrateIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LteueSidelinkAggregateMaximumBitrateIeExtensions(Vec<LteueSidelinkAggregateMaximumBitrateIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ltev2xServicesAuthorizedIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Ltev2xServicesAuthorizedIeExtensions(Vec<Ltev2xServicesAuthorizedIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LastVisitedCellItemIeExtensions(Vec<LastVisitedCellItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedNgranCellInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LastVisitedNgranCellInformationIeExtensions(Vec<LastVisitedNgranCellInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 116)]
    IdUePresenceInAreaOfInterestList(UePresenceInAreaOfInterestList),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: LocationReportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportProtocolIEs(Vec<LocationReportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: LocationReportingControlProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportingControlProtocolIEs(Vec<LocationReportingControlProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingFailureIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingFailureIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: LocationReportingFailureIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportingFailureIndicationProtocolIEs(Vec<LocationReportingFailureIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingRequestTypeIeExtensionsItemExtensionValue {
    #[asn(key = 170)]
    IdLocationReportingAdditionalInfo(LocationReportingAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingRequestTypeIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: LocationReportingRequestTypeIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LocationReportingRequestTypeIeExtensions(Vec<LocationReportingRequestTypeIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMdtNrIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LoggedMdtNrIeExtensions(Vec<LoggedMdtNrIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null30;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMdtTriggerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1ConfigurationIeExtensions(Vec<M1ConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1PeriodicReportingIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1PeriodicReportingIeExtensions(Vec<M1PeriodicReportingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdEventA2IeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1ThresholdEventA2IeExtensions(Vec<M1ThresholdEventA2IeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4ConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M4ConfigurationIeExtensions(Vec<M4ConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M5ConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M5ConfigurationIeExtensions(Vec<M5ConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M6ConfigurationIeExtensions(Vec<M6ConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M7ConfigurationIeExtensions(Vec<M7ConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtConfigurationIeExtensions(Vec<MdtConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationEutraIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtConfigurationEutraIeExtensions(Vec<MdtConfigurationEutraIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationNrIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtConfigurationNrIeExtensions(Vec<MdtConfigurationNrIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtLocationInfoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtLocationInfoIeExtensions(Vec<MdtLocationInfoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtModeNrchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementThresholdL1LoggedMdTchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MobilityRestrictionListIeExtensionsItemExtensionValue {
    #[asn(key = 160)]
    IdCnTypeRestrictionsForEquivalent(CnTypeRestrictionsForEquivalent),
    #[asn(key = 161)]
    IdCnTypeRestrictionsForServing(CnTypeRestrictionsForServing),
    #[asn(key = 150)]
    IdLastEutranPlmnIdentity(PlmnIdentity),
    #[asn(key = 261)]
    IdNpnMobilityInformation(NpnMobilityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MobilityRestrictionListIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: MobilityRestrictionListIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MobilityRestrictionListIeExtensions(Vec<MobilityRestrictionListIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BitString31(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct N3iwfIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NasNonDeliveryIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NasNonDeliveryIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NasNonDeliveryIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NasNonDeliveryIndicationProtocolIEs(Vec<NasNonDeliveryIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NbIoTPagingEDrxInfoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NbIoTPagingEDrxInfoIeExtensions(Vec<NbIoTPagingEDrxInfoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgranCgIchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgranTnlAssociationToRemoveItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NgranTnlAssociationToRemoveItemIeExtensions(Vec<NgranTnlAssociationToRemoveItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgResetProtocolIEsItemValue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 88)]
    IdResetType(ResetType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgResetProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgResetProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgResetProtocolIEs(Vec<NgResetProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgResetAcknowledgeProtocolIEsItemValue {
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    IdUeAssociatedLogicalNgConnectionList(UeAssociatedLogicalNgConnectionList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgResetAcknowledgeProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgResetAcknowledgeProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgResetAcknowledgeProtocolIEs(Vec<NgResetAcknowledgeProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgSetupFailureProtocolIEsItemValue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    IdTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgSetupFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgSetupFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgSetupFailureProtocolIEs(Vec<NgSetupFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgSetupRequestProtocolIEsItemValue {
    #[asn(key = 21)]
    IdDefaultPagingDrx(PagingDrx),
    #[asn(key = 273)]
    IdExtendedRanNodeName(ExtendedRanNodeName),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 204)]
    IdNbIoTDefaultPagingDrx(NbIoTDefaultPagingDrx),
    #[asn(key = 82)]
    IdRanNodeName(RanNodeName),
    #[asn(key = 102)]
    IdSupportedTaList(SupportedTaList),
    #[asn(key = 147)]
    IdUeRetentionInformation(UeRetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgSetupRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgSetupRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgSetupRequestProtocolIEs(Vec<NgSetupRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgSetupResponseProtocolIEsItemValue {
    #[asn(key = 1)]
    IdAmfName(AmfName),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 274)]
    IdExtendedAmfName(ExtendedAmfName),
    #[asn(key = 200)]
    IdIabSupported(IabSupported),
    #[asn(key = 80)]
    IdPlmnSupportList(PlmnSupportList),
    #[asn(key = 86)]
    IdRelativeAmfCapacity(RelativeAmfCapacity),
    #[asn(key = 96)]
    IdServedGuamiList(ServedGuamiList),
    #[asn(key = 147)]
    IdUeRetentionInformation(UeRetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgSetupResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgSetupResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgSetupResponseProtocolIEs(Vec<NgSetupResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnAccessInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnMobilityInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnPagingAssistanceInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnSupportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrCgiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrCgiIeExtensions(Vec<NrCgiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrFrequencyBandItemIeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrFrequencyBandItemIeExtension(Vec<NrFrequencyBandItemIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrFrequencyInfoIeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrFrequencyInfoIeExtension(Vec<NrFrequencyInfoIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrueSidelinkAggregateMaximumBitrateIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrueSidelinkAggregateMaximumBitrateIeExtensions(Vec<NrueSidelinkAggregateMaximumBitrateIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Nrv2xServicesAuthorizedIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Nrv2xServicesAuthorizedIeExtensions(Vec<Nrv2xServicesAuthorizedIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BitString32(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BitString33(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BitString34(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgEnbIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NonDynamic5QiDescriptorIeExtensionsItemExtensionValue {
    #[asn(key = 187)]
    IdCnPacketDelayBudgetDl(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    IdCnPacketDelayBudgetUl(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamic5QiDescriptorIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: NonDynamic5QiDescriptorIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NonDynamic5QiDescriptorIeExtensions(Vec<NonDynamic5QiDescriptorIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadResponsechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartProtocolIEsItemValue {
    #[asn(key = 2)]
    IdAmfOverloadResponse(OverloadResponse),
    #[asn(key = 9)]
    IdAmfTrafficLoadReductionIndication(TrafficLoadReductionIndication),
    #[asn(key = 49)]
    IdOverloadStartNssaiList(OverloadStartNssaiList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: OverloadStartProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct OverloadStartProtocolIEs(Vec<OverloadStartProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartNssaiItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct OverloadStartNssaiItemIeExtensions(Vec<OverloadStartNssaiItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopProtocolIEsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct OverloadStopProtocolIEs(Vec<OverloadStopProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5FlowBitRatesIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Pc5FlowBitRatesIeExtensions(Vec<Pc5FlowBitRatesIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5QoSFlowItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Pc5QoSFlowItemIeExtensions(Vec<Pc5QoSFlowItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5QoSParametersIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Pc5QoSParametersIeExtensions(Vec<Pc5QoSParametersIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionAggregateMaximumBitRateIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionAggregateMaximumBitRateIeExtensions(Vec<PduSessionAggregateMaximumBitRateIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString35(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceAdmittedItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceAdmittedItemIeExtensions(Vec<PduSessionResourceAdmittedItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString36(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToModifyItemModCfmIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToModifyItemModCfmIeExtensions(Vec<PduSessionResourceFailedToModifyItemModCfmIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString37(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToModifyItemModResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToModifyItemModResIeExtensions(Vec<PduSessionResourceFailedToModifyItemModResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToResumeItemResReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToResumeItemResReqIeExtensions(Vec<PduSessionResourceFailedToResumeItemResReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToResumeItemResResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToResumeItemResResIeExtensions(Vec<PduSessionResourceFailedToResumeItemResResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString38(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemCxtFailIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemCxtFailIeExtensions(Vec<PduSessionResourceFailedToSetupItemCxtFailIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString39(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemCxtResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemCxtResIeExtensions(Vec<PduSessionResourceFailedToSetupItemCxtResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString40(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemHoAckIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemHoAckIeExtensions(Vec<PduSessionResourceFailedToSetupItemHoAckIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString41(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemPsReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemPsReqIeExtensions(Vec<PduSessionResourceFailedToSetupItemPsReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString42(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemSuResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemSuResIeExtensions(Vec<PduSessionResourceFailedToSetupItemSuResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString43(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceHandoverItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceHandoverItemIeExtensions(Vec<PduSessionResourceHandoverItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceInformationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceInformationItemIeExtensions(Vec<PduSessionResourceInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceItemCxtRelCplIeExtensionsItemExtensionValue { }

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceItemCxtRelCplIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceItemCxtRelCplIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceItemCxtRelCplIeExtensions(Vec<PduSessionResourceItemCxtRelCplIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceItemCxtRelReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceItemCxtRelReqIeExtensions(Vec<PduSessionResourceItemCxtRelReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString44(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceItemHoRqdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceItemHoRqdIeExtensions(Vec<PduSessionResourceItemHoRqdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyConfirmProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 131)]
    IdPduSessionResourceFailedToModifyListModCfm(PduSessionResourceFailedToModifyListModCfm),
    #[asn(key = 62)]
    IdPduSessionResourceModifyListModCfm(PduSessionResourceModifyListModCfm),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyConfirmProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyConfirmProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyConfirmProtocolIEs(Vec<PduSessionResourceModifyConfirmProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyConfirmTransferIeExtensionsItemExtensionValue {
    #[asn(key = 185)]
    IdAdditionalRedundantNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyConfirmTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyConfirmTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyConfirmTransferIeExtensions(Vec<PduSessionResourceModifyConfirmTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 63)]
    IdPduSessionResourceModifyListModInd(PduSessionResourceModifyListModInd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyIndicationProtocolIEs(Vec<PduSessionResourceModifyIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyIndicationTransferIeExtensionsItemExtensionValue {
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 193)]
    IdRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformation),
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
    #[asn(key = 156)]
    IdSecurityResult(SecurityResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyIndicationTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyIndicationTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyIndicationTransferIeExtensions(Vec<PduSessionResourceModifyIndicationTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyIndicationUnsuccessfulTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyIndicationUnsuccessfulTransferIeExtensions(Vec<PduSessionResourceModifyIndicationUnsuccessfulTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString45(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModCfmIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModCfmIeExtensions(Vec<PduSessionResourceModifyItemModCfmIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString46(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModIndIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModIndIeExtensions(Vec<PduSessionResourceModifyItemModIndIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyItemModReqIeExtensionsItemExtensionValue {
    #[asn(key = 148)]
    IdSNssai(SNssai),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModReqIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyItemModReqIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModReqIeExtensions(Vec<PduSessionResourceModifyItemModReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString48(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModResIeExtensions(Vec<PduSessionResourceModifyItemModResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 64)]
    IdPduSessionResourceModifyListModReq(PduSessionResourceModifyListModReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyRequestProtocolIEs(Vec<PduSessionResourceModifyRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyRequestTransferProtocolIEsItemValue {
    #[asn(key = 186)]
    IdAdditionalRedundantUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 126)]
    IdAdditionalUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 166)]
    IdCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 129)]
    IdNetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    IdPduSessionAggregateMaximumBitRate(PduSessionAggregateMaximumBitRate),
    #[asn(key = 135)]
    IdQosFlowAddOrModifyRequestList(QosFlowAddOrModifyRequestList),
    #[asn(key = 137)]
    IdQosFlowToReleaseList(QosFlowListWithCause),
    #[asn(key = 190)]
    IdRedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 138)]
    IdSecurityIndication(SecurityIndication),
    #[asn(key = 140)]
    IdUlNguUpTnlModifyList(UlNguUpTnlModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyRequestTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyRequestTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyRequestTransferProtocolIEs(Vec<PduSessionResourceModifyRequestTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 54)]
    IdPduSessionResourceFailedToModifyListModRes(PduSessionResourceFailedToModifyListModRes),
    #[asn(key = 65)]
    IdPduSessionResourceModifyListModRes(PduSessionResourceModifyListModRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyResponseProtocolIEs(Vec<PduSessionResourceModifyResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyResponseTransferIeExtensionsItemExtensionValue {
    #[asn(key = 154)]
    IdAdditionalNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 185)]
    IdAdditionalRedundantNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyResponseTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyResponseTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyResponseTransferIeExtensions(Vec<PduSessionResourceModifyResponseTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyUnsuccessfulTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyUnsuccessfulTransferIeExtensions(Vec<PduSessionResourceModifyUnsuccessfulTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceNotifyProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 66)]
    IdPduSessionResourceNotifyList(PduSessionResourceNotifyList),
    #[asn(key = 67)]
    IdPduSessionResourceReleasedListNot(PduSessionResourceReleasedListNot),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceNotifyProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceNotifyProtocolIEs(Vec<PduSessionResourceNotifyProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString49(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceNotifyItemIeExtensions(Vec<PduSessionResourceNotifyItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceNotifyReleasedTransferIeExtensionsItemExtensionValue {
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyReleasedTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceNotifyReleasedTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceNotifyReleasedTransferIeExtensions(Vec<PduSessionResourceNotifyReleasedTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceNotifyTransferIeExtensionsItemExtensionValue {
    #[asn(key = 278)]
    IdQosFlowFeedbackList(QosFlowFeedbackList),
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceNotifyTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceNotifyTransferIeExtensions(Vec<PduSessionResourceNotifyTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceReleaseCommandProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 79)]
    IdPduSessionResourceToReleaseListRelCmd(PduSessionResourceToReleaseListRelCmd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseCommandProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceReleaseCommandProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceReleaseCommandProtocolIEs(Vec<PduSessionResourceReleaseCommandProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseCommandTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleaseCommandTransferIeExtensions(Vec<PduSessionResourceReleaseCommandTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceReleaseResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 70)]
    IdPduSessionResourceReleasedListRelRes(PduSessionResourceReleasedListRelRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceReleaseResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceReleaseResponseProtocolIEs(Vec<PduSessionResourceReleaseResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceReleaseResponseTransferIeExtensionsItemExtensionValue {
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseResponseTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceReleaseResponseTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleaseResponseTransferIeExtensions(Vec<PduSessionResourceReleaseResponseTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString50(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemNotIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemNotIeExtensions(Vec<PduSessionResourceReleasedItemNotIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString51(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemPsAckIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemPsAckIeExtensions(Vec<PduSessionResourceReleasedItemPsAckIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString52(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemPsFailIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemPsFailIeExtensions(Vec<PduSessionResourceReleasedItemPsFailIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString53(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemRelResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemRelResIeExtensions(Vec<PduSessionResourceReleasedItemRelResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString54(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceResumeItemResReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceResumeItemResReqIeExtensions(Vec<PduSessionResourceResumeItemResReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString55(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceResumeItemResResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceResumeItemResResIeExtensions(Vec<PduSessionResourceResumeItemResResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString56(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSecondaryRatUsageItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSecondaryRatUsageItemIeExtensions(Vec<PduSessionResourceSecondaryRatUsageItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString57(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemCxtReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemCxtReqIeExtensions(Vec<PduSessionResourceSetupItemCxtReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString58(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemCxtResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemCxtResIeExtensions(Vec<PduSessionResourceSetupItemCxtResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString59(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemHoReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemHoReqIeExtensions(Vec<PduSessionResourceSetupItemHoReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString60(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemSuReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemSuReqIeExtensions(Vec<PduSessionResourceSetupItemSuReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString61(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemSuResIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemSuResIeExtensions(Vec<PduSessionResourceSetupItemSuResIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 74)]
    IdPduSessionResourceSetupListSuReq(PduSessionResourceSetupListSuReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceSetupRequestProtocolIEs(Vec<PduSessionResourceSetupRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupRequestTransferProtocolIEsItemValue {
    #[asn(key = 186)]
    IdAdditionalRedundantUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 126)]
    IdAdditionalUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 166)]
    IdCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 127)]
    IdDataForwardingNotPossible(DataForwardingNotPossible),
    #[asn(key = 22)]
    IdDirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 129)]
    IdNetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    IdPduSessionAggregateMaximumBitRate(PduSessionAggregateMaximumBitRate),
    #[asn(key = 134)]
    IdPduSessionType(PduSessionType),
    #[asn(key = 136)]
    IdQosFlowSetupRequestList(QosFlowSetupRequestList),
    #[asn(key = 190)]
    IdRedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 197)]
    IdRedundantPduSessionInformation(RedundantPduSessionInformation),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 138)]
    IdSecurityIndication(SecurityIndication),
    #[asn(key = 139)]
    IdUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupRequestTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupRequestTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceSetupRequestTransferProtocolIEs(Vec<PduSessionResourceSetupRequestTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 58)]
    IdPduSessionResourceFailedToSetupListSuRes(PduSessionResourceFailedToSetupListSuRes),
    #[asn(key = 75)]
    IdPduSessionResourceSetupListSuRes(PduSessionResourceSetupListSuRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceSetupResponseProtocolIEs(Vec<PduSessionResourceSetupResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupResponseTransferIeExtensionsItemExtensionValue {
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 193)]
    IdRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformation),
    #[asn(key = 198)]
    IdUsedRsnInformation(RedundantPduSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupResponseTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceSetupResponseTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupResponseTransferIeExtensions(Vec<PduSessionResourceSetupResponseTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupUnsuccessfulTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupUnsuccessfulTransferIeExtensions(Vec<PduSessionResourceSetupUnsuccessfulTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSuspendItemSusReqIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSuspendItemSusReqIeExtensions(Vec<PduSessionResourceSuspendItemSusReqIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString63(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSwitchedItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSwitchedItemIeExtensions(Vec<PduSessionResourceSwitchedItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString64(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceToBeSwitchedDlItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceToBeSwitchedDlItemIeExtensions(Vec<PduSessionResourceToBeSwitchedDlItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceToReleaseItemHoCmdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceToReleaseItemHoCmdIeExtensions(Vec<PduSessionResourceToReleaseItemHoCmdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString66(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceToReleaseItemRelCmdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceToReleaseItemRelCmdIeExtensions(Vec<PduSessionResourceToReleaseItemRelCmdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated67(u8);
impl Enumerated67 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionUsageReportIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionUsageReportIeExtensions(Vec<PduSessionUsageReportIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PlmnSupportItemIeExtensionsItemExtensionValue {
    #[asn(key = 270)]
    IdExtendedSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    IdNpnSupport(NpnSupport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PlmnSupportItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PlmnSupportItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PlmnSupportItemIeExtensions(Vec<PlmnSupportItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PniNpnMobilityInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PniNpnMobilityInformationIeExtensions(Vec<PniNpnMobilityInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsCancelRequestProtocolIEsItemValue {
    #[asn(key = 14)]
    IdCancelAllWarningMessages(CancelAllWarningMessages),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
    #[asn(key = 122)]
    IdWarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsCancelRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsCancelRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsCancelRequestProtocolIEs(Vec<PwsCancelRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsCancelResponseProtocolIEsItemValue {
    #[asn(key = 12)]
    IdBroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsCancelResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsCancelResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsCancelResponseProtocolIEs(Vec<PwsCancelResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailedCellIdListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsFailureIndicationProtocolIEsItemValue {
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 81)]
    IdPwsFailedCellIdList(PwsFailedCellIdList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailureIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsFailureIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsFailureIndicationProtocolIEs(Vec<PwsFailureIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsRestartIndicationProtocolIEsItemValue {
    #[asn(key = 16)]
    IdCellIdListForRestart(CellIdListForRestart),
    #[asn(key = 23)]
    IdEmergencyAreaIdListForRestart(EmergencyAreaIdListForRestart),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 104)]
    IdTaiListForRestart(TaiListForRestart),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsRestartIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsRestartIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsRestartIndicationProtocolIEs(Vec<PwsRestartIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct Integer68(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct Integer69(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PacketErrorRateIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PacketErrorRateIeExtensions(Vec<PacketErrorRateIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingProtocolIEsItemValue {
    #[asn(key = 11)]
    IdAssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 203)]
    IdNbIoTPagingEDrxInfo(NbIoTPagingEDrxInfo),
    #[asn(key = 202)]
    IdNbIoTPagingDrx(NbIoTPagingDrx),
    #[asn(key = 50)]
    IdPagingDrx(PagingDrx),
    #[asn(key = 51)]
    IdPagingOrigin(PagingOrigin),
    #[asn(key = 52)]
    IdPagingPriority(PagingPriority),
    #[asn(key = 223)]
    IdPagingeDrxInformation(PagingeDrxInformation),
    #[asn(key = 103)]
    IdTaiListForPaging(TaiListForPaging),
    #[asn(key = 115)]
    IdUePagingIdentity(UePagingIdentity),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
    #[asn(key = 208)]
    IdWusAssistanceInformation(WusAssistanceInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PagingProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PagingProtocolIEs(Vec<PagingProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAssisDataforCEcapabUeIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingAssisDataforCEcapabUeIeExtensions(Vec<PagingAssisDataforCEcapabUeIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAttemptInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingAttemptInformationIeExtensions(Vec<PagingAttemptInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingeDrxInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingeDrxInformationIeExtensions(Vec<PagingeDrxInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestProtocolIEsItemValue {
    #[asn(key = 57)]
    IdPduSessionResourceFailedToSetupListPsReq(PduSessionResourceFailedToSetupListPsReq),
    #[asn(key = 76)]
    IdPduSessionResourceToBeSwitchedDlList(PduSessionResourceToBeSwitchedDlList),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 237)]
    IdRrcResumeCause(RrcEstablishmentCause),
    #[asn(key = 100)]
    IdSourceAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PathSwitchRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestProtocolIEs(Vec<PathSwitchRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 41)]
    IdNewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 68)]
    IdPduSessionResourceReleasedListPsAck(PduSessionResourceReleasedListPsAck),
    #[asn(key = 77)]
    IdPduSessionResourceSwitchedList(PduSessionResourceSwitchedList),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 146)]
    IdRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 234)]
    IdUeUpCIoTSupport(UeUpCIoTSupport),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PathSwitchRequestAcknowledgeProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestAcknowledgeProtocolIEs(Vec<PathSwitchRequestAcknowledgeProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeTransferIeExtensionsItemExtensionValue {
    #[asn(key = 154)]
    IdAdditionalNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 185)]
    IdAdditionalRedundantNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 277)]
    IdQosFlowParametersList(QosFlowParametersList),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestAcknowledgeTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestAcknowledgeTransferIeExtensions(Vec<PathSwitchRequestAcknowledgeTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 69)]
    IdPduSessionResourceReleasedListPsFail(PduSessionResourceReleasedListPsFail),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PathSwitchRequestFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestFailureProtocolIEs(Vec<PathSwitchRequestFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestSetupFailedTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestSetupFailedTransferIeExtensions(Vec<PathSwitchRequestSetupFailedTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestTransferIeExtensionsItemExtensionValue {
    #[asn(key = 155)]
    IdAdditionalDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 191)]
    IdRedundantDlNguTnlInformationReused(DlNguTnlInformationReused),
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 198)]
    IdUsedRsnInformation(RedundantPduSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestTransferIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestTransferIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestTransferIeExtensions(Vec<PathSwitchRequestTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestUnsuccessfulTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestUnsuccessfulTransferIeExtensions(Vec<PathSwitchRequestUnsuccessfulTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct Integer70(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct ObjectIdentifier71;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessagePrivateIEsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PrivateMessagePrivateIEs(Vec<PrivateMessagePrivateIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProcedureStageChoicechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated72(u8);
impl Enumerated72 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSFlowsUsageReportItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QoSFlowsUsageReportItemIeExtensions(Vec<QoSFlowsUsageReportItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosCharacteristicschoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAcceptedItemIeExtensionsItemExtensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAcceptedItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowAcceptedItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAcceptedItemIeExtensions(Vec<QosFlowAcceptedItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyRequestItemIeExtensionsItemExtensionValue {
    #[asn(key = 194)]
    IdRedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    IdTscTrafficCharacteristics(TscTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyRequestItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyRequestItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAddOrModifyRequestItemIeExtensions(Vec<QosFlowAddOrModifyRequestItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyResponseItemIeExtensionsItemExtensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyResponseItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyResponseItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAddOrModifyResponseItemIeExtensions(Vec<QosFlowAddOrModifyResponseItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowFeedbackItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowFeedbackItemIeExtensions(Vec<QosFlowFeedbackItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowInformationItemIeExtensionsItemExtensionValue {
    #[asn(key = 163)]
    IdUlForwarding(UlForwarding),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowInformationItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowInformationItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowInformationItemIeExtensions(Vec<QosFlowInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowItemWithDataForwardingIeExtensionsItemExtensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowItemWithDataForwardingIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowItemWithDataForwardingIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowItemWithDataForwardingIeExtensions(Vec<QosFlowItemWithDataForwardingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowLevelQosParametersIeExtensionsItemExtensionValue {
    #[asn(key = 276)]
    IdQosMonitoringReportingFrequency(QosMonitoringReportingFrequency),
    #[asn(key = 181)]
    IdQosMonitoringRequest(QosMonitoringRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowLevelQosParametersIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowLevelQosParametersIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowLevelQosParametersIeExtensions(Vec<QosFlowLevelQosParametersIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowModifyConfirmItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowModifyConfirmItemIeExtensions(Vec<QosFlowModifyConfirmItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowNotifyItemIeExtensionsItemExtensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetNotifyIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowNotifyItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowNotifyItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowNotifyItemIeExtensions(Vec<QosFlowNotifyItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowParametersItemIeExtensionsItemExtensionValue {
    #[asn(key = 279)]
    IdBurstArrivalTimeDownlink(BurstArrivalTime),
    #[asn(key = 187)]
    IdCnPacketDelayBudgetDl(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    IdCnPacketDelayBudgetUl(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowParametersItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowParametersItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowParametersItemIeExtensions(Vec<QosFlowParametersItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTnlInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowPerTnlInformationIeExtensions(Vec<QosFlowPerTnlInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTnlInformationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowPerTnlInformationItemIeExtensions(Vec<QosFlowPerTnlInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowSetupRequestItemIeExtensionsItemExtensionValue {
    #[asn(key = 194)]
    IdRedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    IdTscTrafficCharacteristics(TscTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowSetupRequestItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowSetupRequestItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowSetupRequestItemIeExtensions(Vec<QosFlowSetupRequestItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowToBeForwardedItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowToBeForwardedItemIeExtensions(Vec<QosFlowToBeForwardedItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowWithCauseItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowWithCauseItemIeExtensions(Vec<QosFlowWithCauseItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RancpRelocationIndicationProtocolIEsItemValue {
    #[asn(key = 25)]
    IdEutraCgi(EutraCgi),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 213)]
    IdTai(Tai),
    #[asn(key = 211)]
    IdUlCpSecurityInformation(UlCpSecurityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RancpRelocationIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RancpRelocationIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RancpRelocationIndicationProtocolIEs(Vec<RancpRelocationIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RanConfigurationUpdateProtocolIEsItemValue {
    #[asn(key = 21)]
    IdDefaultPagingDrx(PagingDrx),
    #[asn(key = 273)]
    IdExtendedRanNodeName(ExtendedRanNodeName),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 204)]
    IdNbIoTDefaultPagingDrx(NbIoTDefaultPagingDrx),
    #[asn(key = 167)]
    IdNgranTnlAssociationToRemoveList(NgranTnlAssociationToRemoveList),
    #[asn(key = 82)]
    IdRanNodeName(RanNodeName),
    #[asn(key = 102)]
    IdSupportedTaList(SupportedTaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanConfigurationUpdateProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RanConfigurationUpdateProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RanConfigurationUpdateProtocolIEs(Vec<RanConfigurationUpdateProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RanConfigurationUpdateAcknowledgeProtocolIEsItemValue {
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanConfigurationUpdateAcknowledgeProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RanConfigurationUpdateAcknowledgeProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RanConfigurationUpdateAcknowledgeProtocolIEs(Vec<RanConfigurationUpdateAcknowledgeProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RanConfigurationUpdateFailureProtocolIEsItemValue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    IdTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanConfigurationUpdateFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RanConfigurationUpdateFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RanConfigurationUpdateFailureProtocolIEs(Vec<RanConfigurationUpdateFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanStatusTransferTransparentContainerIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RanStatusTransferTransparentContainerIeExtensions(Vec<RanStatusTransferTransparentContainerIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RatRestrictionsItemIeExtensionsItemExtensionValue {
    #[asn(key = 180)]
    IdExtendedRatRestrictionInformation(ExtendedRatRestrictionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RatRestrictionsItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: RatRestrictionsItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RatRestrictionsItemIeExtensions(Vec<RatRestrictionsItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated73(u8);
impl Enumerated73 {
    const RS_DETECTED: u8 = 0u8;
    const RS_DISAPPEARED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RimInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RimInformationIeExtensions(Vec<RimInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RimInformationTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RimInformationTransferIeExtensions(Vec<RimInformationTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RrcInactiveTransitionReportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 92)]
    IdRrcState(RrcState),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RrcInactiveTransitionReportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RrcInactiveTransitionReportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RrcInactiveTransitionReportProtocolIEs(Vec<RrcInactiveTransitionReportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer74(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedCellItemIeExtensions(Vec<RecommendedCellItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellsForPagingIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedCellsForPagingIeExtensions(Vec<RecommendedCellsForPagingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRanNodeItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedRanNodeItemIeExtensions(Vec<RecommendedRanNodeItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRanNodesForPagingIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedRanNodesForPagingIeExtensions(Vec<RecommendedRanNodesForPagingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedundantPduSessionInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RedundantPduSessionInformationIeExtensions(Vec<RedundantPduSessionInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNasRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 3)]
    IdAmfSetId(AmfSetId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 171)]
    IdSourceToTargetAmfInformationReroute(SourceToTargetAmfInformationReroute),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNasRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RerouteNasRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RerouteNasRequestProtocolIEs(Vec<RerouteNasRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RetrieveUeInformationProtocolIEsItemValue {
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RetrieveUeInformationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RetrieveUeInformationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RetrieveUeInformationProtocolIEs(Vec<RetrieveUeInformationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNssaiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SNssaiIeExtensions(Vec<SNssaiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SnpnMobilityInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SnpnMobilityInformationIeExtensions(Vec<SnpnMobilityInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonConfigurationTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SonConfigurationTransferIeExtensions(Vec<SonConfigurationTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SonInformationchoiceExtensionsValue {
    #[asn(key = 252)]
    IdSonInformationReport(SonInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SonInformationchoiceExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonInformationReplyIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SonInformationReplyIeExtensions(Vec<SonInformationReplyIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonInformationReportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BitString75(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct Integer76(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct Integer77(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScheduledCommunicationTimeIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ScheduledCommunicationTimeIeExtensions(Vec<ScheduledCommunicationTimeIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRatDataUsageReportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 143)]
    IdHandoverFlag(HandoverFlag),
    #[asn(key = 142)]
    IdPduSessionResourceSecondaryRatUsageList(PduSessionResourceSecondaryRatUsageList),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRatDataUsageReportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SecondaryRatDataUsageReportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct SecondaryRatDataUsageReportProtocolIEs(Vec<SecondaryRatDataUsageReportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRatDataUsageReportTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecondaryRatDataUsageReportTransferIeExtensions(Vec<SecondaryRatDataUsageReportTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRatUsageInformationIeExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecondaryRatUsageInformationIeExtension(Vec<SecondaryRatUsageInformationIeExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityContextIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityContextIeExtensions(Vec<SecurityContextIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityIndicationIeExtensionsItemExtensionValue {
    #[asn(key = 151)]
    IdMaximumIntegrityProtectedDataRateDl(MaximumIntegrityProtectedDataRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityIndicationIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SecurityIndicationIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityIndicationIeExtensions(Vec<SecurityIndicationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityResultIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityResultIeExtensions(Vec<SecurityResultIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasConfigNameItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SensorMeasConfigNameItemIeExtensions(Vec<SensorMeasConfigNameItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasurementConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SensorMeasurementConfigurationIeExtensions(Vec<SensorMeasurementConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated78(u8);
impl Enumerated78 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated79(u8);
impl Enumerated79 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated80(u8);
impl Enumerated80 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorNameConfigchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedGuamiItemIeExtensionsItemExtensionValue {
    #[asn(key = 176)]
    IdGuamiType(GuamiType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGuamiItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ServedGuamiItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ServedGuamiItemIeExtensions(Vec<ServedGuamiItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServiceAreaInformationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ServiceAreaInformationItemIeExtensions(Vec<ServiceAreaInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceOverloadItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SliceOverloadItemIeExtensions(Vec<SliceOverloadItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceSupportItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SliceSupportItemIeExtensions(Vec<SliceSupportItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceNgranNodeToTargetNgranNodeTransparentContainerIeExtensionsItemExtensionValue {
    #[asn(key = 182)]
    IdSgNbUeX2apId(SgNbUeX2apId),
    #[asn(key = 253)]
    IdUeHistoryInformationFromTheUe(UeHistoryInformationFromTheUe),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNgranNodeToTargetNgranNodeTransparentContainerIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value:
        SourceNgranNodeToTargetNgranNodeTransparentContainerIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceNgranNodeToTargetNgranNodeTransparentContainerIeExtensions(Vec<SourceNgranNodeToTargetNgranNodeTransparentContainerIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRanNodeIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceRanNodeIdIeExtensions(Vec<SourceRanNodeIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceToTargetAmfInformationRerouteIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceToTargetAmfInformationRerouteIeExtensions(Vec<SourceToTargetAmfInformationRerouteIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 0)]
    IdAmfConfigurationUpdate(AmfConfigurationUpdateAcknowledge),
    #[asn(key = 10)]
    IdHandoverCancel(HandoverCancelAcknowledge),
    #[asn(key = 12)]
    IdHandoverPreparation(HandoverCommand),
    #[asn(key = 13)]
    IdHandoverResourceAllocation(HandoverRequestAcknowledge),
    #[asn(key = 14)]
    IdInitialContextSetup(InitialContextSetupResponse),
    #[asn(key = 20)]
    IdNgReset(NgResetAcknowledge),
    #[asn(key = 21)]
    IdNgSetup(NgSetupResponse),
    #[asn(key = 26)]
    IdPduSessionResourceModify(PduSessionResourceModifyResponse),
    #[asn(key = 27)]
    IdPduSessionResourceModifyIndication(PduSessionResourceModifyConfirm),
    #[asn(key = 28)]
    IdPduSessionResourceRelease(PduSessionResourceReleaseResponse),
    #[asn(key = 29)]
    IdPduSessionResourceSetup(PduSessionResourceSetupResponse),
    #[asn(key = 32)]
    IdPwsCancel(PwsCancelResponse),
    #[asn(key = 25)]
    IdPathSwitchRequest(PathSwitchRequestAcknowledge),
    #[asn(key = 35)]
    IdRanConfigurationUpdate(RanConfigurationUpdateAcknowledge),
    #[asn(key = 40)]
    IdUeContextModification(UeContextModificationResponse),
    #[asn(key = 41)]
    IdUeContextRelease(UeContextReleaseComplete),
    #[asn(key = 58)]
    IdUeContextResume(UeContextResumeResponse),
    #[asn(key = 59)]
    IdUeContextSuspend(UeContextSuspendResponse),
    #[asn(key = 43)]
    IdUeRadioCapabilityCheck(UeRadioCapabilityCheckResponse),
    #[asn(key = 60)]
    IdUeRadioCapabilityIdMapping(UeRadioCapabilityIdMappingResponse),
    #[asn(key = 51)]
    IdWriteReplaceWarning(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTaItemIeExtensionsItemExtensionValue {
    #[asn(key = 272)]
    IdConfiguredTacIndication(ConfiguredTacIndication),
    #[asn(key = 179)]
    IdRatInformation(RatInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTaItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SupportedTaItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SupportedTaItemIeExtensions(Vec<SupportedTaItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaBasedMdtIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaBasedMdtIeExtensions(Vec<TaBasedMdtIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiIeExtensions(Vec<TaiIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiBasedMdtIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBasedMdtIeExtensions(Vec<TaiBasedMdtIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiBroadcastEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastEutraItemIeExtensions(Vec<TaiBroadcastEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiBroadcastNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastNrItemIeExtensions(Vec<TaiBroadcastNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiCancelledEutraItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledEutraItemIeExtensions(Vec<TaiCancelledEutraItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiCancelledNrItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledNrItemIeExtensions(Vec<TaiCancelledNrItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiListForInactiveItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiListForInactiveItemIeExtensions(Vec<TaiListForInactiveItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiListForPagingItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiListForPagingItemIeExtensions(Vec<TaiListForPagingItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BitString81(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TngfIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TnlAssociationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TnlAssociationItemIeExtensions(Vec<TnlAssociationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TscAssistanceInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TscAssistanceInformationIeExtensions(Vec<TscAssistanceInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TscTrafficCharacteristicsIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TscTrafficCharacteristicsIeExtensions(Vec<TscTrafficCharacteristicsIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BitString82(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TwifIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetIDchoiceExtensionsValue {
    #[asn(key = 178)]
    IdTargetRncId(TargetRncId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetIDchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TargetIDchoiceExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNgranNodeToSourceNgranNodeFailureTransparentContainerIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetNgranNodeToSourceNgranNodeFailureTransparentContainerIeExtensions(Vec<TargetNgranNodeToSourceNgranNodeFailureTransparentContainerIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetNgranNodeToSourceNgranNodeTransparentContainerIeExtensionsItemExtensionValue {
    #[asn(key = 267)]
    IdDapsResponseInfoList(DapsResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNgranNodeToSourceNgranNodeTransparentContainerIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value:
        TargetNgranNodeToSourceNgranNodeTransparentContainerIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetNgranNodeToSourceNgranNodeTransparentContainerIeExtensions(Vec<TargetNgranNodeToSourceNgranNodeTransparentContainerIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRanNodeIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetRanNodeIdIeExtensions(Vec<TargetRanNodeIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRncIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetRncIdIeExtensions(Vec<TargetRncIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNbIdIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargeteNbIdIeExtensions(Vec<TargeteNbIdIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TooearlyIntersystemHoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TooearlyIntersystemHoIeExtensions(Vec<TooearlyIntersystemHoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationIeExtensionsItemExtensionValue {
    #[asn(key = 255)]
    IdMdtConfiguration(MdtConfiguration),
    #[asn(key = 257)]
    IdTraceCollectionEntityUri(UriAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceActivationIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: TraceActivationIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TraceActivationIeExtensions(Vec<TraceActivationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceFailureIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 44)]
    IdNgranTraceId(NgranTraceId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceFailureIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TraceFailureIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct TraceFailureIndicationProtocolIEs(Vec<TraceFailureIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceStartProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 108)]
    IdTraceActivation(TraceActivation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceStartProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TraceStartProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct TraceStartProtocolIEs(Vec<TraceStartProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated83(u8);
impl Enumerated83 {
    const PERIODICALLY: u8 = 0u8;
    const ONDEMAND: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3600", extensible = true)]
pub struct Integer84(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated85(u8);
impl Enumerated85 {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated86(u8);
impl Enumerated86 {
    const SINGLE_PACKET: u8 = 0u8;
    const DUAL_PACKETS: u8 = 1u8;
    const MULTIPLE_PACKETS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated87(u8);
impl Enumerated87 {
    const BATTERY_POWERED: u8 = 0u8;
    const BATTERY_POWERED_NOT_RECHARGEABLE_OR_REPLACEABLE: u8 = 1u8;
    const NOT_BATTERY_POWERED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeDifferentiationInfoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeDifferentiationInfoIeExtensions(Vec<UeDifferentiationInfoIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeNgapIdPairIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeNgapIdPairIeExtensions(Vec<UeNgapIdPairIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeNgapIDschoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAssociatedLogicalNgConnectionItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeAssociatedLogicalNgConnectionItemIeExtensions(Vec<UeAssociatedLogicalNgConnectionItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAggregateMaximumBitRateIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeAggregateMaximumBitRateIeExtensions(Vec<UeAggregateMaximumBitRateIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextModificationFailureProtocolIEs(Vec<UeContextModificationFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    IdEmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 199)]
    IdIabAuthorized(IabAuthorized),
    #[asn(key = 31)]
    IdIndexToRfsp(IndexToRfsp),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 40)]
    IdNewAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 162)]
    IdNewGuami(Guami),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
    #[asn(key = 238)]
    IdRgLevelWirelineAccessCharacteristics(RgLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 94)]
    IdSecurityKey(SecurityKey),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextModificationRequestProtocolIEs(Vec<UeContextModificationRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 92)]
    IdRrcState(RrcState),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextModificationResponseProtocolIEs(Vec<UeContextModificationResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseCommandProtocolIEsItemValue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 114)]
    IdUeNgapIDs(UeNgapIDs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseCommandProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseCommandProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextReleaseCommandProtocolIEs(Vec<UeContextReleaseCommandProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseCompleteProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    IdInfoOnRecommendedCellsAndRanNodesForPaging(InfoOnRecommendedCellsAndRanNodesForPaging),
    #[asn(key = 60)]
    IdPduSessionResourceListCxtRelCpl(PduSessionResourceListCxtRelCpl),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseCompleteProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseCompleteProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextReleaseCompleteProtocolIEs(Vec<UeContextReleaseCompleteProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 133)]
    IdPduSessionResourceListCxtRelReq(PduSessionResourceListCxtRelReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextReleaseRequestProtocolIEs(Vec<UeContextReleaseRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextResumeFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextResumeFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextResumeFailureProtocolIEs(Vec<UeContextResumeFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextResumeRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 32)]
    IdInfoOnRecommendedCellsAndRanNodesForPaging(InfoOnRecommendedCellsAndRanNodesForPaging),
    #[asn(key = 229)]
    IdPduSessionResourceFailedToResumeListResReq(PduSessionResourceFailedToResumeListResReq),
    #[asn(key = 232)]
    IdPduSessionResourceResumeListResReq(PduSessionResourceResumeListResReq),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 237)]
    IdRrcResumeCause(RrcEstablishmentCause),
    #[asn(key = 235)]
    IdSuspendRequestIndication(SuspendRequestIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextResumeRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextResumeRequestProtocolIEs(Vec<UeContextResumeRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeRequestTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeContextResumeRequestTransferIeExtensions(Vec<UeContextResumeRequestTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextResumeResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 230)]
    IdPduSessionResourceFailedToResumeListResRes(PduSessionResourceFailedToResumeListResRes),
    #[asn(key = 233)]
    IdPduSessionResourceResumeListResRes(PduSessionResourceResumeListResRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
    #[asn(key = 236)]
    IdSuspendResponseIndication(SuspendResponseIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextResumeResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextResumeResponseProtocolIEs(Vec<UeContextResumeResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeResponseTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeContextResumeResponseTransferIeExtensions(Vec<UeContextResumeResponseTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSuspendFailureProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendFailureProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSuspendFailureProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextSuspendFailureProtocolIEs(Vec<UeContextSuspendFailureProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSuspendRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 32)]
    IdInfoOnRecommendedCellsAndRanNodesForPaging(InfoOnRecommendedCellsAndRanNodesForPaging),
    #[asn(key = 231)]
    IdPduSessionResourceSuspendListSusReq(PduSessionResourceSuspendListSusReq),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSuspendRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextSuspendRequestProtocolIEs(Vec<UeContextSuspendRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendRequestTransferIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeContextSuspendRequestTransferIeExtensions(Vec<UeContextSuspendRequestTransferIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSuspendResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSuspendResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextSuspendResponseProtocolIEs(Vec<UeContextSuspendResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeHistoryInformationFromTheUEchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BitString88(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeIdentityIndexValuechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeInformationTransferProtocolIEsItemValue {
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 210)]
    IdNbIoTUePriority(NbIoTUePriority),
    #[asn(key = 148)]
    IdSNssai(SNssai),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeInformationTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeInformationTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeInformationTransferProtocolIEs(Vec<UeInformationTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UePagingIdentitychoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UePresenceInAreaOfInterestItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UePresenceInAreaOfInterestItemIeExtensions(Vec<UePresenceInAreaOfInterestItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UerlfReportContainerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityCheckRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityCheckRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityCheckRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityCheckRequestProtocolIEs(Vec<UeRadioCapabilityCheckRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityCheckResponseProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 30)]
    IdImsVoiceSupportIndicator(ImsVoiceSupportIndicator),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityCheckResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityCheckResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityCheckResponseProtocolIEs(Vec<UeRadioCapabilityCheckResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityForPagingIeExtensionsItemExtensionValue {
    #[asn(key = 214)]
    IdUeRadioCapabilityForPagingOfNbIoT(UeRadioCapabilityForPagingOfNbIoT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityForPagingIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UeRadioCapabilityForPagingIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeRadioCapabilityForPagingIeExtensions(Vec<UeRadioCapabilityForPagingIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityIdMappingRequestProtocolIEsItemValue {
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityIdMappingRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityIdMappingRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityIdMappingRequestProtocolIEs(Vec<UeRadioCapabilityIdMappingRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityIdMappingResponseProtocolIEsItemValue {
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityIdMappingResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityIdMappingResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityIdMappingResponseProtocolIEs(Vec<UeRadioCapabilityIdMappingResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityInfoIndicationProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 265)]
    IdUeRadioCapabilityEutraFormat(UeRadioCapability),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityInfoIndicationProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityInfoIndicationProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityInfoIndicationProtocolIEs(Vec<UeRadioCapabilityInfoIndicationProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeSecurityCapabilitiesIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeSecurityCapabilitiesIeExtensions(Vec<UeSecurityCapabilitiesIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UetnlaBindingReleaseRequestProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UetnlaBindingReleaseRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UetnlaBindingReleaseRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UetnlaBindingReleaseRequestProtocolIEs(Vec<UetnlaBindingReleaseRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlCpSecurityInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UlCpSecurityInformationIeExtensions(Vec<UlCpSecurityInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UlNguUpTnlModifyItemIeExtensionsItemExtensionValue {
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlNguUpTnlModifyItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UlNguUpTnlModifyItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UlNguUpTnlModifyItemIeExtensions(Vec<UlNguUpTnlModifyItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UpTransportLayerInformationItemIeExtensions(Vec<UpTransportLayerInformationItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationPairItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UpTransportLayerInformationPairItemIeExtensions(Vec<UpTransportLayerInformationPairItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnavailableGuamiItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UnavailableGuamiItemIeExtensions(Vec<UnavailableGuamiItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 0)]
    IdAmfConfigurationUpdate(AmfConfigurationUpdateFailure),
    #[asn(key = 12)]
    IdHandoverPreparation(HandoverPreparationFailure),
    #[asn(key = 13)]
    IdHandoverResourceAllocation(HandoverFailure),
    #[asn(key = 14)]
    IdInitialContextSetup(InitialContextSetupFailure),
    #[asn(key = 21)]
    IdNgSetup(NgSetupFailure),
    #[asn(key = 25)]
    IdPathSwitchRequest(PathSwitchRequestFailure),
    #[asn(key = 35)]
    IdRanConfigurationUpdate(RanConfigurationUpdateFailure),
    #[asn(key = 40)]
    IdUeContextModification(UeContextModificationFailure),
    #[asn(key = 58)]
    IdUeContextResume(UeContextResumeFailure),
    #[asn(key = 59)]
    IdUeContextSuspend(UeContextSuspendFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNasTransportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNasTransportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkNasTransportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkNasTransportProtocolIEs(Vec<UplinkNasTransportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNonUeAssociatedNrpPaTransportProtocolIEsItemValue {
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUeAssociatedNrpPaTransportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkNonUeAssociatedNrpPaTransportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkNonUeAssociatedNrpPaTransportProtocolIEs(Vec<UplinkNonUeAssociatedNrpPaTransportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRanConfigurationTransferProtocolIEsItemValue {
    #[asn(key = 158)]
    IdEndcSonConfigurationTransferUl(EnDcsonConfigurationTransfer),
    #[asn(key = 251)]
    IdIntersystemSonConfigurationTransferUl(IntersystemSonConfigurationTransfer),
    #[asn(key = 99)]
    IdSonConfigurationTransferUl(SonConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRanConfigurationTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRanConfigurationTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRanConfigurationTransferProtocolIEs(Vec<UplinkRanConfigurationTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRanEarlyStatusTransferProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 268)]
    IdEarlyStatusTransferTransparentContainer(EarlyStatusTransferTransparentContainer),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRanEarlyStatusTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRanEarlyStatusTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRanEarlyStatusTransferProtocolIEs(Vec<UplinkRanEarlyStatusTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRanStatusTransferProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 84)]
    IdRanStatusTransferTransparentContainer(RanStatusTransferTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRanStatusTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRanStatusTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRanStatusTransferProtocolIEs(Vec<UplinkRanStatusTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRimInformationTransferProtocolIEsItemValue {
    #[asn(key = 175)]
    IdRimInformationTransfer(RimInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRimInformationTransferProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRimInformationTransferProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRimInformationTransferProtocolIEs(Vec<UplinkRimInformationTransferProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUeAssociatedNrpPaTransportProtocolIEsItemValue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUeAssociatedNrpPaTransportProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkUeAssociatedNrpPaTransportProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkUeAssociatedNrpPaTransportProtocolIEs(Vec<UplinkUeAssociatedNrpPaTransportProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationchoiceExtensionsValue {
    #[asn(key = 244)]
    IdUserLocationInformationTngf(UserLocationInformationTngf),
    #[asn(key = 248)]
    IdUserLocationInformationTwif(UserLocationInformationTwif),
    #[asn(key = 243)]
    IdUserLocationInformationWAgf(UserLocationInformationWAgf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UserLocationInformationchoiceExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationEutraIeExtensionsItemExtensionValue {
    #[asn(key = 149)]
    IdPsCellInformation(NgranCgi),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationEutraIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationEutraIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationEutraIeExtensions(Vec<UserLocationInformationEutraIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationN3iwfIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationN3iwfIeExtensions(Vec<UserLocationInformationN3iwfIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationNrIeExtensionsItemExtensionValue {
    #[asn(key = 263)]
    IdNid(Nid),
    #[asn(key = 149)]
    IdPsCellInformation(NgranCgi),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationNrIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationNrIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationNrIeExtensions(Vec<UserLocationInformationNrIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTngfIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationTngfIeExtensions(Vec<UserLocationInformationTngfIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTwifIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationTwifIeExtensions(Vec<UserLocationInformationTwifIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationWAgFchoiceExtensionsValue {
    #[asn(key = 275)]
    IdGlobalCableId(GlobalCableId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationWAgFchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UserLocationInformationWAgFchoiceExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneSecurityInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserPlaneSecurityInformationIeExtensions(Vec<UserPlaneSecurityInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OctetString89(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OctetString90(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct Integer91(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct Integer92(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VolumeTimedReportItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct VolumeTimedReportItemIeExtensions(Vec<VolumeTimedReportItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct BitString93(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WAgfIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WlanMeasConfigNameItemIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WlanMeasConfigNameItemIeExtensions(Vec<WlanMeasConfigNameItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated94(u8);
impl Enumerated94 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated95(u8);
impl Enumerated95 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WlanMeasurementConfigurationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WlanMeasurementConfigurationIeExtensions(Vec<WlanMeasurementConfigurationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WusAssistanceInformationIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WusAssistanceInformationIeExtensions(Vec<WusAssistanceInformationIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WarningAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestProtocolIEsItemValue {
    #[asn(key = 17)]
    IdConcurrentWarningMessageInd(ConcurrentWarningMessageInd),
    #[asn(key = 20)]
    IdDataCodingScheme(DataCodingScheme),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 47)]
    IdNumberOfBroadcastsRequested(NumberOfBroadcastsRequested),
    #[asn(key = 87)]
    IdRepetitionPeriod(RepetitionPeriod),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
    #[asn(key = 141)]
    IdWarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 122)]
    IdWarningAreaList(WarningAreaList),
    #[asn(key = 123)]
    IdWarningMessageContents(WarningMessageContents),
    #[asn(key = 124)]
    IdWarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 125)]
    IdWarningType(WarningType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningRequestProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningRequestProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct WriteReplaceWarningRequestProtocolIEs(Vec<WriteReplaceWarningRequestProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningResponseProtocolIEsItemValue {
    #[asn(key = 13)]
    IdBroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningResponseProtocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningResponseProtocolIEsItemValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct WriteReplaceWarningResponseProtocolIEs(Vec<WriteReplaceWarningResponseProtocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum XnExtTlaItemIeExtensionsItemExtensionValue {
    #[asn(key = 173)]
    IdSctpTlAs(SctpTlAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnExtTlaItemIeExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: XnExtTlaItemIeExtensionsItemExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct XnExtTlaItemIeExtensions(Vec<XnExtTlaItemIeExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnTnlConfigurationInfoIeExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct XnTnlConfigurationInfoIeExtensions(Vec<XnTnlConfigurationInfoIeExtensionsItem>);

