import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('TagPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('组件文件存在', () => {
    const TagPanel = require('../TagPanel').default;
    expect(typeof TagPanel).toBe('function');
  });
});
