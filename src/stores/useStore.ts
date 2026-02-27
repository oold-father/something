/**
 * 全局状态管理
 */
import { create } from 'zustand';
import type { File, Tag, SearchResultResponse, SystemStats, WatchedDirectory } from '../types/api';

interface AppState {
  // 文件列表
  files: File[];
  setFiles: (files: File[]) => void;

  // 标签列表
  tags: Tag[];
  setTags: (tags: Tag[]) => void;

  // 搜索结果
  searchResults: SearchResultResponse | null;
  setSearchResults: (results: SearchResultResponse | null) => void;

  // 选中的标签
  selectedTags: string[];
  toggleTag: (tagName: string) => void;
  clearSelectedTags: () => void;

  // 系统统计
  stats: SystemStats | null;
  setStats: (stats: SystemStats) => void;

  // 监控目录
  watchedDirectories: WatchedDirectory[];
  setWatchedDirectories: (dirs: WatchedDirectory[]) => void;

  // 搜索关键字
  searchKeywords: string[];
  setSearchKeywords: (keywords: string[]) => void;

  // 搜索运算符
  searchOperator: 'AND' | 'OR';
  setSearchOperator: (operator: 'AND' | 'OR') => void;

  // 是否正在加载
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
}

export const useStore = create<AppState>((set) => ({
  files: [],
  setFiles: (files) => set({ files }),

  tags: [],
  setTags: (tags) => set({ tags }),

  searchResults: null,
  setSearchResults: (results) => set({ searchResults: results }),

  selectedTags: [],
  toggleTag: (tagName) =>
    set((state) => {
      const isSelected = state.selectedTags.includes(tagName);
      return {
        selectedTags: isSelected
          ? state.selectedTags.filter((t) => t !== tagName)
          : [...state.selectedTags, tagName],
      };
    }),
  clearSelectedTags: () => set({ selectedTags: [] }),

  stats: null,
  setStats: (stats) => set({ stats }),

  watchedDirectories: [],
  setWatchedDirectories: (dirs) => set({ watchedDirectories: dirs }),

  searchKeywords: [],
  setSearchKeywords: (keywords) => set({ searchKeywords: keywords }),

  searchOperator: 'AND',
  setSearchOperator: (operator) => set({ searchOperator: operator }),

  isLoading: false,
  setIsLoading: (loading) => set({ isLoading: loading }),
}));
