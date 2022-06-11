// Partly copied from free5GC project, under Apache License 2.0, January 2004.
// License file: https://raw.githubusercontent.com/free5gc/free5gc/main/LICENSE

package main

import (
	"bufio"
	"encoding/hex"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/free5gc/CommonConsumerTestData/UDM/TestGenAuthData"
	"github.com/free5gc/MongoDBLibrary"
	"github.com/free5gc/nas"
	"github.com/free5gc/nas/nasMessage"
	"github.com/free5gc/nas/nasTestpacket"
	"github.com/free5gc/nas/nasType"
	"github.com/free5gc/nas/security"
	"github.com/free5gc/openapi/models"
)

func main() {
	log := log.New(os.Stderr, "", log.Ltime|log.Lshortfile)
	stdin := bufio.NewReader(os.Stdin)

	// Instantiate UE context
	ue := NewRanUeContext("imsi-2089300007487", 1, security.AlgCiphering128NEA0, security.AlgIntegrity128NIA2,
		models.AccessType__3_GPP_ACCESS)
	ue.AmfUeNgapId = 1
	ue.AuthenticationSubs = GetAuthSubscription(TestGenAuthData.MilenageTestSet19.K,
		TestGenAuthData.MilenageTestSet19.OPC,
		TestGenAuthData.MilenageTestSet19.OP)

	// Configure UE in free5GC (idempotent)
	MongoDBLibrary.SetMongoDB("free5gc", "mongodb://127.0.0.1:27017")
	provisionUeInFree5GC(ue)

	// Send a registration request over stdout
	mobileIdentity5GS := nasType.MobileIdentity5GS{
		Len:    12, // suci
		Buffer: []uint8{0x01, 0x02, 0xf8, 0x39, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 0x47, 0x78},
	}
	ueSecurityCapability := ue.GetUESecurityCapability()
	registrationRequest := nasTestpacket.GetRegistrationRequest(
		nasMessage.RegistrationType5GSInitialRegistration, mobileIdentity5GS, nil, ueSecurityCapability, nil, nil, nil)

	sendNas(registrationRequest)

	// Get authentication request
	authenticationRequest := recvNas(ue, stdin)
	if authenticationRequest.GmmHeader.GetMessageType() != nas.MsgTypeAuthenticationRequest {
		log.Fatal("Not an authentication request")
	}
	rand := authenticationRequest.AuthenticationRequest.GetRANDValue()
	resStat := ue.DeriveRESstarAndSetKey(ue.AuthenticationSubs, rand[:], "5G:mnc093.mcc208.3gppnetwork.org")

	// send NAS Authentication Response
	authenticationResponse := nasTestpacket.GetAuthenticationResponse(resStat, "")
	sendNas(authenticationResponse)

	time.Sleep(10 * time.Second)
}

func recvNas(ue *RanUeContext, stdin *bufio.Reader) (msg *nas.Message) {
	hexString, err := stdin.ReadString('\n')
	if err != nil {
		log.Fatal("Stdin closed - exit ue-sim")
	}
	nasBytes, _ := hex.DecodeString(hexString)
	nasPdu, _ := NASDecode(ue, nas.GetSecurityHeaderType(nasBytes), nasBytes)
	return nasPdu
}

func sendNas(msg []byte) {
	hexMessage := hex.EncodeToString(msg)
	fmt.Println(hexMessage)
}

func provisionUeInFree5GC(ue *RanUeContext) {
	servingPlmnId := "20893"
	InsertAuthSubscriptionToMongoDB(ue.Supi, ue.AuthenticationSubs)
	InsertAccessAndMobilitySubscriptionDataToMongoDB(ue.Supi, GetAccessAndMobilitySubscriptionData(), servingPlmnId)
	InsertSmfSelectionSubscriptionDataToMongoDB(ue.Supi, GetSmfSelectionSubscriptionData(), servingPlmnId)
	InsertSessionManagementSubscriptionDataToMongoDB(ue.Supi, servingPlmnId, GetSessionManagementSubscriptionData())
	InsertAmPolicyDataToMongoDB(ue.Supi, GetAmPolicyData())
	InsertSmPolicyDataToMongoDB(ue.Supi, GetSmPolicyData())
}
