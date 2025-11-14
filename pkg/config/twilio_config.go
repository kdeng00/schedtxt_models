package config

import (
	"fmt"
)


type TwiloConfig struct {
	AccountSID string `json:"auth_sid"`
	ServiceSID string `json:"service_sid"`
	URI        string `json:"uri"`
	AuthToken  string `json:"auth_token"`
	Number     string `json:"phone_number"`
}

func (config *TwiloConfig) PrintConfig() {
	fmt.Printf("Account SID: %s\n", config.AccountSID)
	fmt.Printf("Service SID: %s\n", config.ServiceSID)
	fmt.Println("URI:", config.URI)
	fmt.Printf("Auth Token: %s\n", config.AuthToken)
	fmt.Printf("Number: %s\n", config.Number)
}
