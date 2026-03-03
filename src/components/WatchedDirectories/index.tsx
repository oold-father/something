/**
 * 监控目录管理组件
 */
import { useState, useEffect } from 'react';
import { FolderOpen, FolderPlus, Trash2, RefreshCw, AlertCircle, Play, CheckCircle2 } from 'lucide-react';
import { useStore } from '../../stores/useStore';
import { api } from '../../lib/api';
import type { BatchScanResult } from '../../types/api';

export default function WatchedDirectories() {
  const { watchedDirectories, setWatchedDirectories, setFiles } = useStore();
  const [isScanning, setIsScanning] = useState<string | null>(null);
  const [isBatchScanning, setIsBatchScanning] = useState(false);
  const [newPath, setNewPath] = useState('');
  const [isRecursive, setIsRecursive] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showDialog, setShowDialog] = useState(false);
  const [scanResult, setScanResult] = useState<BatchScanResult | null>(null);
  const [showResultDialog, setShowResultDialog] = useState(false);
  const [isAdding, setIsAdding] = useState(false);

  useEffect(() => {
    loadWatchedDirectories();
  }, []);

  const loadWatchedDirectories = async () => {
    try {
      const dirs = await api.getWatchedDirectories();
      setWatchedDirectories(dirs);
    } catch (err) {
      console.error('加载监控目录失败:', err);
      setError('加载监控目录失败');
    }
  };

  const loadFiles = async () => {
    try {
      const data = await api.getFiles({ limit: 100, offset: 0 });
      setFiles(data);
    } catch (err) {
      console.error('加载文件列表失败:', err);
    }
  };

  const handleAddDirectory = async () => {
    if (!newPath.trim()) {
      setError('请输入目录路径');
      return;
    }

    setIsAdding(true);
    try {
      await api.addWatchedDirectory({
        path: newPath,
        recursive: isRecursive,
      });
      await loadWatchedDirectories();
      setShowDialog(false);
      setNewPath('');
      setError(null);
    } catch (err) {
      console.error('添加目录失败:', err);
      setError('添加目录失败，请检查路径是否有效');
    } finally {
      setIsAdding(false);
    }
  };

  const handleRemoveDirectory = async (id: number, path: string) => {
    if (!confirm(`确定要移除监控目录 "${path}" 吗？`)) {
      return;
    }

    try {
      await api.removeWatchedDirectory(id);
      await loadWatchedDirectories();
    } catch (err) {
      console.error('移除目录失败:', err);
      setError('移除目录失败');
    }
  };

  const handleToggleEnabled = async (id: number, enabled: boolean) => {
    try {
      await api.updateWatchedDirectory(id, !enabled);
      await loadWatchedDirectories();
    } catch (err) {
      console.error('更新目录状态失败:', err);
      setError('更新目录状态失败');
    }
  };

  const handleScanDirectory = async (_id: number, path: string) => {
    setIsScanning(path);
    try {
      const result = await api.scanDirectory(path, true);
      console.log('扫描结果:', result);
      await loadWatchedDirectories();
      await loadFiles(); // 刷新文件列表

      // 显示扫描结果
      if (result.scannedFiles > 0) {
        setError(null);
      }
    } catch (err) {
      console.error('扫描目录失败:', err);
      setError('扫描目录失败');
    } finally {
      setIsScanning(null);
    }
  };

  const handleBatchScan = async () => {
    const enabledCount = watchedDirectories.filter(d => d.enabled).length;
    if (enabledCount === 0) {
      setError('请先添加并启用监控目录');
      return;
    }

    setIsBatchScanning(true);
    setError(null);

    try {
      const result = await api.scanAllDirectories();
      console.log('批量扫描结果:', result);
      setScanResult(result);
      setShowResultDialog(true);

      await loadWatchedDirectories();
      await loadFiles(); // 刷新文件列表
    } catch (err) {
      console.error('批量扫描失败:', err);
      setError('批量扫描失败');
    } finally {
      setIsBatchScanning(false);
    }
  };

  const handleSelectFolder = async () => {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
      });
      if (selected && typeof selected === 'string') {
        setNewPath(selected);
      }
    } catch (err) {
      console.error('选择文件夹失败:', err);
    }
  };

  const enabledDirs = watchedDirectories.filter(d => d.enabled);

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold flex items-center gap-2">
          <FolderOpen size={20} />
          监控目录
          {enabledDirs.length > 0 && (
            <span className="text-sm font-normal text-gray-500">
              ({enabledDirs.length} 个启用)
            </span>
          )}
        </h2>
        <div className="flex gap-2">
          {enabledDirs.length > 0 && (
            <button
              onClick={handleBatchScan}
              disabled={isBatchScanning}
              className="px-3 py-1.5 text-sm bg-green-600 text-white rounded hover:bg-green-700 flex items-center gap-2 disabled:opacity-50"
            >
              <Play size={16} />
              {isBatchScanning ? '扫描中...' : '全部扫描'}
            </button>
          )}
          <button
            onClick={() => setShowDialog(true)}
            className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 flex items-center gap-2"
          >
            <FolderPlus size={16} />
            添加目录
          </button>
        </div>
      </div>

      {error && (
        <div className="p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg flex items-center gap-2">
          <AlertCircle size={16} />
          {error}
        </div>
      )}

      {watchedDirectories.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <FolderOpen size={48} className="mx-auto mb-2 opacity-50" />
          <p className="text-lg">暂无监控目录</p>
          <p className="text-sm">点击"添加目录"按钮开始监控文件</p>
        </div>
      ) : (
        <div className="space-y-2">
          {watchedDirectories.map((dir) => (
            <div
              key={dir.id}
              className={`p-3 rounded-lg border transition-colors ${
                dir.enabled
                  ? 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700'
                  : 'bg-gray-50 dark:bg-gray-800/50 border-gray-200 dark:border-gray-700 opacity-60'
              }`}
            >
              <div className="flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <FolderOpen size={18} className="flex-shrink-0 text-blue-600" />
                    <span className="font-medium truncate">{dir.path}</span>
                  </div>
                  <div className="flex items-center gap-4 mt-1 text-xs text-gray-500 dark:text-gray-400">
                    <span>{dir.recursive ? '递归监控' : '单层监控'}</span>
                    {dir.lastScannedAt && (
                      <span>扫描于: {new Date(dir.lastScannedAt).toLocaleString('zh-CN')}</span>
                    )}
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => handleToggleEnabled(dir.id!, dir.enabled)}
                    className={`px-2 py-1 text-xs rounded ${
                      dir.enabled
                        ? 'bg-green-100 text-green-700 hover:bg-green-200'
                        : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
                    }`}
                  >
                    {dir.enabled ? '已启用' : '已禁用'}
                  </button>

                  {dir.enabled && (
                    <button
                      onClick={() => handleScanDirectory(dir.id!, dir.path)}
                      disabled={isScanning === dir.path}
                      className="p-1.5 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded disabled:opacity-50"
                      title="扫描目录"
                    >
                      <RefreshCw
                        size={16}
                        className={isScanning === dir.path ? 'animate-spin' : ''}
                      />
                    </button>
                  )}

                  <button
                    onClick={() => handleRemoveDirectory(dir.id!, dir.path)}
                    className="p-1.5 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded"
                    title="移除目录"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* 添加目录对话框 */}
      {showDialog && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md p-6">
            <h3 className="text-lg font-semibold mb-4">添加监控目录</h3>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  目录路径
                </label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={newPath}
                    onChange={(e) => setNewPath(e.target.value)}
                    placeholder="例如: C:\Documents 或 /home/user/Documents"
                    className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                  <button
                    type="button"
                    onClick={handleSelectFolder}
                    className="px-3 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600"
                  >
                    浏览
                  </button>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <input
                  type="checkbox"
                  id="recursive"
                  checked={isRecursive}
                  onChange={(e) => setIsRecursive(e.target.checked)}
                  className="w-4 h-4 text-blue-600 rounded border-gray-300 focus:ring-blue-500"
                />
                <label htmlFor="recursive" className="text-sm text-gray-700 dark:text-gray-300">
                  递归监控子目录
                </label>
              </div>

              <div className="flex justify-end gap-3 pt-2">
                <button
                  onClick={() => {
                    setShowDialog(false);
                    setNewPath('');
                    setError(null);
                  }}
                  className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
                >
                  取消
                </button>
                <button
                  onClick={handleAddDirectory}
                  disabled={isAdding}
                  className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
                >
                  {isAdding ? '添加中...' : '添加'}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 扫描结果对话框 */}
      {showResultDialog && scanResult && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-lg p-6">
            <div className="flex items-center gap-2 mb-4">
              {scanResult.scannedDirectories > 0 && scanResult.errors.length === 0 ? (
                <CheckCircle2 size={24} className="text-green-600" />
              ) : (
                <AlertCircle size={24} className="text-orange-600" />
              )}
              <h3 className="text-lg font-semibold">扫描完成</h3>
            </div>

            <div className="space-y-3 mb-4">
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div className="flex flex-col">
                  <span className="text-gray-500 dark:text-gray-400">扫描目录</span>
                  <span className="font-medium">{scanResult.scannedDirectories}/{scanResult.totalDirectories}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500 dark:text-gray-400">扫描文件</span>
                  <span className="font-medium">{scanResult.totalFiles}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500 dark:text-gray-400">新增文件</span>
                  <span className="font-medium text-green-600">+{scanResult.addedFiles}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500 dark:text-gray-400">更新文件</span>
                  <span className="font-medium text-blue-600">{scanResult.updatedFiles}</span>
                </div>
              </div>

              {scanResult.errors.length > 0 && (
                <div className="mt-4">
                  <h4 className="text-sm font-medium text-red-600 mb-2">错误 ({scanResult.errors.length})</h4>
                  <div className="max-h-40 overflow-y-auto space-y-1">
                    {scanResult.errors.map((err, idx) => (
                      <div key={idx} className="text-xs bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 p-2 rounded">
                        <div className="font-medium truncate">{err.path}</div>
                        <div className="opacity-75">{err.message}</div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>

            <div className="flex justify-end">
              <button
                onClick={() => {
                  setShowResultDialog(false);
                  setScanResult(null);
                }}
                className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
              >
                确定
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
