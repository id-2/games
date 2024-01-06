// This file was auto-generated by Fern from our API Definition.

package group

import (
	json "encoding/json"
	fmt "fmt"
	core "sdk/core"
)

type ResolveJoinRequestRequest struct {
	Resolution *bool `json:"resolution,omitempty"`

	_rawJSON json.RawMessage
}

func (r *ResolveJoinRequestRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler ResolveJoinRequestRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*r = ResolveJoinRequestRequest(value)
	r._rawJSON = json.RawMessage(data)
	return nil
}

func (r *ResolveJoinRequestRequest) String() string {
	if len(r._rawJSON) > 0 {
		if value, err := core.StringifyJSON(r._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(r); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", r)
}
