import { describe, it, expect, vi, beforeEach } from 'vitest';
import FileList from '../FileList';

describe('FileList', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('组件文件存在', () => {
    const FileList = require('../FileList').default;
    expect(typeof FileList).toBe('function');
  });
});
