/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../..";

export interface PutRequest {
    namespaceId?: string;
    key: Rivet.kv.Key;
    value?: Rivet.kv.Value;
}
