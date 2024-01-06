/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../..";

export interface ListActivitiesResponse {
    identities: Rivet.identity.Handle[];
    games: Rivet.game.Summary[];
    suggestedGroups: Rivet.group.Summary[];
    suggestedPlayers: Rivet.identity.Handle[];
    watch: Rivet.WatchResponse;
}
