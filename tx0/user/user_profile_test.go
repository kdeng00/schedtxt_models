package user

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInitUserProfile(t *testing.T) {
	usr := User{Username: "Bob", PhoneNumber: "+12224572351", Password: "TheNight!"}

	profile, err := InitUserProfile(&usr)

	assert.NoError(t, err, "Error %v", err)
	assert.NotEmpty(t, profile, "UserProfile is empty")
}
