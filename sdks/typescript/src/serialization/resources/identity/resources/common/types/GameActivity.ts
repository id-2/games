/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";

export const GameActivity: core.serialization.ObjectSchema<
    serializers.identity.GameActivity.Raw,
    Rivet.identity.GameActivity
> = core.serialization.object({
    game: core.serialization.lazyObject(async () => (await import("../../../../..")).game.Handle),
    message: core.serialization.string(),
    publicMetadata: core.serialization.property("public_metadata", core.serialization.unknown().optional()),
    mutualMetadata: core.serialization.property("mutual_metadata", core.serialization.unknown().optional()),
});

export declare namespace GameActivity {
    interface Raw {
        game: serializers.game.Handle.Raw;
        message: string;
        public_metadata?: unknown | null;
        mutual_metadata?: unknown | null;
    }
}
