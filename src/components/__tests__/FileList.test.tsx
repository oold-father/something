import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('FileList', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('组件文件存在', async () => {
    const { default: FileList } = await import('../FileList');
    expect(typeof FileList).toBe('function');
  });
});
