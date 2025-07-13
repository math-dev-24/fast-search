import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";  
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AlertCircle, CheckmarkCircle, SyncCircle, FolderOutline, DocumentTextOutline } from "@vicons/ionicons5";
import { Phase } from "../types";

// Constantes pour les événements (cohérentes avec le backend)
const EVENTS = {
  SCAN: {
    STARTED: "scan_files_started",
    PROGRESS: "scan_files_progress",
    COLLECTED: "scan_files_collected",
    INSERT_PROGRESS: "scan_files_insert_progress",
    FINISHED: "scan_files_finished",
    ERROR: "scan_files_error"
  },
  INDEX: {
    STARTED: "index_content_started",
    PROGRESS: "index_content_progress",
    FINISHED: "index_content_finished",
    ERROR: "index_content_error"
  },
  STAT: {
    UPDATED: "stat_updated"
  }
} as const;

// Types pour les payloads d'événements
interface ScanProgressPayload {
  progress: number;
  message: string;
  current_path: string;
}

interface ScanCollectedPayload {
  total: number;
  message: string;
}

interface InsertProgressPayload {
  progress: number;
  processed: number;
  total: number;
}

interface ScanFinishedPayload {
  total: number;
  message: string;
}

interface IndexProgressPayload {
  progress: number;
  message: string;
  processed: number;
  total: number;
}

interface IndexFinishedPayload {
  total: number;
  message: string;
}

interface ProcessState {
  isActive: boolean;
  progress: number;
  message: string;
  currentPath: string;
  total: number;
  processed: number;
  phase: Phase;
  error: string;
  success: boolean;
}

