import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AlertCircle, CheckmarkCircle, SyncCircle } from "@vicons/ionicons5";

export const useSync = () => {
  const inSync = ref<boolean>(false);
  const syncProgress = ref<number>(0);
  const syncMessage = ref<string>("");
  const currentPath = ref<string>("");
  const totalFiles = ref<number>(0);
  const processedFiles = ref<number>(0);
  const syncError = ref<string>("");
  const syncSuccess = ref<boolean>(false);
  const scanPhase = ref<"collecting" | "inserting" | "finished" | "error">(
    "collecting"
  );

  const listeners: UnlistenFn[] = [];

  const startSync = async () => {
    try {
      inSync.value = true;
      syncProgress.value = 0;
      syncMessage.value = "Initialisation...";
      syncError.value = "";
      syncSuccess.value = false;
      scanPhase.value = "collecting";

      await invoke("sync_files_and_folders");
    } catch (error) {
      syncError.value = `Erreur lors du démarrage: ${error}`;
      inSync.value = false;
      scanPhase.value = "error";
    }
  };

  const valueProgress = computed(() => {
    return Math.round(syncProgress.value * 1000) / 10;
  });

  const progressStatus = computed(() => {
    if (syncError.value) return "error";
    if (syncSuccess.value) return "success";
    return "info";
  });

  const statusIcon = computed(() => {
    if (syncError.value) return AlertCircle;
    if (syncSuccess.value) return CheckmarkCircle;
    return SyncCircle;
  });

  const progressText = computed(() => {
    if (syncError.value) return "Erreur";
    if (syncSuccess.value) return "Terminé";

    switch (scanPhase.value) {
      case "collecting":
        return currentPath.value
          ? `Collecte: ${
              currentPath.value.split("/").pop() ||
              currentPath.value.split("\\").pop()
            }`
          : "Collecte en cours...";
      case "inserting":
        return `Insertion: ${processedFiles.value}/${totalFiles.value}`;
      case "finished":
        return `Terminé: ${totalFiles.value} fichiers`;
      default:
        return syncMessage.value;
    }
  });

  onMounted(async () => {
    listeners.push(
      await listen("scan_files_started", () => {
        inSync.value = true;
        syncProgress.value = 0;
        syncMessage.value = "Démarrage du scan...";
        scanPhase.value = "collecting";
        syncError.value = "";
        syncSuccess.value = false;
      })
    );

    listeners.push(
      await listen("scan_files_progress", (event: any) => {
        const payload = event.payload;
        syncProgress.value = payload.progress;
        syncMessage.value = payload.message;
        currentPath.value = payload.current_path;
        scanPhase.value = "collecting";
      })
    );

    listeners.push(
      await listen("scan_files_collected", (event: any) => {
        const payload = event.payload;
        totalFiles.value = payload.total;
        syncMessage.value = payload.message;
        scanPhase.value = "inserting";
        syncProgress.value = 0;
      })
    );

    listeners.push(
      await listen("scan_files_insert_progress", (event: any) => {
        const payload = event.payload;
        syncProgress.value = payload.progress;
        processedFiles.value = payload.processed;
        totalFiles.value = payload.total;
        scanPhase.value = "inserting";
      })
    );

    listeners.push(
      await listen("scan_files_finished", (event: any) => {
        const payload = event.payload;
        inSync.value = false;
        syncProgress.value = 100;
        syncMessage.value = payload.message;
        totalFiles.value = payload.total;
        scanPhase.value = "finished";
        syncSuccess.value = true;

        setTimeout(() => {
          syncSuccess.value = false;
          syncProgress.value = 0;
        }, 3000);
      })
    );

    listeners.push(
      await listen("scan_files_error", (event: any) => {
        inSync.value = false;
        syncError.value = event.payload as string;
        scanPhase.value = "error";
        syncProgress.value = 0;

        // Auto-hide après 5 secondes
        setTimeout(() => {
          syncError.value = "";
        }, 5000);
      })
    );
  });

  onUnmounted(() => {
    listeners.forEach((unlisten) => unlisten());
  });

  return {
    inSync,
    syncMessage,
    currentPath,
    totalFiles,
    processedFiles,
    syncError,
    syncSuccess,
    scanPhase,
    startSync,
    listeners,
    valueProgress,
    progressStatus,
    statusIcon,
    progressText,
  };
};
