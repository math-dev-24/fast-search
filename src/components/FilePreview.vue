<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { NModal, NButton, NText, NSpin, NImage, NScrollbar } from 'naive-ui';
import { Close, Eye } from '@vicons/ionicons5';
import type { File } from '../types';

const props = defineProps<{
    show: boolean;
    file: File | null;
}>();

const emit = defineEmits<{
    (e: 'update:show', value: boolean): void;
}>();

const loading = ref(false);
const previewContent = ref<string>('');
const previewError = ref<string>('');
const fileUrl = ref<string>('');

// Variable réactive pour le v-model de la modal
const show = computed({
    get: () => props.show,
    set: (value: boolean) => emit('update:show', value)
});

// Convertir le chemin de fichier en URL accessible
const getFileUrl = async (filePath: string) => {
    try {
        // Utiliser le protocole asset:// pour les fichiers dans Tauri
        return `asset://${filePath}`;
    } catch (error) {
        console.error('Erreur lors de la conversion du chemin:', error);
        return null;
    }
};

const fileType = computed(() => {
    if (!props.file) return null;
    
    const extension = props.file.name.split('.').pop()?.toLowerCase();
    
    if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(extension || '')) {
        return 'image';
    } else if (['pdf'].includes(extension || '')) {
        return 'pdf';
    } else if (['txt', 'md', 'json', 'xml', 'csv', 'log', 'ini', 'conf', 'cfg'].includes(extension || '')) {
        return 'text';
    }
    
    return null;
});

// Charger le contenu du fichier pour la prévisualisation
const loadPreview = async () => {
    if (!props.file || !fileType.value) return;
    
    loading.value = true;
    previewError.value = '';
    
    try {
        console.log('Chargement de la prévisualisation pour:', props.file.path);
        console.log('Type de fichier:', fileType.value);
        
        // Générer l'URL du fichier
        fileUrl.value = await getFileUrl(props.file.path) || '';
        console.log('URL générée:', fileUrl.value);
        
        if (fileType.value === 'text') {
            // Pour les fichiers texte, on peut essayer de lire le contenu
            // Note: Cela nécessitera une API Tauri pour lire le fichier
            previewContent.value = 'Prévisualisation du fichier texte...';
        }
    } catch (error) {
        previewError.value = 'Impossible de charger la prévisualisation';
        console.error('Erreur lors du chargement de la prévisualisation:', error);
    } finally {
        loading.value = false;
    }
};

const openFile = () => {
    if (props.file) {
        (window as any).__TAURI__?.shell?.open(props.file.path);
    }
};

const closeModal = () => {
    emit('update:show', false);
    previewContent.value = '';
    previewError.value = '';
};

watch(() => props.file, () => {
    if (props.file) {
        loadPreview();
    }
}, { immediate: true });

watch(() => props.show, (newValue) => {
    if (newValue && props.file) {
        loadPreview();
    }
});

</script>

<template>
    <NModal
        v-model:show="show"
        preset="card"
        title="Prévisualisation du fichier"
        class="w-[90vw] max-w-4xl"
        :mask-closable="false"
        :closable="true"
    >
        <div v-if="file" class="space-y-4">
            <!-- En-tête avec informations du fichier -->
            <div class="flex items-center justify-between p-4 rounded-lg">
                <div class="flex-1">
                    <NText class="text-lg font-semibold">{{ file.name }}</NText>
                </div>
                <div class="flex gap-2">
                    <NButton @click="openFile" type="primary" size="small">
                        <template #icon>
                            <Eye />
                        </template>
                        Ouvrir
                    </NButton>
                </div>
            </div>

            <!-- Debug info -->
            <div class="p-2 bg-yellow-800 rounded text-xs">
                <p>Debug: Type={{ fileType }}, URL={{ fileUrl }}, Loading={{ loading }}</p>
            </div>
            
            <!-- Contenu de prévisualisation -->
            <div class="min-h-[400px] max-h-[70vh] overflow-hidden">
                <NSpin v-if="loading" size="large" />
                
                <div v-else-if="previewError" class="flex items-center justify-center h-64 text-red-500">
                    <NText>{{ previewError }}</NText>
                </div>
                
                <div v-else-if="fileType === 'image'" class="flex items-center justify-center">
                    <NImage
                        :src="fileUrl"
                        :alt="file.name"
                        class="max-w-full max-h-full object-contain"
                        :preview-disabled="false"
                        :show-toolbar="true"
                    />
                </div>
                
                <div v-else-if="fileType === 'pdf'" class="h-full">
                    <iframe
                        :src="fileUrl"
                        class="w-full h-full border-0"
                        title="PDF Preview"
                    />
                </div>
                
                <div v-else-if="fileType === 'text'" class="h-full">
                    <NScrollbar class="h-full">
                        <pre class="p-4 bg-gray-50 rounded-lg text-sm font-mono whitespace-pre-wrap">{{ previewContent }}</pre>
                    </NScrollbar>
                </div>
                
                <div v-else class="flex items-center justify-center h-64 text-gray-500">
                    <NText>Prévisualisation non disponible pour ce type de fichier</NText>
                </div>
            </div>
        </div>
    </NModal>
</template> 