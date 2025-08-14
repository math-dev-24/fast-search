-- Table des fichiers avec toutes les métadonnées
CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    is_dir INTEGER NOT NULL,
    file_type TEXT,
    size INTEGER,
    last_modified INTEGER,
    created_at INTEGER,
    accessed_at INTEGER,
    is_indexed INTEGER NOT NULL DEFAULT 0,
    content_indexed INTEGER NOT NULL DEFAULT 0,
    is_indexable INTEGER NOT NULL DEFAULT 1,
    is_hidden INTEGER NOT NULL DEFAULT 0,
    is_readonly INTEGER NOT NULL DEFAULT 0,
    is_system INTEGER NOT NULL DEFAULT 0,
    is_executable INTEGER NOT NULL DEFAULT 0,
    is_symlink INTEGER NOT NULL DEFAULT 0,
    permissions INTEGER,
    owner TEXT,
    "group" TEXT,
    mime_type TEXT,
    encoding TEXT,
    line_count INTEGER,
    word_count INTEGER,
    checksum TEXT,
    is_encrypted INTEGER NOT NULL DEFAULT 0
);

-- Table des types de fichiers
CREATE TABLE IF NOT EXISTS types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Table des chemins de recherche
CREATE TABLE IF NOT EXISTS paths (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL UNIQUE
);

-- Table de recherche plein texte pour le contenu des fichiers (unicode tokenizer, diacritics folding, et préfixes)
CREATE VIRTUAL TABLE IF NOT EXISTS fts_content USING fts5(
    content,
    file_id UNINDEXED,
    tokenize = 'unicode61 remove_diacritics 2',
    prefix = '3 4',
    detail = 'none'
);

-- Index pour optimiser les performances de recherche
CREATE INDEX IF NOT EXISTS idx_files_name ON files(name COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_files_is_dir ON files(is_dir);
CREATE INDEX IF NOT EXISTS idx_files_file_type ON files(file_type);
CREATE INDEX IF NOT EXISTS idx_files_size ON files(size);
CREATE INDEX IF NOT EXISTS idx_files_last_modified ON files(last_modified);
CREATE INDEX IF NOT EXISTS idx_files_created_at ON files(created_at);
CREATE INDEX IF NOT EXISTS idx_files_accessed_at ON files(accessed_at);
CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
CREATE INDEX IF NOT EXISTS idx_files_is_hidden ON files(is_hidden);
CREATE INDEX IF NOT EXISTS idx_files_is_readonly ON files(is_readonly);
CREATE INDEX IF NOT EXISTS idx_files_is_system ON files(is_system);
CREATE INDEX IF NOT EXISTS idx_files_is_executable ON files(is_executable);
CREATE INDEX IF NOT EXISTS idx_files_is_symlink ON files(is_symlink);
CREATE INDEX IF NOT EXISTS idx_files_permissions ON files(permissions);
CREATE INDEX IF NOT EXISTS idx_files_owner ON files(owner);
CREATE INDEX IF NOT EXISTS idx_files_group ON files("group");
CREATE INDEX IF NOT EXISTS idx_files_mime_type ON files(mime_type);
CREATE INDEX IF NOT EXISTS idx_files_encoding ON files(encoding);
CREATE INDEX IF NOT EXISTS idx_files_line_count ON files(line_count);
CREATE INDEX IF NOT EXISTS idx_files_word_count ON files(word_count);
CREATE INDEX IF NOT EXISTS idx_files_is_encrypted ON files(is_encrypted);

-- Pour les recherches les plus courantes
CREATE INDEX IF NOT EXISTS idx_files_type_size ON files(file_type, size);
CREATE INDEX IF NOT EXISTS idx_files_dir_modified ON files(is_dir, last_modified);
CREATE INDEX IF NOT EXISTS idx_files_indexed_type ON files(is_indexed, file_type);
CREATE INDEX IF NOT EXISTS idx_files_path_name ON files(path, name);

-- Pour les filtres complexes
CREATE INDEX IF NOT EXISTS idx_files_flags ON files(is_hidden, is_system, is_readonly);

-- Index pour les autres tables
CREATE INDEX IF NOT EXISTS idx_types_name ON types(name);
CREATE INDEX IF NOT EXISTS idx_paths_path ON paths(path);

-- Insertion de quelques types de fichiers courants
INSERT OR IGNORE INTO types (name) VALUES 
    ('pdf'), ('doc'), ('docx'), ('txt'), ('md'), ('html'), ('css'), ('js'),
    ('json'), ('xml'), ('csv'), ('log'), ('jpg'), ('jpeg'), ('png'), ('gif'),
    ('webp'), ('svg'), ('mp3'), ('mp4'), ('zip'), ('rar'), ('7z'), ('tar'),
    ('gz'), ('rs'), ('py'), ('java'), ('cpp'), ('c'), ('go'), ('php'), ('rb'),
    ('swift'), ('kt'), ('exe'), ('dll'), ('so'), ('dylib');

-- Nettoyage d’index redondants (sécurisé si exécuté plusieurs fois)
DROP INDEX IF EXISTS idx_files_path;
DROP INDEX IF EXISTS idx_files_path_name;
DROP INDEX IF EXISTS idx_files_is_dir;
DROP INDEX IF EXISTS idx_files_file_type;
DROP INDEX IF EXISTS idx_files_is_hidden;

ANALYZE;

PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = 10000;
PRAGMA temp_store = MEMORY;
PRAGMA mmap_size = 268435456;