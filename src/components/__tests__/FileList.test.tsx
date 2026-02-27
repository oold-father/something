import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import FileList from '../FileList';
import type { File as FileType } from '../../types/api';

// Mock API
vi.mock('../../lib/api', () => ({
  api: {
    getFiles: vi.fn(() => Promise.resolve([
      {
        id: 1,
        path: '/test/file.jpg',
        name: 'file.jpg',
        extension: 'jpg',
        size: 1024 * 1024,
        fileType: 'image',
        createdAt: new Date().toISOString(),
        modifiedAt: new Date().toISOString(),
        status: 'active',
      },
    ])),
  },
}));

// Mock store
vi.mock('../../stores/useStore', () => ({
  useStore: vi.fn(() => ({
    files: [
      {
        id: 1,
        path: '/test/file.jpg',
        name: 'file.jpg',
        extension: 'jpg',
        size: 1024 * 1024,
        fileType: 'image',
        createdAt: new Date().toISOString(),
        modifiedAt: new Date().toISOString(),
        status: 'active',
      },
    ],
    setFiles: vi.fn(),
    searchResults: null,
    setSearchResults: vi.fn(),
    isLoading: false,
    setIsLoading: vi.fn(),
    searchKeywords: [],
  })),
}));

// Mock opener
vi.mock('@tauri-apps/plugin-opener', () => ({
  open: vi.fn(),
}));

describe('FileList', () => {
  it('应该渲染文件列表', () => {
    render(<FileList />);

    expect(screen.getByText('file.jpg')).toBeInTheDocument();
  });

  it('应该显示文件数量', () => {
    render(<FileList />);

    expect(screen.getByText(/共.*个文件/)).toBeInTheDocument();
  });

  it('应该显示文件信息', () => {
    render(<FileList />);

    // 验证文件名显示
    expect(screen.getByText('file.jpg')).toBeInTheDocument();

    // 验证文件大小显示（应该包含 MB）
    expect(screen.getByText(/MB/)).toBeInTheDocument();
  });
});
