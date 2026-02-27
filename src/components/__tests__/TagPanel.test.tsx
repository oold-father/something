import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import TagPanel from '../TagPanel';

// Mock API
vi.mock('../../lib/api', () => ({
  api: {
    getAllTags: vi.fn(() => Promise.resolve([
      {
        id: 1,
        name: '图片',
        displayName: '图片',
        tagType: 'system',
        color: '#F59E0B',
        useCount: 10,
      },
    ])),
  },
}));

// Mock store
vi.mock('../../stores/useStore', () => ({
  useStore: vi.fn(() => ({
    tags: [
      {
        id: 1,
        name: '图片',
        displayName: '图片',
        tagType: 'system',
        color: '#F59E0B',
        useCount: 10,
      },
    ],
    setTags: vi.fn(),
    selectedTags: [],
    toggleTag: vi.fn(),
    clearSelectedTags: vi.fn(),
  })),
}));

describe('TagPanel', () => {
  it('应该渲染标签列表', async () => {
    render(<TagPanel />);

    // 等待标签加载
    await screen.findByText('图片');
  });

  it('应该显示系统标签分组', async () => {
    render(<TagPanel />);

    await screen.findByText('系统标签');
  });
});
