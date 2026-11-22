/** Esquema de adaptación IPC; la prueba Rust lo contrasta con la serialización del host. */
export const coreStatusContract = {
  "phase": {
    "type": "enum",
    "values": ["starting", "connected", "reconnecting", "error"]
  },
  "isRecovering": { "type": "boolean" },
  "recovery": {
    "type": "nullable_enum",
    "values": [null, "initial_snapshot", "cursor_resumed", "fresh_snapshot"]
  },
  "observedAtUnixMs": { "type": "number" }
} as const;

type ContractValue<T> = T extends { type: "boolean" }
  ? boolean
  : T extends { type: "number" }
    ? number
    : T extends { type: "enum"; values: readonly string[] }
      ? T["values"][number]
      : T extends {
            type: "nullable_enum";
            values: readonly (string | null)[];
          }
        ? T["values"][number]
        : never;

export type CoreStatusSnapshot = {
  [Field in keyof typeof coreStatusContract]: ContractValue<
    (typeof coreStatusContract)[Field]
  >;
};

export type CoreConnectionPhase = CoreStatusSnapshot["phase"];
export type CoreRecoveryResult = Exclude<CoreStatusSnapshot["recovery"], null>;
