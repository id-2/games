// This file was auto-generated by Fern from our API Definition.

package client

import (
	bytes "bytes"
	context "context"
	json "encoding/json"
	errors "errors"
	io "io"
	http "net/http"
	sdk "sdk"
	cloud "sdk/cloud"
	auth "sdk/cloud/auth"
	devicesclient "sdk/cloud/devices/client"
	gamesclient "sdk/cloud/games/client"
	groups "sdk/cloud/groups"
	logs "sdk/cloud/logs"
	tiers "sdk/cloud/tiers"
	uploads "sdk/cloud/uploads"
	core "sdk/core"
)

type Client struct {
	baseURL string
	caller  *core.Caller
	header  http.Header

	Games   *gamesclient.Client
	Auth    *auth.Client
	Devices *devicesclient.Client
	Groups  *groups.Client
	Logs    *logs.Client
	Tiers   *tiers.Client
	Uploads *uploads.Client
}

func NewClient(opts ...core.ClientOption) *Client {
	options := core.NewClientOptions()
	for _, opt := range opts {
		opt(options)
	}
	return &Client{
		baseURL: options.BaseURL,
		caller:  core.NewCaller(options.HTTPClient),
		header:  options.ToHeader(),
		Games:   gamesclient.NewClient(opts...),
		Auth:    auth.NewClient(opts...),
		Devices: devicesclient.NewClient(opts...),
		Groups:  groups.NewClient(opts...),
		Logs:    logs.NewClient(opts...),
		Tiers:   tiers.NewClient(opts...),
		Uploads: uploads.NewClient(opts...),
	}
}

// Returns the basic information required to use the cloud APIs.
func (c *Client) Bootstrap(ctx context.Context) (*cloud.BootstrapResponse, error) {
	baseURL := "https://api.rivet.gg"
	if c.baseURL != "" {
		baseURL = c.baseURL
	}
	endpointURL := baseURL + "/" + "cloud/bootstrap"

	errorDecoder := func(statusCode int, body io.Reader) error {
		raw, err := io.ReadAll(body)
		if err != nil {
			return err
		}
		apiError := core.NewAPIError(statusCode, errors.New(string(raw)))
		decoder := json.NewDecoder(bytes.NewReader(raw))
		switch statusCode {
		case 500:
			value := new(sdk.InternalError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 429:
			value := new(sdk.RateLimitError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 403:
			value := new(sdk.ForbiddenError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 408:
			value := new(sdk.UnauthorizedError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 404:
			value := new(sdk.NotFoundError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		case 400:
			value := new(sdk.BadRequestError)
			value.APIError = apiError
			if err := decoder.Decode(value); err != nil {
				return apiError
			}
			return value
		}
		return apiError
	}

	var response *cloud.BootstrapResponse
	if err := c.caller.Call(
		ctx,
		&core.CallParams{
			URL:          endpointURL,
			Method:       http.MethodGet,
			Headers:      c.header,
			Response:     &response,
			ErrorDecoder: errorDecoder,
		},
	); err != nil {
		return nil, err
	}
	return response, nil
}
