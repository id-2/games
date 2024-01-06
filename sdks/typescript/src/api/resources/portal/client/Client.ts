/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as environments from "../../../../environments";
import * as core from "../../../../core";
import { Games } from "../resources/games/client/Client";
import { Notifications } from "../resources/notifications/client/Client";

export declare namespace Portal {
    interface Options {
        environment?: core.Supplier<environments.RivetEnvironment | string>;
        token?: core.Supplier<core.BearerToken | undefined>;
        fetcher?: core.FetchFunction;
    }

    interface RequestOptions {
        timeoutInSeconds?: number;
        maxRetries?: number;
    }
}

export class Portal {
    constructor(protected readonly _options: Portal.Options = {}) {}

    protected _games: Games | undefined;

    public get games(): Games {
        return (this._games ??= new Games(this._options));
    }

    protected _notifications: Notifications | undefined;

    public get notifications(): Notifications {
        return (this._notifications ??= new Notifications(this._options));
    }
}
