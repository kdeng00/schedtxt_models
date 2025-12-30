package token

import (
	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
)

type Claims struct {
	UserId uuid.UUID `json:"user_id"`
	Role   string    `json:"role"`
	jwt.RegisteredClaims
}

type LoginResult struct {
	UserId      uuid.UUID `json:"user_id"`
	AccessToken string    `json:"access_token"`
	TokenType   string    `json:"token_type"`
	ExpiresIn   int64     `json:"expires_in"`
	IssuedAt    int64     `json:"issued_at"`
}

// Alias for LoginResult
// Will be deprecated at some point
type Login = LoginResult
