/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Rivet from "../../../../..";

/**
 * A group's billing transfer.
 */
export interface GroupBillingTransfer {
    /** Payment amount (in hundredths USD, 100 = $1.00). */
    amount: number;
    /** A description of this transfer. */
    description?: string;
    /** RFC3339 timestamp. */
    createdTs: Date;
    status: Rivet.cloud.GroupBillingStatus;
}
