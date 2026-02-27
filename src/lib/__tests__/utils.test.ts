import { describe, it, expect } from 'vitest';
import { formatFileSize, formatDate, getFileIcon, getFileTypeIcon } from '../utils';

describe('formatFileSize', () => {
  it('应该正确格式化字节', () => {
    expect(formatFileSize(0)).toBe('0 B');
    expect(formatFileSize(512)).toBe('512 B');
  });

  it('应该正确格式化 KB', () => {
    expect(formatFileSize(1024)).toBe('1 KB');
    expect(formatFileSize(1536)).toBe('1.5 KB');
  });

  it('应该正确格式化 MB', () => {
    expect(formatFileSize(1024 * 1024)).toBe('1 MB');
    expect(formatFileSize(2.5 * 1024 * 1024)).toBe('2.5 MB');
  });

  it('应该正确格式化 GB', () => {
    expect(formatFileSize(1024 * 1024 * 1024)).toBe('1 GB');
  });
});

describe('formatDate', () => {
  it('应该显示刚刚', () => {
    const now = new Date().toISOString();
    expect(formatDate(now)).toBe('刚刚');
  });

  it('应该显示分钟前', () => {
    const date = new Date(Date.now() - 5 * 60 * 1000).toISOString();
    expect(formatDate(date)).toBe('5 分钟前');
  });

  it('应该显示小时前', () => {
    const date = new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString();
    expect(formatDate(date)).toBe('2 小时前');
  });

  it('应该显示天数前', () => {
    const date = new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString();
    expect(formatDate(date)).toBe('3 天前');
  });

  it('应该显示日期格式', () => {
    const date = new Date('2024-01-15').toISOString();
    expect(formatDate(date)).toMatch(/2024/);
  });
});

describe('getFileIcon', () => {
  it('应该返回图片图标', () => {
    expect(getFileIcon('jpg')).toBe('🖼️');
    expect(getFileIcon('png')).toBe('🖼️');
  });

  it('应该返回音频图标', () => {
    expect(getFileIcon('mp3')).toBe('🎵');
    expect(getFileIcon('wav')).toBe('🎵');
  });

  it('应该返回视频图标', () => {
    expect(getFileIcon('mp4')).toBe('🎬');
    expect(getFileIcon('avi')).toBe('🎬');
  });

  it('应该返回默认图标', () => {
    expect(getFileIcon('unknown')).toBe('📄');
  });
});

describe('getFileTypeIcon', () => {
  it('应该返回正确的文件类型图标', () => {
    expect(getFileTypeIcon('image')).toBe('🖼️');
    expect(getFileTypeIcon('audio')).toBe('🎵');
    expect(getFileTypeIcon('video')).toBe('🎬');
    expect(getFileTypeIcon('text')).toBe('📄');
    expect(getFileTypeIcon('binary')).toBe('⚙️');
  });
});
