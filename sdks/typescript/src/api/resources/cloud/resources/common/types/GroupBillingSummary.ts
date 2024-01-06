/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../..";

/**
 * A group billing summary.
 */
export interface GroupBillingSummary {
    /** A list of multiple game lobby expenses. */
    games: Rivet.cloud.GameLobbyExpenses[];
    /** A group's available balance. */
    balance: number;
}
