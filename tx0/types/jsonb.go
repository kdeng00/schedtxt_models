package types

import (
	"database/sql/driver"
	"encoding/json"
	"fmt"
)

type JSONB json.RawMessage

// Scan implements sql.Scanner
func (j *JSONB) Scan(value interface{}) error {
	if value == nil {
		*j = nil
		return nil
	}

	var data []byte
	switch v := value.(type) {
	case []byte:
		// pgx may pass data as []byte in binary format
		data = v
	case string:
		// pgx may pass data as string in text format
		data = []byte(v)
	default:
		return fmt.Errorf("unsupported type: %T", v)
	}

	*j = JSONB(data)
	return nil
}

// Value implements driver.Valuer
func (j JSONB) Value() (driver.Value, error) {
	if j == nil {
		return nil, nil
	} else {
		return []byte(j), nil
	}
}
