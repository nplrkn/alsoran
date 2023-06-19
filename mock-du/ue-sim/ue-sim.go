// Portions are copied from free5GC project, licensed under Apache License 2.0, January 2004 - see FREE5GC-LICENSE.txt.

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
	// Set up I/O
	log := log.New(os.Stderr, "", log.Ltime|log.Lshortfile)
	stdin := bufio.NewReader(os.Stdin)

	// Register a UE
	ue := new_ue()
	register(ue, stdin, log)

	// Wait for a line on stdin before exiting
	stdin.ReadString('\n')
}

func new_ue() *RanUeContext {
	// Instantiate UE context
	ue := NewRanUeContext("imsi-2089300007487", 1, security.AlgCiphering128NEA0, security.AlgIntegrity128NIA2,
		models.AccessType__3_GPP_ACCESS)
	ue.AmfUeNgapId = 1
	ue.AuthenticationSubs = GetAuthSubscription(TestGenAuthData.MilenageTestSet19.K,
		TestGenAuthData.MilenageTestSet19.OPC,
		TestGenAuthData.MilenageTestSet19.OP)

	// Configure UE in free5GC (idempotent)
	provisionUeInFree5GC(ue)
	return ue
}

func register(ue *RanUeContext, stdin *bufio.Reader, log *log.Logger) {
	// Send Registration Request
	mobileIdentity5GS := nasType.MobileIdentity5GS{
		Len:    12, // suci
		Buffer: []uint8{0x01, 0x02, 0xf8, 0x39, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 0x47, 0x78},
	}
	ueSecurityCapability := ue.GetUESecurityCapability()
	registrationRequest := nasTestpacket.GetRegistrationRequest(
		nasMessage.RegistrationType5GSInitialRegistration, mobileIdentity5GS, nil, ueSecurityCapability, nil, nil, nil)
	sendNas(registrationRequest)

	// Identity request would go here

	// Receive Authentication Request
	authenticationRequest := recvNas(ue, stdin)
	if authenticationRequest.GmmHeader.GetMessageType() != nas.MsgTypeAuthenticationRequest {
		log.Fatal("Not an authentication request")
	}

	// Calculate key
	rand := authenticationRequest.AuthenticationRequest.GetRANDValue()
	resStat := ue.DeriveRESstarAndSetKey(ue.AuthenticationSubs, rand[:], "5G:mnc093.mcc208.3gppnetwork.org")

	// Send Authentication Response
	authenticationResponse := nasTestpacket.GetAuthenticationResponse(resStat, "")
	sendNas(authenticationResponse)

	// Receive Security Mode Command.
	securityModeCommand := recvNas(ue, stdin)
	if securityModeCommand.GmmHeader.GetMessageType() != nas.MsgTypeSecurityModeCommand {
		log.Fatal("Not a security mode command")
	}

	// Send Security Mode Complete
	registrationRequestWith5GMM := nasTestpacket.GetRegistrationRequest(nasMessage.RegistrationType5GSInitialRegistration,
		mobileIdentity5GS, nil, ueSecurityCapability, ue.Get5GMMCapability(), nil, nil)
	securityModeComplete := nasTestpacket.GetSecurityModeComplete(registrationRequestWith5GMM)
	securityModeComplete, _ = EncodeNasPduWithSecurity(ue, securityModeComplete, nas.SecurityHeaderTypeIntegrityProtectedAndCipheredWithNew5gNasSecurityContext, true, true)
	sendNas(securityModeComplete)

	// Receive Registration Accept.
	registrationAccept := recvNas(ue, stdin)
	if registrationAccept.GmmHeader.GetMessageType() != nas.MsgTypeRegistrationAccept {
		log.Fatal("Not a Registration Accept")
	}

	// Send Registration Complete
	registrationComplete := nasTestpacket.GetRegistrationComplete(nil)
	registrationComplete, _ = EncodeNasPduWithSecurity(ue, registrationComplete, nas.SecurityHeaderTypeIntegrityProtectedAndCiphered, true, false)
	sendNas(registrationComplete)
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
	done := make(chan bool, 1)
	go func() {
		done <- doProvisionUeInFree5GC(ue)
	}()
	select {
	case <-time.After(1 * time.Second):
		log.Fatal("Timed out trying to provision Ue in MongoDb")
	case <-done:
		return
	}

}

func doProvisionUeInFree5GC(ue *RanUeContext) bool {
	MongoDBLibrary.SetMongoDB("free5gc", "mongodb://127.0.0.1:27017")
	servingPlmnId := "20893"
	InsertAuthSubscriptionToMongoDB(ue.Supi, ue.AuthenticationSubs)
	InsertAccessAndMobilitySubscriptionDataToMongoDB(ue.Supi, GetAccessAndMobilitySubscriptionData(), servingPlmnId)
	InsertSmfSelectionSubscriptionDataToMongoDB(ue.Supi, GetSmfSelectionSubscriptionData(), servingPlmnId)
	InsertSessionManagementSubscriptionDataToMongoDB(ue.Supi, servingPlmnId, GetSessionManagementSubscriptionData())
	InsertAmPolicyDataToMongoDB(ue.Supi, GetAmPolicyData())
	InsertSmPolicyDataToMongoDB(ue.Supi, GetSmPolicyData())
	return true
}
