/**
 * 全局状态管理
 */
import { create } from 'zustand';
import type { File, Tag, SearchResultResponse, SystemStats, WatchedDirectory } from '../types/api';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number; // 自动消失时间（毫秒），0 表示不自动消失
  timestamp: number;
}

export interface AppSettings {
  theme: 'light' | 'dark' | 'auto';
  accentColor: string;
  defaultSearchOperator: 'AND' | 'OR';
  searchResultLimit: number;
  autoScanInterval: number;
  showScanNotifications: boolean;
  showHiddenFiles: boolean;
}

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

  // 通知消息
  notifications: Notification[];
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp'>) => void;
  removeNotification: (id: string) => void;

  // 应用设置
  settings: AppSettings;
  updateSettings: (settings: Partial<AppSettings>) => void;

  // 主题
  theme: 'light' | 'dark';
  setTheme: (theme: 'light' | 'dark') => void;

  // 文件列表修订号（用于触发刷新）
  filesRevision: number;
  incrementFilesRevision: () => void;
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

  notifications: [],
  addNotification: (notification) => {
    const id: string = Math.random().toString(36).substr(2, 9);
    const newNotification: Notification = {
      id,
      timestamp: Date.now(),
      ...notification,
    };
    set((state) => ({
      notifications: [...state.notifications, newNotification],
    }));

    // 自动消失
    if (notification.duration !== 0) {
      setTimeout(() => {
        useStore.getState().removeNotification(id);
      }, notification.duration || 3000);
    }
  },
  removeNotification: (id) =>
    set((state) => ({
      notifications: state.notifications.filter((n) => n.id !== id),
    })),

  settings: {
    theme: 'auto',
    accentColor: '#3b82f6',
    defaultSearchOperator: 'AND',
    searchResultLimit: 50,
    autoScanInterval: 30,
    showScanNotifications: true,
    showHiddenFiles: false,
  },
  updateSettings: (newSettings) =>
    set((state) => ({
      settings: { ...state.settings, ...newSettings },
    })),

  theme: 'light',
  setTheme: (theme) => {
    set({ theme });
    // 更新 HTML class
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    // 保存到 localStorage
    localStorage.setItem('something_theme', theme);
  },

  filesRevision: 0,
  incrementFilesRevision: () => set((state) => ({ filesRevision: state.filesRevision + 1 })),
}));

// 初始化主题
if (typeof window !== 'undefined') {
  const savedTheme = localStorage.getItem('something_theme') as 'light' | 'dark' | null;
  if (savedTheme) {
    useStore.getState().setTheme(savedTheme);
  } else {
    // 检测系统主题偏好
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    useStore.getState().setTheme(prefersDark ? 'dark' : 'light');
  }
}
