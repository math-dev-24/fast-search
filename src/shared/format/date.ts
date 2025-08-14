import {dateFile} from "../../types";

export const formatDate = (dateString: dateFile | null): string => {
    if (!dateString) return 'N/A';
    try {
        const timestampMs = dateString.secs_since_epoch * 1000 + Math.floor(dateString.nanos_since_epoch / 1000000);
        return new Date(timestampMs).toLocaleString('fr-FR');
    } catch {
        return "Date invalide";
    }
};