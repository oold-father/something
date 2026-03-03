/**
 * 文件列表组件
 */
import { useEffect, useState } from 'react';
import { File, Calendar, HardDrive, RefreshCw, Download } from 'lucide-react';
import { api } from '../../lib/api';
import { formatFileSize, formatDate, getFileIcon } from '../../lib/utils';
import { useStore } from '../../stores/useStore';
import type { File as FileType } from '../../types/api';

export default function FileList() {
  const files = useStore((s) => s.files);
  const setFiles = useStore((s) => s.setFiles);
  const searchResults = useStore((s) => s.searchResults);
  const isLoading = useStore((s) => s.isLoading);
  const setIsLoading = useStore((s) => s.setIsLoading);
  const selectedTags = useStore((s) => s.selectedTags);
  const [loadingMore, setLoadingMore] = useState(false);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [page, setPage] = useState(0);
  const hasMore = files.length > 0 && files.length % 50 === 0;

  useEffect(() => {
    loadFiles();
  }, []);

  useEffect(() => {
    // 当选择的标签变化时，重新加载文件
    if (selectedTags.length > 0) {
      loadFilesByTags();
    } else {
      loadFiles();
    }
  }, [selectedTags]);

  const loadFiles = async (offset = 0, silent = false) => {
    if (!silent) setIsLoading(true);
    try {
      const data = await api.getFiles({ limit: 50, offset });
      if (offset === 0) {
        setFiles(data);
      } else {
        setFiles([...files, ...data]);
      }
    } catch (error) {
      console.error('加载文件失败:', error);
    } finally {
      if (!silent) setIsLoading(false);
    }
  };

  const loadFilesByTags = async () => {
    setIsLoading(true);
    try {
      const data = await api.getFilesByTags(selectedTags);
      setFiles(data);
    } catch (error) {
      console.error('按标签加载文件失败:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleRefresh = async () => {
    setIsRefreshing(true);
    setPage(0);
    await loadFiles(0, false);
    setIsRefreshing(false);
  };

  const handleLoadMore = () => {
    setLoadingMore(true);
    const newPage = page + 1;
    setPage(newPage);
    loadFiles(newPage * 50, true).finally(() => setLoadingMore(false));
  };

  const handleFileClick = async (file: FileType) => {
    try {
      // 使用 Tauri 的 shell API 打开文件所在目录
      const { open } = await import('@tauri-apps/plugin-opener') as any;
      open(file.path);
    } catch (error) {
      console.error('打开文件失败:', error);
    }
  };

  const displayFiles = (searchResults?.results && searchResults.results.length > 0)
    ? searchResults.results.map((r) => ({ ...r.file, tags: r.tags }))
    : files;

  const totalCount = searchResults?.total ?? files.length;

  return (
    <div className="flex flex-col h-full bg-background">
      <div className="p-4 border-b border-border flex items-center justify-between">
        <div>
          <h2 className="font-semibold text-lg">文件列表</h2>
          <p className="text-sm text-muted-foreground">
            共 {totalCount} 个文件
          </p>
        </div>
        <button
          onClick={handleRefresh}
          disabled={isRefreshing}
          className="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg disabled:opacity-50 transition-colors"
          title="刷新文件列表"
        >
          <RefreshCw size={20} className={isRefreshing ? 'animate-spin' : ''} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto">
        {isLoading && !displayFiles.length ? (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <RefreshCw size={32} className="mb-4 animate-spin" />
            <p>加载中...</p>
          </div>
        ) : displayFiles.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <File size={64} className="mb-4 opacity-30" />
            <p className="text-lg">暂无文件</p>
            <p className="text-sm mt-2">
              {selectedTags.length > 0
                ? '该标签下暂无文件，请尝试其他标签或添加监控目录'
                : '请先添加监控目录并扫描文件'}
            </p>
            {selectedTags.length === 0 && (
              <button
                onClick={handleRefresh}
                className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 flex items-center gap-2"
              >
                <Download size={16} />
                扫描文件
              </button>
            )}
          </div>
        ) : (
          <ul className="divide-y divide-border">
            {displayFiles.map((file) => (
              <FileItem
                key={file.id || file.path}
                file={file as any}
                onClick={() => handleFileClick(file as any)}
              />
            ))}
          </ul>
        )}
      </div>

      {hasMore && !searchResults && (
        <div className="p-4 border-t border-border">
          <button
            onClick={handleLoadMore}
            disabled={loadingMore}
            className="w-full py-2 px-4 bg-secondary text-secondary-foreground rounded-lg hover:bg-secondary/80 disabled:opacity-50 transition-colors"
          >
            {loadingMore ? '加载中...' : '加载更多'}
          </button>
        </div>
      )}
    </div>
  );
}

function FileItem({ file, onClick }: { file: FileType & { tags?: any[] }; onClick: () => void }) {
  const icon = getFileIcon(file.extension);

  return (
    <li
      onClick={onClick}
      className="flex items-center gap-4 p-4 hover:bg-muted/50 cursor-pointer transition-colors group"
    >
      <div className="text-2xl group-hover:scale-110 transition-transform">{icon}</div>
      <div className="flex-1 min-w-0">
        <div className="font-medium truncate">{file.name}</div>
        <div className="text-sm text-muted-foreground truncate">{file.path}</div>
        <div className="flex items-center gap-4 mt-1 text-xs text-muted-foreground">
          <span className="flex items-center gap-1">
            <HardDrive className="w-3 h-3" />
            {formatFileSize(file.size)}
          </span>
          <span className="flex items-center gap-1">
            <Calendar className="w-3 h-3" />
            {formatDate(file.createdAt)}
          </span>
        </div>
        {file.tags && file.tags.length > 0 && (
          <div className="flex flex-wrap gap-1 mt-2">
            {file.tags.slice(0, 3).map((tag) => (
              <span
                key={tag.id || tag.name}
                className="text-xs px-2 py-0.5 rounded-full text-white"
                style={{ backgroundColor: tag.color }}
              >
                {tag.displayName || tag.name}
              </span>
            ))}
            {file.tags.length > 3 && (
              <span className="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground">
                +{file.tags.length - 3}
              </span>
            )}
          </div>
        )}
      </div>
    </li>
  );
}
