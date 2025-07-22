<script setup lang="ts">
import { computed } from 'vue';
import { NModal, NButton, NText, NDescriptions, NDescriptionsItem, NTag, NSpace, NDivider, NFlex, NIcon } from 'naive-ui';
import { Eye, FolderOutline, DocumentOutline, InformationCircleOutline } from '@vicons/ionicons5';
import type { File } from '../types';

const props = defineProps<{
    show: boolean;
    file: File | null;
}>();

const emit = defineEmits<{
    (e: 'update:show', value: boolean): void;
}>();

const show = computed({
    get: () => props.show,
    set: (value: boolean) => emit('update:show', value)
});

// Formater la taille en bytes
const formatSize = (bytes: number | null): string => {
    if (bytes === null) return 'N/A';
    if (bytes === 0) return '0 B';
    
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

// Formater les permissions
const formatPermissions = (permissions: number | null): string => {
    if (permissions === null) return 'N/A';
    return permissions.toString(8).padStart(4, '0');
};

// Formater la date
const formatDate = (dateString: string | null): string => {
    if (!dateString) return 'N/A';
    try {
        return new Date(dateString).toLocaleString('fr-FR');
    } catch {
        return dateString;
    }
};

// Grouper les métadonnées par catégorie
const metadataGroups = computed(() => {
    if (!props.file) return [];
    
    const file = props.file;
    
    return [
        {
            title: 'Informations générales',
            icon: file.is_dir ? FolderOutline : DocumentOutline,
            items: [
                { label: 'Nom', value: file.name },
                { label: 'Chemin', value: file.path },
                { label: 'Type', value: file.is_dir ? 'Dossier' : 'Fichier' },
                { label: 'Taille', value: formatSize(file.size) },
                { label: 'Type MIME', value: file.mime_type || 'N/A' },
                { label: 'Encodage', value: file.encoding || 'N/A' },
            ]
        },
        {
            title: 'Dates',
            icon: InformationCircleOutline,
            items: [
                { label: 'Créé le', value: formatDate(file.created_at) },
                { label: 'Modifié le', value: formatDate(file.last_modified) },
                { label: 'Accédé le', value: formatDate(file.accessed_at) },
            ]
        },
        {
            title: 'Permissions et propriété',
            icon: InformationCircleOutline,
            items: [
                { label: 'Permissions', value: formatPermissions(file.permissions) },
                { label: 'Propriétaire', value: file.owner || 'N/A' },
                { label: 'Groupe', value: file.group || 'N/A' },
            ]
        },
        {
            title: 'Attributs',
            icon: InformationCircleOutline,
            items: [
                { label: 'Indexé', value: file.is_indexed ? 'Oui' : 'Non' },
                { label: 'Contenu indexé', value: file.content_indexed ? 'Oui' : 'Non' },
                { label: 'Indexable', value: file.is_indexable ? 'Oui' : 'Non' },
                { label: 'Caché', value: file.is_hidden ? 'Oui' : 'Non' },
                { label: 'Lecture seule', value: file.is_readonly ? 'Oui' : 'Non' },
                { label: 'Système', value: file.is_system ? 'Oui' : 'Non' },
                { label: 'Exécutable', value: file.is_executable ? 'Oui' : 'Non' },
                { label: 'Lien symbolique', value: file.is_symlink ? 'Oui' : 'Non' },
                { label: 'Chiffré', value: file.is_encrypted ? 'Oui' : 'Non' },
            ]
        }
    ];
});

// Ajouter les métadonnées spécifiques aux fichiers texte si applicable
const textMetadata = computed(() => {
    if (!props.file || props.file.is_dir) return null;
    
    const items = [];
    if (props.file.line_count !== null) {
        items.push({ label: 'Nombre de lignes', value: props.file.line_count.toString() });
    }
    if (props.file.word_count !== null) {
        items.push({ label: 'Nombre de mots', value: props.file.word_count.toString() });
    }
    if (props.file.checksum) {
        items.push({ label: 'Checksum', value: props.file.checksum });
    }
    
    return items.length > 0 ? items : null;
});

const openFile = () => {
    if (props.file) {
        (window as any).__TAURI__?.shell?.open(props.file.path);
    }
};

</script>

<template>
    <NModal
        v-model:show="show"
        preset="card"
        class="w-[90vw] max-w-4xl max-h-[90vh] overflow-y-auto"
        :mask-closable="false"
        :closable="true"
    >
        <div v-if="file" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div class="flex items-center gap-3 flex-1">
                    <component :is="file.is_dir ? FolderOutline : DocumentOutline" class="text-2xl text-slate-400 h-10 w-10" />
                    <NFlex vertical>
                        <NText class="text-xl font-bold text-white">{{ file.name }}</NText>
                    </NFlex>
                </div>
                <NButton @click="openFile" type="primary" size="small">
                    <template #icon>
                        <Eye />
                    </template>
                    Ouvrir
                </NButton>
            </div>
            <NDivider />

            <!-- Métadonnées groupées -->
            <div v-for="(group, index) in metadataGroups" :key="index" class="space-y-3">
                <div class="flex items-center gap-2">
                    <component :is="group.icon" class="text-lg text-gray-400 h-6 w-6" />
                    <NText class="text-lg font-medium">{{ group.title }}</NText>
                </div>
                
                <NDescriptions :column="2" bordered>
                    <NDescriptionsItem 
                        v-for="item in group.items" 
                        :key="item.label"
                        :label="item.label"
                    >
                        <NText>{{ item.value }}</NText>
                    </NDescriptionsItem>
                </NDescriptions>
            </div>

            <!-- Métadonnées spécifiques aux fichiers texte -->
            <div v-if="textMetadata" class="space-y-3">
                <div class="flex items-center gap-2">
                    <NIcon size="24">
                        <DocumentOutline class="text-lg text-slate-400" />
                    </NIcon>
                    <NText class="text-lg font-medium">Informations du contenu</NText>
                </div>
                
                <NDescriptions :column="2" bordered>
                    <NDescriptionsItem 
                        v-for="item in textMetadata" 
                        :key="item.label"
                        :label="item.label"
                    >
                        <NText>{{ item.value }}</NText>
                    </NDescriptionsItem>
                </NDescriptions>
            </div>

            <!-- Tags pour les attributs spéciaux -->
            <div class="space-y-3">
                <NText class="text-lg font-medium">Attributs spéciaux</NText>
                <NSpace>
                    <NTag v-if="file.is_hidden" type="warning" size="small">Caché</NTag>
                    <NTag v-if="file.is_readonly" type="error" size="small">Lecture seule</NTag>
                    <NTag v-if="file.is_system" type="error" size="small">Système</NTag>
                    <NTag v-if="file.is_executable" type="success" size="small">Exécutable</NTag>
                    <NTag v-if="file.is_symlink" type="info" size="small">Lien symbolique</NTag>
                    <NTag v-if="file.is_encrypted" type="warning" size="small">Chiffré</NTag>
                    <NTag v-if="file.is_indexed" type="success" size="small">Indexé</NTag>
                    <NTag v-if="file.content_indexed" type="success" size="small">Contenu indexé</NTag>
                </NSpace>
            </div>
        </div>
    </NModal>
</template> 