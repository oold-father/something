/**
 * API 客户端
 */
import { invoke } from '@tauri-apps/api/core';
import type * as Api from '../types/api';

export const api = {
  // ===== 文件操作 =====
  getFiles: async (params?: { limit?: number; offset?: number }): Promise<Api.File[]> => {
    return invoke('get_files', { limit: params?.limit, offset: params?.offset });
  },

  getFileById: async (id: number): Promise<Api.File> => {
    return invoke('get_file_by_id', { id });
  },

  getFileByPath: async (path: string): Promise<Api.File> => {
    return invoke('get_file_by_path', { path });
  },

  addFile: async (path: string): Promise<number> => {
    return invoke('add_file', { path });
  },

  deleteFile: async (id: number): Promise<void> => {
    return invoke('delete_file', { id });
  },

  // ===== 标签操作 =====
  getAllTags: async (): Promise<Api.Tag[]> => {
    return invoke('get_all_tags');
  },

  getTagsByFile: async (fileId: number): Promise<Api.Tag[]> => {
    return invoke('get_tags_by_file', { fileId });
  },

  createTag: async (request: Api.CreateTagRequest): Promise<number> => {
    return invoke('create_tag', { request });
  },

  addTagToFile: async (fileId: number, tagName: string): Promise<void> => {
    return invoke('add_tag_to_file', { fileId, tagName });
  },

  removeTagFromFile: async (fileId: number, tagId: number): Promise<void> => {
    return invoke('remove_tag_from_file', { fileId, tagId });
  },

  batchAddTags: async (fileIds: number[], tagNames: string[]): Promise<void> => {
    return invoke('batch_add_tags', { fileIds, tagNames });
  },

  getFilesByTags: async (tagNames: string[]): Promise<Api.File[]> => {
    return invoke('get_files_by_tags', { tagNames });
  },

  deleteTag: async (tagId: number): Promise<void> => {
    return invoke('delete_tag', { tagId });
  },

  updateTag: async (tagId: number, displayName: string, color: string): Promise<void> => {
    return invoke('update_tag', { tagId, displayName, color });
  },

  // ===== 搜索操作 =====
  searchFiles: async (
    keywords: string[],
    operator: 'AND' | 'OR',
    fileTypeFilter?: string,
    limit?: number,
    offset?: number
  ): Promise<Api.SearchResultResponse> => {
    return invoke('search_files', {
      keywords,
      operator,
      fileTypeFilter,
      limit: limit ?? 50,
      offset: offset ?? 0,
    });
  },

  // ===== 监控目录操作 =====
  getWatchedDirectories: async (): Promise<Api.WatchedDirectory[]> => {
    return invoke('get_watched_directories');
  },

  addWatchedDirectory: async (request: Api.CreateWatchedDirectoryRequest): Promise<number> => {
    return invoke('add_watched_directory', { request });
  },

  removeWatchedDirectory: async (id: number): Promise<void> => {
    return invoke('remove_watched_directory', { id });
  },

  updateWatchedDirectory: async (id: number, enabled: boolean): Promise<void> => {
    return invoke('update_watched_directory', { id, enabled });
  },

  scanDirectory: async (path: string, recursive: boolean): Promise<Api.ScanResult> => {
    return invoke('scan_directory', { path, recursive });
  },

  // ===== 系统操作 =====
  getStats: async (): Promise<Api.SystemStats> => {
    return invoke('get_stats');
  },
};
