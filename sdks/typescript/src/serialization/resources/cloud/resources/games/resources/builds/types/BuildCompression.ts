/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../..";
import * as Rivet from "../../../../../../../../api";
import * as core from "../../../../../../../../core";

export const BuildCompression: core.serialization.Schema<
    serializers.cloud.games.BuildCompression.Raw,
    Rivet.cloud.games.BuildCompression
> = core.serialization.enum_(["none", "lz4"]);

export declare namespace BuildCompression {
    type Raw = "none" | "lz4";
}
