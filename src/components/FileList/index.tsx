/**
 * 文件列表组件
 */
import { useEffect, useState, useRef } from 'react';
import { File, Calendar, HardDrive, RefreshCw, Download, MoreHorizontal, Tag, Trash2 } from 'lucide-react';
import { api } from '../../lib/api';
import { formatFileSize, formatDate, getFileIcon } from '../../lib/utils';
import { useStore } from '../../stores/useStore';
import type { File as FileType } from '../../types/api';
import AddTagModal from '../AddTagModal';

export default function FileList() {
  const files = useStore((s) => s.files);
  const setFiles = useStore((s) => s.setFiles);
  const searchResults = useStore((s) => s.searchResults);
  const searchKeywords = useStore((s) => s.searchKeywords);
  const isLoading = useStore((s) => s.isLoading);
  const setIsLoading = useStore((s) => s.setIsLoading);
  const selectedTags = useStore((s) => s.selectedTags);
  const addNotification = useStore((s) => s.addNotification);
  const filesRevision = useStore((s) => s.filesRevision);

  const [loadingMore, setLoadingMore] = useState(false);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [page, setPage] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const dragCounter = useRef(0);

  // 多选文件状态
  const [selectedFileIds, setSelectedFileIds] = useState<Set<number | string>>(new Set());
  const [showAddTagModal, setShowAddTagModal] = useState(false);

  // 右键菜单状态
  const [contextMenu, setContextMenu] = useState<{
    visible: boolean;
    x: number;
    y: number;
    file: FileType | null;
  }>({
    visible: false,
    x: 0,
    y: 0,
    file: null,
  });

  const hasMore = files.length > 0 && files.length % 50 === 0;

  useEffect(() => {
    loadFiles();
  }, []);

  useEffect(() => {
    // 当选择的标签变化时，重新加载文件
    // 如果有搜索关键字，搜索会在 App.tsx 中自动重新执行，不需要这里处理
    if (searchKeywords.length === 0) {
      if (selectedTags.length > 0) {
        loadFilesByTags();
      } else {
        loadFiles();
      }
    }
  }, [selectedTags, searchKeywords]);

  // 当 filesRevision 变化时刷新文件列表
  useEffect(() => {
    if (filesRevision > 0 && searchKeywords.length === 0) {
      if (selectedTags.length > 0) {
        loadFilesByTags();
      } else {
        loadFiles();
      }
    }
  }, [filesRevision, searchKeywords]);

  // 点击外部关闭右键菜单
  useEffect(() => {
    const handleClickOutside = () => {
      if (contextMenu.visible) {
        setContextMenu({ ...contextMenu, visible: false });
      }
    };
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  }, [contextMenu.visible]);

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

  // 拖放处理
  const handleDragEnter = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    dragCounter.current++;
    if (dragCounter.current === 1) {
      setIsDragging(true);
    }
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    dragCounter.current--;
    if (dragCounter.current === 0) {
      setIsDragging(false);
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
    dragCounter.current = 0;

    const items = e.dataTransfer?.items;
    if (!items) return;

    setIsUploading(true);
    let successCount = 0;
    let errorCount = 0;

    try {
      // 处理拖放的文件
      const files: File[] = [];
      for (let i = 0; i < items.length; i++) {
        const item = items[i];
        if (item.kind === 'file') {
          const file = item.getAsFile();
          if (file) {
            files.push(file);
          }
        }
      }

      if (files.length === 0) {
        addNotification({ type: 'warning', message: '没有检测到有效的文件' });
        setIsUploading(false);
        return;
      }

      // 处理每个文件
      for (const file of files) {
        try {
          // 暂时使用文件名作为路径（实际应用中需要获取完整路径）
          const filePath = file.name;
          // 调用 API 添加文件
          await api.addFile(filePath);
          successCount++;
        } catch (error) {
          console.error(`添加文件 ${file.name} 失败:`, error);
          errorCount++;
        }
      }

      // 刷新文件列表
      await loadFiles();

      // 显示结果通知
      if (successCount > 0) {
        addNotification({
          type: 'success',
          message: `成功添加 ${successCount} 个文件${errorCount > 0 ? `，${errorCount} 个失败` : ''}`,
        });
      } else {
        addNotification({
          type: 'error',
          message: `添加文件失败，共 ${errorCount} 个文件`,
        });
      }
    } catch (error) {
      console.error('处理拖放文件失败:', error);
      addNotification({ type: 'error', message: '处理文件失败' });
    } finally {
      setIsUploading(false);
    }
  };

  const displayFiles = (searchResults?.results && searchResults.results.length > 0)
    ? searchResults.results.map((r) => ({ ...r.file, tags: r.tags }))
    : files;

  const totalCount = searchResults?.total ?? files.length;

  // 右键菜单处理
  const handleContextMenu = (e: React.MouseEvent, file: FileType) => {
    e.preventDefault();
    e.stopPropagation();

    // 获取鼠标点击位置
    setContextMenu({
      visible: true,
      x: e.clientX,
      y: e.clientY,
      file: file,
    });
  };

  // 处理文件选择
  const handleSelectFile = (fileId: number | string, e?: React.MouseEvent) => {
    e?.stopPropagation();
    setSelectedFileIds(prev => {
      const newSet = new Set(prev);
      if (newSet.has(fileId)) {
        newSet.delete(fileId);
      } else {
        newSet.add(fileId);
      }
      return newSet;
    });
  };

  // 清空多选
  const clearSelection = () => {
    setSelectedFileIds(new Set());
  };

  const handleContextMenuItem = async (action: string, file: FileType) => {
    setContextMenu({ ...contextMenu, visible: false });

    switch (action) {
      case 'open':
        await handleFileClick(file);
        break;
      case 'delete':
        try {
          if (file.id) {
            await api.deleteFile(file.id);
            addNotification({ type: 'success', message: `已删除 ${file.name}` });
            // 刷新文件列表
            await loadFiles();
          }
        } catch (error) {
          console.error('删除文件失败:', error);
          addNotification({ type: 'error', message: '删除文件失败' });
        }
        break;
      case 'addTag':
        // 显示添加标签对话框
        setShowAddTagModal(true);
        break;
      default:
        console.log('其他功能待实现:', action);
    }
  };

  // 点击外部关闭右键菜单
  useEffect(() => {
    const handleClickOutside = () => {
      if (contextMenu.visible) {
        setContextMenu({ ...contextMenu, visible: false });
      }
    };
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  }, [contextMenu.visible]);

  console.log('[FileList] Render state:', {
    filesLength: files.length,
    files,
    searchResults,
    displayFilesLength: displayFiles.length,
    displayFiles
  });

  return (
    <div
      className="flex flex-col h-full bg-background relative"
      onDragEnter={handleDragEnter}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    >
      {/* 拖放遮罩 */}
      {isDragging && (
        <div className="absolute inset-0 bg-blue-500/10 border-4 border-dashed border-blue-500 rounded-lg z-10 flex items-center justify-center">
          <div className="text-center">
            <Download size={64} className="mx-auto mb-4 text-blue-500" />
            <p className="text-xl font-semibold text-blue-600">拖放文件到此处</p>
            <p className="text-sm text-gray-600 mt-2">释放鼠标添加文件到索引</p>
          </div>
        </div>
      )}

      {/* 上传中提示 */}
      {isUploading && (
        <div className="absolute inset-0 bg-black/50 flex items-center justify-center z-20">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 text-center">
            <RefreshCw size={48} className="mx-auto mb-4 text-blue-600 animate-spin" />
            <p className="text-lg font-semibold">正在处理文件...</p>
          </div>
        </div>
      )}

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
                ? '该标签下暂无文件，请尝试其他标签或添加监控目录并扫描文件'
                : '拖放文件到此处，或添加监控目录并扫描文件'}
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
                onContextMenu={(e) => handleContextMenu(e, file as any)}
                isSelected={file.id ? selectedFileIds.has(file.id) : false}
                onSelect={(e) => file.id && handleSelectFile(file.id, e)}
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

      {/* 右键菜单 */}
      {contextMenu.visible && contextMenu.file && (
        <div
          className="fixed z-50 min-w-48 bg-card border border-border rounded-lg shadow-lg py-1"
          style={{
            left: `${contextMenu.x}px`,
            top: `${contextMenu.y}px`,
          }}
          onContextMenu={(e) => e.preventDefault()}
        >
          <div
            className="px-3 py-2 hover:bg-muted cursor-pointer transition-colors"
            onClick={() => contextMenu.file ? handleContextMenuItem('open', contextMenu.file) : undefined}
          >
            <div className="flex items-center gap-2">
              <Download size={16} />
              <span className="text-sm">打开文件所在目录</span>
            </div>
          </div>
          <div
            className="px-3 py-2 hover:bg-muted cursor-pointer transition-colors"
            onClick={() => contextMenu.file ? handleContextMenuItem('addTag', contextMenu.file) : undefined}
          >
            <div className="flex items-center gap-2">
              <Tag size={16} />
              <span className="text-sm">添加标签{selectedFileIds.size > 1 ? ` (${selectedFileIds.size})` : ''}</span>
            </div>
          </div>
          <div
            className="px-3 py-2 hover:bg-muted cursor-pointer transition-colors"
            onClick={() => contextMenu.file ? handleContextMenuItem('delete', contextMenu.file) : undefined}
          >
            <div className="flex items-center gap-2">
              <Trash2 size={16} />
              <span className="text-sm">删除文件</span>
            </div>
          </div>
          <div className="flex items-center gap-1 px-3 py-2 text-xs text-muted-foreground">
            <span>更多功能开发中...</span>
            <MoreHorizontal size={16} />
          </div>
        </div>
      )}

      {/* 添加标签对话框 */}
      <AddTagModal
        isOpen={showAddTagModal}
        onClose={() => {
          setShowAddTagModal(false);
          clearSelection();
        }}
        onSuccess={async () => {
          // 成功添加标签后刷新文件列表
          if (selectedTags.length > 0) {
            await loadFilesByTags();
          } else {
            await loadFiles();
          }
        }}
        selectedFiles={selectedFileIds.size > 0
          ? displayFiles.filter(f => f.id && selectedFileIds.has(f.id))
          : (contextMenu.file ? [contextMenu.file] : [])}
      />
    </div>
  );
}

function FileItem({ file, onClick, onContextMenu, isSelected, onSelect }: {
  file: FileType & { tags?: any[] };
  onClick: () => void;
  onContextMenu: (e: React.MouseEvent) => void;
  isSelected: boolean;
  onSelect: (e: React.MouseEvent) => void;
}) {
  const icon = getFileIcon(file.extension);

  return (
    <li
      onClick={onClick}
      onContextMenu={onContextMenu}
      className={`flex items-center gap-4 p-4 hover:bg-muted/50 cursor-pointer transition-colors group ${isSelected ? 'bg-muted' : ''}`}
      role="listitem"
    >
      <div
        onClick={onSelect}
        className={`w-5 h-5 rounded border-2 flex items-center justify-center transition-colors ${
          isSelected
            ? 'bg-primary border-primary'
            : 'border-border hover:border-primary'
        }`}
      >
        {isSelected && (
          <svg className="w-3 h-3 text-primary-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3">
            <path d="M20 6L9 17l-5-5" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
        )}
      </div>
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
            {file.tags.slice(0, 3).map((tag: any) => (
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
