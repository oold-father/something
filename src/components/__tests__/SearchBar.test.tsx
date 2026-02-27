import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import SearchBar from '../SearchBar';

// Mock store
vi.mock('../../stores/useStore', () => ({
  useStore: vi.fn(() => ({
    searchKeywords: [],
    setSearchKeywords: vi.fn(),
    searchOperator: 'AND',
    setSearchOperator: vi.fn(),
    isLoading: false,
    setIsLoading: vi.fn(),
  })),
}));

describe('SearchBar', () => {
  it('应该渲染搜索输入框', () => {
    render(<SearchBar />);
    const input = screen.getByPlaceholderText('搜索文件...');
    expect(input).toBeInTheDocument();
  });

  it('应该允许输入搜索关键字', async () => {
    const user = userEvent.setup();
    render(<SearchBar />);

    const input = screen.getByPlaceholderText('搜索文件...');
    await user.type(input, 'test file');

    expect(input).toHaveValue('test file');
  });

  it('按回车应该触发搜索', async () => {
    const user = userEvent.setup();
    render(<SearchBar />);

    const input = screen.getByPlaceholderText('搜索文件...');
    await user.type(input, 'test{Enter}');

    // 验证 setKeywords 被调用
    const { useStore } = await import('../../stores/useStore');
    const mockStore = useStore() as any;
    expect(mockStore.setSearchKeywords).toHaveBeenCalledWith(['test']);
  });
});
