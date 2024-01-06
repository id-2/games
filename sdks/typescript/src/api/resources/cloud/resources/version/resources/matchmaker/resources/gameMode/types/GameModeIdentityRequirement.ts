/**
 * This file was auto-generated by Fern from our API Definition.
 */

/**
 * **Deprecated**
 * The registration requirement for a user when joining/finding/creating a lobby. "None" allows for connections without an identity.
 */
export type GameModeIdentityRequirement = "none" | "guest" | "registered";

export const GameModeIdentityRequirement = {
    None: "none",
    Guest: "guest",
    Registered: "registered",
} as const;
