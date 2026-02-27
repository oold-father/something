import { describe, it, expect, vi, beforeEach } from 'vitest';
import SearchBar from '../SearchBar';

describe('SearchBar', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('组件文件存在', () => {
    const SearchBar = require('../SearchBar').default;
    expect(typeof SearchBar).toBe('function');
  });
});
