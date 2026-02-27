/**
 * 工具函数
 */

import type { ClassValue } from 'clsx';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * 格式化文件大小
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

/**
 * 格式化日期
 */
export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return '刚刚';
  if (diffMins < 60) return `${diffMins} 分钟前`;
  if (diffHours < 24) return `${diffHours} 小时前`;
  if (diffDays < 7) return `${diffDays} 天前`;

  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  });
}

/**
 * 获取文件图标
 */
export function getFileIcon(extension: string): string {
  const ext = extension.toLowerCase();
  const iconMap: Record<string, string> = {
    // 图片
    jpg: '🖼️', jpeg: '🖼️', png: '🖼️', gif: '🖼️', webp: '🖼️', svg: '🖼️',
    // 音频
    mp3: '🎵', wav: '🎵', flac: '🎵', aac: '🎵', m4a: '🎵',
    // 视频
    mp4: '🎬', avi: '🎬', mkv: '🎬', mov: '🎬', webm: '🎬',
    // 文档
    txt: '📄', md: '📄', pdf: '📕', doc: '📘', docx: '📘',
    // 压缩
    zip: '📦', rar: '📦', '7z': '📦', tar: '📦',
    // 代码
    js: '📜', ts: '📜', py: '🐍', rs: '🦀', java: '☕',
    // 其他
    exe: '⚙️', dll: '⚙️', bin: '⚙️',
  };

  return iconMap[ext] || '📄';
}

/**
 * 获取文件类型图标
 */
export function getFileTypeIcon(fileType: string): string {
  const iconMap: Record<string, string> = {
    image: '🖼️',
    audio: '🎵',
    video: '🎬',
    text: '📄',
    binary: '⚙️',
    other: '📄',
  };

  return iconMap[fileType] || '📄';
}
