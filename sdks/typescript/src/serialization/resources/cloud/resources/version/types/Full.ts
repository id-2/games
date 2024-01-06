/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";

export const Full: core.serialization.ObjectSchema<serializers.cloud.version.Full.Raw, Rivet.cloud.version.Full> =
    core.serialization.object({
        versionId: core.serialization.property("version_id", core.serialization.string()),
        createTs: core.serialization.property("create_ts", core.serialization.date()),
        displayName: core.serialization.property("display_name", core.serialization.string()),
        config: core.serialization.lazyObject(async () => (await import("../../../../..")).cloud.version.Config),
    });

export declare namespace Full {
    interface Raw {
        version_id: string;
        create_ts: string;
        display_name: string;
        config: serializers.cloud.version.Config.Raw;
    }
}
