/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../..";
import * as Rivet from "../../../../api";
import * as core from "../../../../core";

export const WatchQuery: core.serialization.Schema<serializers.WatchQuery.Raw, Rivet.WatchQuery> = core.serialization
    .string()
    .optional();

export declare namespace WatchQuery {
    type Raw = string | null | undefined;
}
