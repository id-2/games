// This file was auto-generated by Fern from our API Definition.

package games

import (
	json "encoding/json"
	fmt "fmt"
	sdk "sdk"
	core "sdk/core"
)

type GetLobbyLogsRequest struct {
	Stream LogStream `json:"-"`
	// A query parameter denoting the requests watch index.
	WatchIndex *string `json:"-"`
}

type DeleteMatchmakerLobbyResponse struct {
	// Whether or not the lobby was successfully stopped.
	DidRemove bool `json:"did_remove"`

	_rawJSON json.RawMessage
}

func (d *DeleteMatchmakerLobbyResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler DeleteMatchmakerLobbyResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*d = DeleteMatchmakerLobbyResponse(value)
	d._rawJSON = json.RawMessage(data)
	return nil
}

func (d *DeleteMatchmakerLobbyResponse) String() string {
	if len(d._rawJSON) > 0 {
		if value, err := core.StringifyJSON(d._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(d); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", d)
}

type ExportLobbyLogsRequest struct {
	Stream LogStream `json:"stream,omitempty"`

	_rawJSON json.RawMessage
}

func (e *ExportLobbyLogsRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler ExportLobbyLogsRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*e = ExportLobbyLogsRequest(value)
	e._rawJSON = json.RawMessage(data)
	return nil
}

func (e *ExportLobbyLogsRequest) String() string {
	if len(e._rawJSON) > 0 {
		if value, err := core.StringifyJSON(e._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(e); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", e)
}

type ExportLobbyLogsResponse struct {
	// The URL to a CSV file for the given lobby history.
	Url string `json:"url"`

	_rawJSON json.RawMessage
}

func (e *ExportLobbyLogsResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler ExportLobbyLogsResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*e = ExportLobbyLogsResponse(value)
	e._rawJSON = json.RawMessage(data)
	return nil
}

func (e *ExportLobbyLogsResponse) String() string {
	if len(e._rawJSON) > 0 {
		if value, err := core.StringifyJSON(e._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(e); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", e)
}

type ExportMatchmakerLobbyHistoryRequest struct {
	// Unsigned 64 bit integer.
	QueryStart int64 `json:"query_start"`
	// Unsigned 64 bit integer.
	QueryEnd int64 `json:"query_end"`

	_rawJSON json.RawMessage
}

func (e *ExportMatchmakerLobbyHistoryRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler ExportMatchmakerLobbyHistoryRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*e = ExportMatchmakerLobbyHistoryRequest(value)
	e._rawJSON = json.RawMessage(data)
	return nil
}

func (e *ExportMatchmakerLobbyHistoryRequest) String() string {
	if len(e._rawJSON) > 0 {
		if value, err := core.StringifyJSON(e._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(e); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", e)
}

type ExportMatchmakerLobbyHistoryResponse struct {
	// The URL to a CSV file for the given lobby history.
	Url string `json:"url"`

	_rawJSON json.RawMessage
}

func (e *ExportMatchmakerLobbyHistoryResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler ExportMatchmakerLobbyHistoryResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*e = ExportMatchmakerLobbyHistoryResponse(value)
	e._rawJSON = json.RawMessage(data)
	return nil
}

func (e *ExportMatchmakerLobbyHistoryResponse) String() string {
	if len(e._rawJSON) > 0 {
		if value, err := core.StringifyJSON(e._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(e); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", e)
}

type GetLobbyLogsResponse struct {
	// Sorted old to new.
	Lines []string `json:"lines,omitempty"`
	// Sorted old to new.
	Timestamps []string           `json:"timestamps,omitempty"`
	Watch      *sdk.WatchResponse `json:"watch,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetLobbyLogsResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetLobbyLogsResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetLobbyLogsResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetLobbyLogsResponse) String() string {
	if len(g._rawJSON) > 0 {
		if value, err := core.StringifyJSON(g._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(g); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", g)
}

type LogStream string

const (
	LogStreamStdOut LogStream = "std_out"
	LogStreamStdErr LogStream = "std_err"
)

func NewLogStreamFromString(s string) (LogStream, error) {
	switch s {
	case "std_out":
		return LogStreamStdOut, nil
	case "std_err":
		return LogStreamStdErr, nil
	}
	var t LogStream
	return "", fmt.Errorf("%s is not a valid %T", s, t)
}

func (l LogStream) Ptr() *LogStream {
	return &l
}
