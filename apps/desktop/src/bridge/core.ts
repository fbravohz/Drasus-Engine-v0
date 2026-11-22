import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  coreStatusContract,
  type CoreConnectionPhase,
  type CoreStatusSnapshot,
} from "./coreStatusContract";

export type { CoreConnectionPhase, CoreRecoveryResult, CoreStatusSnapshot } from "./coreStatusContract";

const contractFields = Object.entries(coreStatusContract) as [
  string,
  { type: string; values?: readonly (string | null)[] },
][];

function adaptCoreStatus(value: unknown): CoreStatusSnapshot {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid_core_status");
  }

  const record = value as Record<string, unknown>;
  const actualFields = Object.keys(record).sort();
  const expectedFields = contractFields.map(([field]) => field).sort();
  if (
    actualFields.length !== expectedFields.length ||
    actualFields.some((field, index) => field !== expectedFields[index])
  ) {
    throw new Error("invalid_core_status");
  }

  for (const [field, descriptor] of contractFields) {
    const fieldValue = record[field];
    const valid =
      (descriptor.type === "boolean" && typeof fieldValue === "boolean") ||
      (descriptor.type === "number" && Number.isSafeInteger(fieldValue)) ||
      ((descriptor.type === "enum" || descriptor.type === "nullable_enum") &&
        descriptor.values?.includes(fieldValue as string | null));
    if (!valid) throw new Error("invalid_core_status");
  }

  return record as CoreStatusSnapshot;
}

function invalidCoreStatus(): CoreStatusSnapshot {
  return {
    phase: "error" as CoreConnectionPhase,
    isRecovering: false,
    recovery: null,
    observedAtUnixMs: Date.now(),
  };
}

/** Solicita al host la última observación real del supervisor. */
export function getCoreStatus() {
  return invoke<unknown>("get_core_status").then(adaptCoreStatus);
}

/** Escucha transiciones publicadas por Tauri sin sondear el Core desde React. */
export function listenForCoreStatus(
  onStatus: (status: CoreStatusSnapshot) => void,
) {
  return listen<unknown>("core-status", (event) => {
    try {
      onStatus(adaptCoreStatus(event.payload));
    } catch {
      onStatus(invalidCoreStatus());
    }
  });
}

/** Pide al supervisor que reintente el arranque tras un error terminal. */
export function retryCoreConnection() {
  return invoke<void>("retry_core_connection");
}
