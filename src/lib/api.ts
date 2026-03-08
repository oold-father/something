/**
 * API 客户端
 */
import { invoke } from '@tauri-apps/api/core';
import type * as Api from '../types/api';

console.log('[API] API client loaded');

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
    console.log('[API] createTag called with:', JSON.stringify(request));
    try {
      // 将请求对象解构为单独的参数
      const result = await invoke('create_tag', {
        name: request.name,
        displayName: request.displayName,
        color: request.color,
        icon: request.icon,
      });
      console.log('[API] createTag result:', result);
      return result as number;
    } catch (error) {
      console.error('[API] createTag error:', error);
      throw error;
    }
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
    tags?: string[],
    limit?: number,
    offset?: number
  ): Promise<Api.SearchResultResponse> => {
    return invoke('search_files', {
      keywords,
      operator,
      fileTypeFilter,
      tags,
      limit: limit ?? 50,
      offset: offset ?? 0,
    });
  },

  // ===== 监控目录操作 =====
  getWatchedDirectories: async (): Promise<Api.WatchedDirectory[]> => {
    return invoke('get_watched_directories');
  },

  addWatchedDirectory: async (request: Api.CreateWatchedDirectoryRequest): Promise<number> => {
    console.log('[API] addWatchedDirectory called with:', request);
    try {
      const result = await invoke('add_watched_directory', {
        path: request.path,
        recursive: request.recursive,
        filters: request.filters,
      });
      console.log('[API] addWatchedDirectory result:', result);
      return result as number;
    } catch (error) {
      console.error('[API] addWatchedDirectory error:', error);
      throw error;
    }
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

  scanAllDirectories: async (): Promise<Api.BatchScanResult> => {
    return invoke('scan_all_directories');
  },

  // ===== 系统操作 =====
  getStats: async (): Promise<Api.SystemStats> => {
    return invoke('get_stats');
  },
};
