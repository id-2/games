/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../..";
import * as Rivet from "../../../../../../api";
import * as core from "../../../../../../core";

export const GetGameLinkNewIdentity: core.serialization.ObjectSchema<
    serializers.identity.GetGameLinkNewIdentity.Raw,
    Rivet.identity.GetGameLinkNewIdentity
> = core.serialization.object({
    identityToken: core.serialization.property(
        "identity_token",
        core.serialization.lazy(async () => (await import("../../../../..")).Jwt)
    ),
    identityTokenExpireTs: core.serialization.property("identity_token_expire_ts", core.serialization.date()),
    identity: core.serialization.lazyObject(async () => (await import("../../../../..")).identity.Profile),
});

export declare namespace GetGameLinkNewIdentity {
    interface Raw {
        identity_token: serializers.Jwt.Raw;
        identity_token_expire_ts: string;
        identity: serializers.identity.Profile.Raw;
    }
}
