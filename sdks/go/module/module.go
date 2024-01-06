// This file was auto-generated by Fern from our API Definition.

package module

import (
	json "encoding/json"
	fmt "fmt"
	core "sdk/core"
)

type CallResponse struct {
	Data interface{} `json:"data,omitempty"`

	_rawJSON json.RawMessage
}

func (c *CallResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler CallResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*c = CallResponse(value)
	c._rawJSON = json.RawMessage(data)
	return nil
}

func (c *CallResponse) String() string {
	if len(c._rawJSON) > 0 {
		if value, err := core.StringifyJSON(c._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(c); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", c)
}
