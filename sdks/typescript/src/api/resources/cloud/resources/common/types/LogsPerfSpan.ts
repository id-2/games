/**
 * This file was auto-generated by Fern from our API Definition.
 */

/**
 * A performance span.
 */
export interface LogsPerfSpan {
    /** The label given to this performance span. */
    label: string;
    /** RFC3339 timestamp. */
    startTs: Date;
    /** RFC3339 timestamp. */
    finishTs?: Date;
    reqId?: string;
}