export const useSync = () => {
  // États séparés pour chaque processus
  const scanState = ref<ProcessState>({
    isActive: false,
    progress: 0,
    message: "",
    currentPath: "",
    total: 0,
    processed: 0,
    phase: "collecting",
    error: "",
    success: false
  });

  const indexState = ref<ProcessState>({
    isActive: false,
    progress: 0,
    message: "",
    currentPath: "",
    total: 0,
    processed: 0,
    phase: "collecting",
    error: "",
    success: false
  });

  const inSync = computed(() => {
    return scanState.value.isActive || indexState.value.isActive;
  });

  const hasError = computed(() => {
    return scanState.value.error || indexState.value.error;
  });

  const hasSuccess = computed(() => {
    return scanState.value.success || indexState.value.success;
  });

  const overallProgress = computed(() => {
    if (!inSync.value) return 0;
    
    const scanWeight = 0.5; // Le scan représente 50% du travail total
    const indexWeight = 0.5; // L'indexation représente 50% du travail total
    
    const scanProgress = scanState.value.isActive ? scanState.value.progress : (scanState.value.success ? 100 : 0);
    const indexProgress = indexState.value.isActive ? indexState.value.progress : (indexState.value.success ? 100 : 0);
    
    return Math.round((scanProgress * scanWeight + indexProgress * indexWeight) * 10) / 10;
  });

  const progressStatus = computed((): 'info' | 'success' | 'error' | 'warning' => {
    if (hasError.value) return "error";
    if (hasSuccess.value && !inSync.value) return "success";
    return "info";
  });

  const statusIcon = computed(() => {
    if (hasError.value) return AlertCircle;
    if (hasSuccess.value && !inSync.value) return CheckmarkCircle;
    return SyncCircle;
  });

  const syncSummary = computed(() => {
    const activeProcesses = [];
    if (scanState.value.isActive) activeProcesses.push("Scan");
    if (indexState.value.isActive) activeProcesses.push("Indexation");
    
    if (activeProcesses.length === 0) {
      if (hasError.value) return "Erreur de synchronisation";
      if (hasSuccess.value) return "Synchronisation terminée";
      return "";
    }
    
    return `Synchronisation en cours (${activeProcesses.join(", ")})`;
  });

  const processDetails = computed(() => {
    const details = [];
    
    if (scanState.value.isActive || scanState.value.success) {
      const scanDetail = {
        name: "Scan des fichiers",
        icon: FolderOutline,
        isActive: scanState.value.isActive,
        progress: scanState.value.progress,
        message: scanState.value.message,
        currentPath: scanState.value.currentPath,
        phase: scanState.value.phase,
        error: scanState.value.error,
        success: scanState.value.success
      };
      details.push(scanDetail);
    }
    
    if (indexState.value.isActive || indexState.value.success) {
      const indexDetail = {
        name: "Indexation du contenu",
        icon: DocumentTextOutline,
        isActive: indexState.value.isActive,
        progress: indexState.value.progress,
        message: indexState.value.message,
        currentPath: indexState.value.currentPath,
        phase: indexState.value.phase,
        error: indexState.value.error,
        success: indexState.value.success
      };
      details.push(indexDetail);
    }
    
    return details;
  });

  const listeners: UnlistenFn[] = [];

  // Fonction utilitaire pour gérer les erreurs d'événements
  const handleEventError = (eventName: string, error: any) => {
    console.error(`Erreur lors du traitement de l'événement ${eventName}:`, error);
    
    // Afficher l'erreur dans l'interface utilisateur
    const errorMessage = `Erreur de communication: ${error}`;
    scanState.value.error = errorMessage;
    indexState.value.error = errorMessage;
    
    // Reset automatique après 10 secondes
    setTimeout(() => {
      scanState.value.error = "";
      indexState.value.error = "";
    }, 10000);
  };

  // Fonction utilitaire pour reset un état de processus
  const resetProcessState = (state: ProcessState, isActive: boolean = false) => {
    state.isActive = isActive;
    state.progress = 0;
    state.message = "";
    state.currentPath = "";
    state.total = 0;
    state.processed = 0;
    state.phase = "idle";
    state.error = "";
    state.success = false;
  };

  const startSync = async () => {
    try {
      // Reset des états avant de commencer
      resetProcessState(scanState.value, true);
      resetProcessState(indexState.value, true);
      
      scanState.value.message = "Initialisation du scan des fichiers...";
      indexState.value.message = "Initialisation de l'indexation du contenu...";

      // Diagnostic préventif des chemins
      try {
        const paths = await invoke("get_all_paths");
        if (Array.isArray(paths) && paths.length > 0) {
          const issues = await invoke("diagnose_scan_issues", { paths });
          if (Array.isArray(issues) && issues.length > 0 && !issues[0].includes("Aucun problème")) {
            console.warn("Problèmes détectés avant le scan:", issues);
            scanState.value.message = "Problèmes détectés, scan en cours...";
          }
        }
      } catch (diagnosticError) {
        console.warn("Erreur lors du diagnostic:", diagnosticError);
      }

      // Ajout d'un timeout pour éviter les blocages
      const timeoutPromise = new Promise((_, reject) => {
        setTimeout(() => reject(new Error("Timeout: Le scan prend trop de temps")), 600000); // 10 minutes
      });

      const syncPromise = Promise.all([
        invoke("sync_files_and_folders"),
        invoke("start_content_indexing")
      ]);

      await Promise.race([syncPromise, timeoutPromise]);
    } catch (error) {
      const errorMsg = `Erreur lors du démarrage: ${error}`;
      console.error(errorMsg);
      
      scanState.value.error = errorMsg;
      indexState.value.error = errorMsg;
      scanState.value.isActive = false;
      indexState.value.isActive = false;
      
      // Reset automatique après 15 secondes
      setTimeout(() => {
        scanState.value.error = "";
        indexState.value.error = "";
      }, 15000);
    }
  };

  onMounted(async () => {
    try {
      // Événements pour le scan des fichiers
      listeners.push(
        await listen(EVENTS.SCAN.STARTED, () => {
          resetProcessState(scanState.value, true);
          scanState.value.message = "Démarrage du scan des fichiers...";
        })
      );

      listeners.push(
        await listen(EVENTS.SCAN.PROGRESS, (event: any) => {
          try {
            const payload = event.payload as ScanProgressPayload;
            scanState.value.progress = payload.progress;
            scanState.value.message = payload.message;
            scanState.value.currentPath = payload.current_path;
            scanState.value.phase = "collecting";
          } catch (error) {
            handleEventError(EVENTS.SCAN.PROGRESS, error);
          }
        })
      );

      listeners.push(
        await listen(EVENTS.SCAN.COLLECTED, (event: any) => {
          try {
            const payload = event.payload as ScanCollectedPayload;
            scanState.value.total = payload.total;
            scanState.value.message = payload.message;
            scanState.value.phase = "inserting";
            scanState.value.progress = 0;
          } catch (error) {
            handleEventError(EVENTS.SCAN.COLLECTED, error);
          }
        })
      );

      listeners.push(
        await listen(EVENTS.SCAN.INSERT_PROGRESS, (event: any) => {
          try {
            const payload = event.payload as InsertProgressPayload;
            scanState.value.progress = payload.progress;
            scanState.value.processed = payload.processed;
            scanState.value.total = payload.total;
            scanState.value.phase = "inserting";
          } catch (error) {
            handleEventError(EVENTS.SCAN.INSERT_PROGRESS, error);
          }
        })
      );

      listeners.push(
        await listen(EVENTS.SCAN.FINISHED, (event: any) => {
          try {
            const payload = event.payload as ScanFinishedPayload;
            scanState.value.isActive = false;
            scanState.value.progress = 100;
            scanState.value.message = payload.message;
            scanState.value.total = payload.total;
            scanState.value.phase = "finished";
            scanState.value.success = true;

            // Reset après 3 secondes
            setTimeout(() => {
              scanState.value.success = false;
              scanState.value.progress = 0;
            }, 3000);
          } catch (error) {
            handleEventError(EVENTS.SCAN.FINISHED, error);
          }
        })
      );

      listeners.push(
        await listen(EVENTS.SCAN.ERROR, (event: any) => {
          try {
            scanState.value.isActive = false;
            scanState.value.error = event.payload as string;
            scanState.value.phase = "error";
            scanState.value.progress = 0;

            setTimeout(() => {
              scanState.value.error = "";
            }, 5000);
          } catch (error) {
            handleEventError(EVENTS.SCAN.ERROR, error);
          }
        })
      );

      // Événements pour l'indexation du contenu
      listeners.push(
        await listen(EVENTS.INDEX.STARTED, () => {
          resetProcessState(indexState.value, true);
          indexState.value.message = "Analyse des fichiers à indexer...";
        })
      );

      listeners.push(
        await listen(EVENTS.INDEX.PROGRESS, (event: any) => {
          try {
            const payload = event.payload as IndexProgressPayload;
            indexState.value.progress = payload.progress;
            indexState.value.message = payload.message;
            indexState.value.processed = payload.processed;
            indexState.value.total = payload.total;
            indexState.value.phase = "collecting";
          } catch (error) {
            handleEventError(EVENTS.INDEX.PROGRESS, error);
          }
        })
      );

      listeners.push(
        await listen(EVENTS.INDEX.FINISHED, (event: any) => {
          try {
            const payload = event.payload as IndexFinishedPayload;
            indexState.value.isActive = false;
            indexState.value.progress = 100;
            indexState.value.message = payload.message;
            indexState.value.total = payload.total;
            indexState.value.phase = "finished";
            indexState.value.success = true;

            // Reset après 3 secondes
            setTimeout(() => {
              indexState.value.success = false;
              indexState.value.progress = 0;
            }, 3000);
          } catch (error) {
            handleEventError(EVENTS.INDEX.FINISHED, error);
          }
        })
      );

      listeners.push(
        await listen(EVENTS.INDEX.ERROR, (event: any) => {
          try {
            indexState.value.isActive = false;
            indexState.value.error = event.payload as string;
            indexState.value.phase = "error";
            indexState.value.progress = 0;

            setTimeout(() => {
              indexState.value.error = "";
            }, 5000);
          } catch (error) {
            handleEventError(EVENTS.INDEX.ERROR, error);
          }
        })
      );

    } catch (error) {
      console.error("Erreur lors de l'initialisation des listeners d'événements:", error);
    }
  });

  onUnmounted(() => {
    listeners.forEach((unlisten) => {
      try {
        unlisten();
      } catch (error) {
        console.error("Erreur lors de la suppression d'un listener:", error);
      }
    });
  });

  return {
    inSync,
    hasError,
    hasSuccess,
    overallProgress,
    progressStatus,
    statusIcon,
    syncSummary,
    processDetails,
    scanState,
    indexState,
    startSync,
  };
};
