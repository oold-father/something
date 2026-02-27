/**
 * API 类型定义
 */

// ===== 文件相关 =====
export interface File {
  id?: number;
  path: string;
  name: string;
  extension: string;
  size: number;
  fileType: 'image' | 'audio' | 'video' | 'text' | 'binary' | 'other';
  createdAt: string;
  modifiedAt: string;
  status: 'active' | 'deleted' | 'moved';
}

export interface FileWithTags extends File {
  tags: Tag[];
}

// ===== 标签相关 =====
export interface Tag {
  id?: number;
  name: string;
  displayName: string;
  tagType: 'system' | 'custom';
  color: string;
  icon?: string;
  useCount: number;
}

// ===== 搜索相关 =====
export interface SearchQuery {
  keywords: string[];
  operator: 'AND' | 'OR';
  fileTypeFilter?: string;
  limit: number;
  offset: number;
}

export interface SearchResult {
  file: File;
  tags: Tag[];
  relevance: number;
}

export interface SearchResultResponse {
  results: SearchResult[];
  total: number;
}

// ===== 监控目录相关 =====
export interface WatchedDirectory {
  id?: number;
  path: string;
  recursive: boolean;
  filters?: DirectoryFilters;
  enabled: boolean;
  createdAt: string;
  lastScannedAt?: string;
}

export interface DirectoryFilters {
  extensions?: string[];
  exclude?: string[];
}

// ===== 系统统计 =====
export interface SystemStats {
  totalFiles: number;
  indexedFiles: number;
  totalTags: number;
  watchedDirectories: number;
}

// ===== 请求/响应类型 =====
export interface CreateTagRequest {
  name: string;
  displayName: string;
  color: string;
  icon?: string;
}

export interface CreateWatchedDirectoryRequest {
  path: string;
  recursive: boolean;
  filters?: DirectoryFilters;
}

export interface ScanResult {
  scanPath: string;
  scannedFiles: number;
  addedFiles: number;
  updatedFiles: number;
  skippedFiles: number;
  errors: Array<[string, string]>;
}
