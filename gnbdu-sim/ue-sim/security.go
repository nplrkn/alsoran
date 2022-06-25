// Copied from free5GC project, under Apache License 2.0, January 2004 with minor changes.
// License file: https://raw.githubusercontent.com/free5gc/free5gc/main/LICENSE

package main

import (
	"fmt"
	"reflect"

	"github.com/free5gc/nas"
	"github.com/free5gc/nas/nasMessage"
	"github.com/free5gc/nas/security"
)

func NASEncode(ue *RanUeContext, msg *nas.Message, securityContextAvailable bool, newSecurityContext bool) (
	payload []byte, err error) {
	var sequenceNumber uint8
	if ue == nil {
		err = fmt.Errorf("amfUe is nil")
		return
	}
	if msg == nil {
		err = fmt.Errorf("NAS Message is empty")
		return
	}

	if !securityContextAvailable {
		return msg.PlainNasEncode()
	} else {
		if newSecurityContext {
			ue.ULCount.Set(0, 0)
			ue.DLCount.Set(0, 0)
		}

		sequenceNumber = ue.ULCount.SQN()
		payload, err = msg.PlainNasEncode()
		if err != nil {
			return
		}

		if err = security.NASEncrypt(ue.CipheringAlg, ue.KnasEnc, ue.ULCount.Get(), ue.GetBearerType(),
			security.DirectionUplink, payload); err != nil {
			return
		}
		// add sequece number
		payload = append([]byte{sequenceNumber}, payload[:]...)
		mac32 := make([]byte, 4)
		_ = mac32
		// fmt.Println("sequenceNumber", sequenceNumber)
		// fmt.Println("ue.IntegrityAlg", ue.IntegrityAlg)
		// fmt.Println("ue.KnasInt", ue.KnasInt)
		// fmt.Println("ue.ULCount.Get()", ue.ULCount.Get())
		// fmt.Println("security.Bearer3GPP", security.Bearer3GPP)
		// fmt.Println("security.DirectionUplink", security.DirectionUplink)
		// fmt.Println("payload", payload)

		mac32, err = security.NASMacCalculate(ue.IntegrityAlg, ue.KnasInt, ue.ULCount.Get(), ue.GetBearerType(),
			security.DirectionUplink, payload)
		if err != nil {
			return
		}

		// Add mac value
		payload = append(mac32, payload[:]...)
		// Add EPD and Security Type
		msgSecurityHeader := []byte{msg.SecurityHeader.ProtocolDiscriminator, msg.SecurityHeader.SecurityHeaderType}
		payload = append(msgSecurityHeader, payload[:]...)

		// Increase UL Count
		ue.ULCount.AddOne()
	}
	return payload, err
}

func NASDecode(ue *RanUeContext, securityHeaderType uint8, payload []byte) (msg *nas.Message, err error) {
	if ue == nil {
		err = fmt.Errorf("amfUe is nil")
		return
	}
	if payload == nil {
		err = fmt.Errorf("NAS payload is empty")
		return
	}

	msg = new(nas.Message)
	msg.SecurityHeaderType = uint8(nas.GetSecurityHeaderType(payload) & 0x0f)
	if securityHeaderType == nas.SecurityHeaderTypePlainNas {
		err = msg.PlainNasDecode(&payload)
		return
	} else if ue.IntegrityAlg == security.AlgIntegrity128NIA0 {
		//fmt.Println("decode payload is ", payload)
		// remove header
		payload = payload[3:]
		if err = security.NASEncrypt(ue.CipheringAlg, ue.KnasEnc, ue.DLCount.Get(), ue.GetBearerType(),
			security.DirectionDownlink, payload); err != nil {
			return nil, err
		}

		err = msg.PlainNasDecode(&payload)
		return
	} else { // Security protected NAS message
		securityHeader := payload[0:6]
		sequenceNumber := payload[6]
		receivedMac32 := securityHeader[2:]
		// remove security Header except for sequece Number
		payload = payload[6:]

		// a security protected NAS message must be integrity protected, and ciphering is optional
		ciphered := false
		switch msg.SecurityHeaderType {
		case nas.SecurityHeaderTypeIntegrityProtected:
			//fmt.Println("Security header type: Integrity Protected")
		case nas.SecurityHeaderTypeIntegrityProtectedAndCiphered:
			//fmt.Println("Security header type: Integrity Protected And Ciphered")
			ciphered = true
		case nas.SecurityHeaderTypeIntegrityProtectedWithNew5gNasSecurityContext:
			//fmt.Println("Security Header Type Integrity Protected With New 5g Nas Security Context")
			ue.DLCount.Set(0, 0)
		case nas.SecurityHeaderTypeIntegrityProtectedAndCipheredWithNew5gNasSecurityContext:
			//fmt.Println("Security header type: Integrity Protected And Ciphered With New 5G Security Context")
			ciphered = true
			ue.DLCount.Set(0, 0)
		default:
			return nil, fmt.Errorf("wrong security header type: 0x%0x", msg.SecurityHeader.SecurityHeaderType)
		}
		// Caculate ul count
		if ue.DLCount.SQN() > sequenceNumber {
			ue.DLCount.SetOverflow(ue.DLCount.Overflow() + 1)
		}
		ue.DLCount.SetSQN(sequenceNumber)

		mac32, errNas := security.NASMacCalculate(ue.IntegrityAlg, ue.KnasInt, ue.DLCount.Get(), ue.GetBearerType(),
			security.DirectionDownlink, payload)
		if errNas != nil {
			return nil, errNas
		}
		if !reflect.DeepEqual(mac32, receivedMac32) {
			return nil, fmt.Errorf("NAS MAC verification failed(0x%x != 0x%x)", mac32, receivedMac32)
		}

		// fmt.Printf("cmac value: 0x%x\n", mac32)

		// remove sequece Number
		payload = payload[1:]
		if ciphered {
			if err = security.NASEncrypt(ue.CipheringAlg, ue.KnasEnc, ue.DLCount.Get(), ue.GetBearerType(),
				security.DirectionDownlink, payload); err != nil {
				return nil, err
			}
		}

		err = msg.PlainNasDecode(&payload)
		return msg, err
	}
}

func EncodeNasPduWithSecurity(ue *RanUeContext, pdu []byte, securityHeaderType uint8,
	securityContextAvailable, newSecurityContext bool) ([]byte, error) {
	m := nas.NewMessage()
	err := m.PlainNasDecode(&pdu)
	if err != nil {
		return nil, err
	}
	m.SecurityHeader = nas.SecurityHeader{
		ProtocolDiscriminator: nasMessage.Epd5GSMobilityManagementMessage,
		SecurityHeaderType:    securityHeaderType,
	}
	return NASEncode(ue, m, securityContextAvailable, newSecurityContext)
}
