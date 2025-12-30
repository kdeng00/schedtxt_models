package config

import (
	"fmt"
)

type TwilioConfig struct {
	AccountSID string `json:"auth_sid"`
	ServiceSID string `json:"service_sid"`
	AuthToken  string `json:"auth_token"`
	Number     string `json:"phone_number"`
}

func (config *TwilioConfig) PrintConfig() {
	fmt.Println("Twilio config")
	fmt.Println("Account SID:", config.AccountSID)
	fmt.Println("Service SID:", config.ServiceSID)
	fmt.Println("Auth Token:", config.AuthToken)
	fmt.Println("Number:", config.Number)
}
