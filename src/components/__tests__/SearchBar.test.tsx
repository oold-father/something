import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('SearchBar', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('组件文件存在', async () => {
    const { default: SearchBar } = await import('../SearchBar');
    expect(typeof SearchBar).toBe('function');
  });
});
