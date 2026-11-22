import { useCallback, useEffect, useState } from "react";
import {
  getCoreStatus,
  listenForCoreStatus,
  retryCoreConnection,
  type CoreStatusSnapshot,
} from "../../bridge/core";

const initialStatus: CoreStatusSnapshot = {
  phase: "starting",
  isRecovering: false,
  recovery: null,
  observedAtUnixMs: 0,
};

/** Mantiene el estado del host sincronizado con eventos y atiende un reintento manual. */
export function useCoreStatus() {
  const [status, setStatus] = useState(initialStatus);
  const [retrying, setRetrying] = useState(false);

  useEffect(() => {
    let active = true;
    let eventRevision = 0;
    let unlisten: (() => void) | undefined;

    const acceptStatus = (next: CoreStatusSnapshot) => {
      eventRevision += 1;
      if (!active) return;
      setStatus(next);
      if (next.phase !== "error") setRetrying(false);
    };

    void listenForCoreStatus(acceptStatus)
      .then((stopListening) => {
        if (!active) {
          stopListening();
          return;
        }
        unlisten = stopListening;
        const revisionBeforeSnapshot = eventRevision;
        return getCoreStatus().then((snapshot) => {
          if (active && eventRevision === revisionBeforeSnapshot) {
            acceptStatus(snapshot);
          }
        });
      })
      .catch(() => {
        if (active) {
          acceptStatus({
            phase: "error",
            isRecovering: false,
            recovery: null,
            observedAtUnixMs: Date.now(),
          });
        }
      });

    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  const retry = useCallback(async () => {
    if (status.phase !== "error" || retrying) return;
    setRetrying(true);
    try {
      await retryCoreConnection();
    } catch {
      setRetrying(false);
    }
  }, [retrying, status.phase]);

  return { status, retry, retrying };
}
